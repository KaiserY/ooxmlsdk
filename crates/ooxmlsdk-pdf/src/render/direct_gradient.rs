use pdf_writer::types::{FunctionShadingType, MaskType};
use pdf_writer::{Content, Finish, Name, Pdf, Rect, Ref};

use super::direct::RefAllocator;
use super::direct_path_gradient::{
  PathGradientProfile, PathGradientRasterSampling, PathGradientResolution, rasterize,
  resolve as resolve_path_gradient, validate_raster,
};
use super::gradient::{linear_line, stops_for_pdf};
use super::image::PreparedRasterImage;
use crate::error::{PdfError, Result};
use ooxmlsdk_layout::common;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct ColorStop {
  pub(super) position: f32,
  pub(super) rgb: [f32; 3],
  pub(super) alpha: u8,
}

#[derive(Clone, Debug, PartialEq)]
struct FunctionGradientSpec {
  geometry: FunctionGradientGeometry,
  stops: Vec<ColorStop>,
}

#[derive(Clone, Debug, PartialEq)]
enum FunctionGradientGeometry {
  Axial([f32; 4]),
  Radial([f32; 6]),
}

impl FunctionGradientGeometry {
  fn shading_type(&self) -> FunctionShadingType {
    match self {
      Self::Axial(_) => FunctionShadingType::Axial,
      Self::Radial(_) => FunctionShadingType::Radial,
    }
  }

  fn is_radial(&self) -> bool {
    matches!(self, Self::Radial(_))
  }
}

#[derive(Clone, Debug)]
struct ExponentialFunction<const N: usize> {
  id: Ref,
  start: [f32; N],
  end: [f32; N],
}

#[derive(Clone, Debug)]
enum GradientFunction<const N: usize> {
  Exponential(ExponentialFunction<N>),
  Stitching {
    id: Ref,
    segments: Vec<ExponentialFunction<N>>,
    bounds: Vec<f32>,
  },
}

impl<const N: usize> GradientFunction<N> {
  fn root_id(&self) -> Ref {
    match self {
      Self::Exponential(function) => function.id,
      Self::Stitching { id, .. } => *id,
    }
  }
}

#[derive(Clone, Debug)]
struct GradientResource {
  name: Vec<u8>,
  shading_id: Ref,
  spec: FunctionGradientSpec,
  function: GradientFunction<3>,
  opacity: GradientOpacity,
  soft_mask: Option<GradientSoftMaskResource>,
  exposed_as_shading: bool,
  pattern: Option<GradientPatternResource>,
}

#[derive(Clone, Debug)]
struct GradientPatternResource {
  name: Vec<u8>,
  id: Ref,
  matrix: [f32; 6],
}

#[derive(Clone, Debug)]
struct GradientSoftMaskResource {
  name: Vec<u8>,
  shading_name: Vec<u8>,
  shading_id: Ref,
  function: GradientFunction<1>,
  form_id: Ref,
  ext_graphics_id: Ref,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum GradientOpacity {
  Opaque,
  Constant(u8),
  SoftMask { name: Vec<u8> },
}

#[derive(Clone, Debug)]
pub(super) enum RegisteredGradient {
  Solid(common::Color),
  Shading {
    name: Vec<u8>,
    opacity: GradientOpacity,
  },
  Pattern {
    name: Vec<u8>,
    opacity: GradientOpacity,
  },
  Raster(PreparedRasterImage),
}

#[derive(Clone, Debug)]
pub(super) enum RegisteredGradientPattern {
  Solid(common::Color),
  Pattern {
    name: Vec<u8>,
    opacity: GradientOpacity,
  },
  Raster(PreparedRasterImage),
}

#[derive(Debug)]
pub(super) struct DirectGradientSet {
  resources: Vec<GradientResource>,
  mask_bbox: [f32; 4],
  path_profile: PathGradientProfile,
}

impl DirectGradientSet {
  pub(super) fn new(
    page_width_pt: f32,
    page_height_pt: f32,
    path_profile: PathGradientProfile,
  ) -> Self {
    Self {
      resources: Vec::new(),
      mask_bbox: [0.0, 0.0, page_width_pt, page_height_pt],
      path_profile,
    }
  }

  pub(super) fn validate(
    gradient: &common::GradientFill<'static>,
    bounds: common::Rect,
  ) -> Result<()> {
    let profile = PathGradientProfile {
      office_fixed_output: false,
      raster_sampling: PathGradientRasterSampling::BoundedPixelsPerPoint(2.0),
    };
    if let ResolvedGradient::Raster { path, stops } = resolve(gradient, bounds, profile)? {
      validate_raster(path, &stops, bounds, profile)?;
    }
    Ok(())
  }

  pub(super) fn register(
    &mut self,
    gradient: &common::GradientFill<'static>,
    bounds: common::Rect,
    commands: Option<&[common::PathCommand]>,
    refs: &mut RefAllocator,
  ) -> Result<RegisteredGradient> {
    let spec = match resolve(gradient, bounds, self.path_profile)? {
      ResolvedGradient::Solid(color) => return Ok(RegisteredGradient::Solid(color)),
      ResolvedGradient::Function(spec) => spec,
      ResolvedGradient::Raster { path, stops } => {
        return Ok(RegisteredGradient::Raster(rasterize(
          path,
          &stops,
          bounds,
          commands,
          self.path_profile,
        )?));
      }
    };

    let index = self.register_resource(spec, refs)?;
    if self.resources[index].spec.geometry.is_radial() {
      self.ensure_pattern(index, refs)?;
      let resource = &self.resources[index];
      let pattern = resource
        .pattern
        .as_ref()
        .expect("radial gradient pattern was registered");
      Ok(RegisteredGradient::Pattern {
        name: pattern.name.clone(),
        opacity: resource.opacity.clone(),
      })
    } else {
      let resource = &mut self.resources[index];
      resource.exposed_as_shading = true;
      Ok(RegisteredGradient::Shading {
        name: resource.name.clone(),
        opacity: resource.opacity.clone(),
      })
    }
  }

  pub(super) fn register_pattern(
    &mut self,
    gradient: &common::GradientFill<'static>,
    definition_bounds: common::Rect,
    raster_bounds: common::Rect,
    commands: Option<&[common::PathCommand]>,
    refs: &mut RefAllocator,
  ) -> Result<RegisteredGradientPattern> {
    let spec = match resolve(gradient, definition_bounds, self.path_profile)? {
      ResolvedGradient::Solid(color) => return Ok(RegisteredGradientPattern::Solid(color)),
      ResolvedGradient::Function(spec) => spec,
      ResolvedGradient::Raster { path, stops } => {
        return Ok(RegisteredGradientPattern::Raster(rasterize(
          path,
          &stops,
          raster_bounds,
          commands,
          self.path_profile,
        )?));
      }
    };
    let index = self.register_resource(spec, refs)?;
    self.ensure_pattern(index, refs)?;
    let resource = &self.resources[index];
    let pattern = resource
      .pattern
      .as_ref()
      .expect("gradient pattern was registered");
    Ok(RegisteredGradientPattern::Pattern {
      name: pattern.name.clone(),
      opacity: resource.opacity.clone(),
    })
  }

  fn register_resource(
    &mut self,
    spec: FunctionGradientSpec,
    refs: &mut RefAllocator,
  ) -> Result<usize> {
    if let Some(index) = self
      .resources
      .iter()
      .position(|resource| resource.spec == spec)
    {
      return Ok(index);
    }
    let index = self.resources.len();
    let shading_id = refs.alloc()?;
    let function = allocate_function(&spec.stops, refs, |stop| stop.rgb)?;
    let (opacity, soft_mask) = allocate_opacity(&spec, index, refs)?;
    self.resources.push(GradientResource {
      name: format!("Sh{index}").into_bytes(),
      shading_id,
      spec,
      function,
      opacity,
      soft_mask,
      exposed_as_shading: false,
      pattern: None,
    });
    Ok(index)
  }

  fn ensure_pattern(&mut self, index: usize, refs: &mut RefAllocator) -> Result<()> {
    if self.resources[index].pattern.is_some() {
      return Ok(());
    }
    let pattern_index = self
      .resources
      .iter()
      .filter(|resource| resource.pattern.is_some())
      .count();
    self.resources[index].pattern = Some(GradientPatternResource {
      name: format!("P{pattern_index}").into_bytes(),
      id: refs.alloc()?,
      // Shading dictionaries use layout's y-down coordinates. A shading
      // pattern, unlike the `sh` operator in the reflected page stream, maps
      // its pattern space to PDF's default y-up user space.
      matrix: [1.0, 0.0, 0.0, -1.0, 0.0, self.mask_bbox[3]],
    });
    Ok(())
  }

  pub(super) fn has_shadings(&self) -> bool {
    self
      .resources
      .iter()
      .any(|resource| resource.exposed_as_shading)
  }

  pub(super) fn has_patterns(&self) -> bool {
    self
      .resources
      .iter()
      .any(|resource| resource.pattern.is_some())
  }

  pub(super) fn has_soft_masks(&self) -> bool {
    self
      .resources
      .iter()
      .any(|resource| resource.soft_mask.is_some())
  }

  pub(super) fn dictionary_entries(&self) -> impl Iterator<Item = (Name<'_>, Ref)> + '_ {
    self
      .resources
      .iter()
      .filter(|resource| resource.exposed_as_shading)
      .map(|resource| (Name(resource.name.as_slice()), resource.shading_id))
  }

  pub(super) fn pattern_dictionary_entries(&self) -> impl Iterator<Item = (Name<'_>, Ref)> + '_ {
    self.resources.iter().filter_map(|resource| {
      resource
        .pattern
        .as_ref()
        .map(|pattern| (Name(pattern.name.as_slice()), pattern.id))
    })
  }

  pub(super) fn soft_mask_dictionary_entries(&self) -> impl Iterator<Item = (Name<'_>, Ref)> + '_ {
    self.resources.iter().filter_map(|resource| {
      resource
        .soft_mask
        .as_ref()
        .map(|mask| (Name(mask.name.as_slice()), mask.ext_graphics_id))
    })
  }

  pub(super) fn write_objects(&self, pdf: &mut Pdf) {
    for resource in &self.resources {
      write_function(pdf, &resource.function);
      write_function_shading(
        pdf,
        resource.shading_id,
        &resource.spec,
        resource.function.root_id(),
        false,
      );
      if let Some(pattern) = &resource.pattern {
        let mut writer = pdf.shading_pattern(pattern.id);
        writer
          .shading_ref(resource.shading_id)
          .matrix(pattern.matrix);
        writer.finish();
      }
      if let Some(mask) = &resource.soft_mask {
        write_soft_mask_objects(pdf, &resource.spec, mask, self.mask_bbox);
      }
    }
  }
}

#[derive(Clone, Debug)]
enum ResolvedGradient {
  Solid(common::Color),
  Function(FunctionGradientSpec),
  Raster {
    path: common::GradientPath,
    stops: Vec<ColorStop>,
  },
}

fn resolve(
  gradient: &common::GradientFill<'static>,
  bounds: common::Rect,
  path_profile: PathGradientProfile,
) -> Result<ResolvedGradient> {
  let mut source_stops = stops_for_pdf(gradient);
  if source_stops.is_empty() {
    return Err(PdfError::Writer(
      "gradient fill must contain at least one color stop".to_string(),
    ));
  }
  if source_stops.len() == 1 {
    let stop = source_stops.remove(0);
    if !stop.position.is_finite() || !(0.0..=1.0).contains(&stop.position) {
      return Err(PdfError::Writer(format!(
        "gradient stop position {} is outside 0..=1",
        stop.position
      )));
    }
    return Ok(ResolvedGradient::Solid(stop.color));
  }

  let mut stops = Vec::<ColorStop>::with_capacity(source_stops.len() + 2);
  for stop in source_stops {
    if !stop.position.is_finite() || !(0.0..=1.0).contains(&stop.position) {
      return Err(PdfError::Writer(format!(
        "gradient stop position {} is outside 0..=1",
        stop.position
      )));
    }
    let rgb = normalized_rgb(stop.color);
    if let Some(previous) = stops.last() {
      if stop.position < previous.position {
        return Err(PdfError::Writer(
          "gradient stop positions are not monotonically increasing".to_string(),
        ));
      }
      if stop.position == previous.position {
        if rgb == previous.rgb && stop.color.a == previous.alpha {
          continue;
        }
        return Err(PdfError::DirectWriterUnsupported {
          feature: "coincident hard-edge gradient stops",
        });
      }
    }
    stops.push(ColorStop {
      position: stop.position,
      rgb,
      alpha: stop.color.a,
    });
  }

  if stops[0].position > 0.0 {
    let mut first = stops[0].clone();
    first.position = 0.0;
    stops.insert(0, first);
  }
  if stops.last().is_some_and(|stop| stop.position < 1.0) {
    let mut last = stops.last().expect("gradient has stops").clone();
    last.position = 1.0;
    stops.push(last);
  }

  if let Some(path) = gradient.path {
    return match resolve_path_gradient(path, path_profile)? {
      PathGradientResolution::Radial(coords) => {
        Ok(ResolvedGradient::Function(FunctionGradientSpec {
          geometry: FunctionGradientGeometry::Radial(coords),
          stops,
        }))
      }
      PathGradientResolution::Raster => Ok(ResolvedGradient::Raster { path, stops }),
    };
  }

  let (start, end) = gradient.line.unwrap_or_else(|| {
    linear_line(
      gradient.definition_bounds.unwrap_or(bounds),
      gradient.angle_degrees,
      gradient.scaled,
    )
  });
  let coords = [start.x.0, start.y.0, end.x.0, end.y.0];
  if !coords.into_iter().all(f32::is_finite) {
    return Err(PdfError::Writer(
      "linear gradient has non-finite coordinates".to_string(),
    ));
  }
  if start == end {
    return Err(PdfError::Writer(
      "linear gradient endpoints must be distinct".to_string(),
    ));
  }

  Ok(ResolvedGradient::Function(FunctionGradientSpec {
    geometry: FunctionGradientGeometry::Axial(coords),
    stops,
  }))
}

fn normalized_rgb(color: common::Color) -> [f32; 3] {
  let maximum = f32::from(u8::MAX);
  [
    f32::from(color.r) / maximum,
    f32::from(color.g) / maximum,
    f32::from(color.b) / maximum,
  ]
}

fn allocate_function<const N: usize>(
  stops: &[ColorStop],
  refs: &mut RefAllocator,
  components: impl Fn(&ColorStop) -> [f32; N],
) -> Result<GradientFunction<N>> {
  if let [start, end] = stops {
    return Ok(GradientFunction::Exponential(ExponentialFunction {
      id: refs.alloc()?,
      start: components(start),
      end: components(end),
    }));
  }

  let id = refs.alloc()?;
  let mut segments = Vec::with_capacity(stops.len() - 1);
  for pair in stops.windows(2) {
    segments.push(ExponentialFunction {
      id: refs.alloc()?,
      start: components(&pair[0]),
      end: components(&pair[1]),
    });
  }
  let bounds = stops[1..stops.len() - 1]
    .iter()
    .map(|stop| stop.position)
    .collect();
  Ok(GradientFunction::Stitching {
    id,
    segments,
    bounds,
  })
}

fn write_function<const N: usize>(pdf: &mut Pdf, function: &GradientFunction<N>) {
  match function {
    GradientFunction::Exponential(function) => write_exponential(pdf, function),
    GradientFunction::Stitching {
      id,
      segments,
      bounds,
    } => {
      for segment in segments {
        write_exponential(pdf, segment);
      }
      let mut stitching = pdf.stitching_function(*id);
      stitching
        .domain([0.0, 1.0])
        .range((0..N).flat_map(|_| [0.0, 1.0]))
        .functions(segments.iter().map(|segment| segment.id))
        .bounds(bounds.iter().copied())
        .encode((0..segments.len()).flat_map(|_| [0.0, 1.0]));
      stitching.finish();
    }
  }
}

fn write_exponential<const N: usize>(pdf: &mut Pdf, function: &ExponentialFunction<N>) {
  let mut exponential = pdf.exponential_function(function.id);
  exponential
    .domain([0.0, 1.0])
    .range((0..N).flat_map(|_| [0.0, 1.0]))
    .c0(function.start)
    .c1(function.end)
    .n(1.0);
  exponential.finish();
}

fn write_function_shading(
  pdf: &mut Pdf,
  id: Ref,
  spec: &FunctionGradientSpec,
  function: Ref,
  gray: bool,
) {
  let mut shading = pdf.function_shading(id);
  shading
    .shading_type(spec.geometry.shading_type())
    .anti_alias(true)
    .function(function)
    .extend([true, true]);
  match spec.geometry {
    FunctionGradientGeometry::Axial(coords) => {
      shading.coords(coords);
    }
    FunctionGradientGeometry::Radial(coords) => {
      shading.coords(coords);
    }
  }
  if gray {
    shading.color_space().device_gray();
  } else {
    shading.color_space().device_rgb();
  }
  shading.finish();
}

fn allocate_opacity(
  spec: &FunctionGradientSpec,
  index: usize,
  refs: &mut RefAllocator,
) -> Result<(GradientOpacity, Option<GradientSoftMaskResource>)> {
  let alpha = spec.stops[0].alpha;
  if spec.stops.iter().all(|stop| stop.alpha == alpha) {
    let opacity = if alpha == u8::MAX {
      GradientOpacity::Opaque
    } else {
      GradientOpacity::Constant(alpha)
    };
    return Ok((opacity, None));
  }

  let name = format!("GSM{index}").into_bytes();
  let mask = GradientSoftMaskResource {
    name: name.clone(),
    shading_name: format!("SMSh{index}").into_bytes(),
    shading_id: refs.alloc()?,
    function: allocate_function(&spec.stops, refs, |stop| {
      [f32::from(stop.alpha) / f32::from(u8::MAX)]
    })?,
    form_id: refs.alloc()?,
    ext_graphics_id: refs.alloc()?,
  };
  Ok((GradientOpacity::SoftMask { name }, Some(mask)))
}

fn write_soft_mask_objects(
  pdf: &mut Pdf,
  spec: &FunctionGradientSpec,
  mask: &GradientSoftMaskResource,
  bbox: [f32; 4],
) {
  write_function(pdf, &mask.function);
  write_function_shading(pdf, mask.shading_id, spec, mask.function.root_id(), true);

  let mut content = Content::new();
  content
    .save_state()
    .shading(Name(mask.shading_name.as_slice()))
    .restore_state();
  let content = content.finish();
  let mut form = pdf.form_xobject(mask.form_id, &content);
  form.bbox(Rect::new(bbox[0], bbox[1], bbox[2], bbox[3]));
  {
    let mut resources = form.resources();
    resources
      .shadings()
      .pair(Name(mask.shading_name.as_slice()), mask.shading_id);
    resources.finish();
  }
  {
    let mut group = form.group();
    group.transparency();
    group.color_space().device_gray();
    group.finish();
  }
  form.finish();

  let mut state = pdf.ext_graphics(mask.ext_graphics_id);
  {
    let mut soft_mask = state.soft_mask();
    soft_mask
      .subtype(MaskType::Luminosity)
      .group(mask.form_id)
      .backdrop([0.0]);
    soft_mask.finish();
  }
  state.finish();
}
