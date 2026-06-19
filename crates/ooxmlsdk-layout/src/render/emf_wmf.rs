use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
// record ids. The byte offsets below are the EMR_STRETCHDIBITS /
// EMR_SETDIBITSTODEVICE record layout fields.
const EMF_HEADER_SIZE: usize = 108;
const EMF_RECORD_HEADER_SIZE: usize = 8;
const EMR_EOF: u32 = 14;
const EMR_POLYGON: u32 = 3;
const EMR_POLYLINE: u32 = 4;
const EMR_POLYPOLYGON: u32 = 8;
const EMR_SET_WINDOW_EXT_EX: u32 = 9;
const EMR_SET_WINDOW_ORG_EX: u32 = 10;
const EMR_SET_VIEWPORT_EXT_EX: u32 = 11;
const EMR_SET_VIEWPORT_ORG_EX: u32 = 12;
const EMR_SAVE_DC: u32 = 33;
const EMR_RESTORE_DC: u32 = 34;
const EMR_SET_WORLD_TRANSFORM: u32 = 35;
const EMR_MODIFY_WORLD_TRANSFORM: u32 = 36;
const EMR_SELECT_OBJECT: u32 = 37;
const EMR_CREATE_PEN: u32 = 38;
const EMR_CREATE_BRUSH_INDIRECT: u32 = 39;
const EMR_DELETE_OBJECT: u32 = 40;
const EMR_ELLIPSE: u32 = 42;
const EMR_RECTANGLE: u32 = 43;
const EMR_SET_DIBITS_TO_DEVICE: u32 = 80;
const EMR_STRETCH_DIBITS: u32 = 81;
const EMR_EXT_CREATE_FONT_INDIRECT_W: u32 = 82;
const EMR_EXT_TEXTOUT_A: u32 = 83;
const EMR_EXT_TEXTOUT_W: u32 = 84;
const EMR_POLYGON16: u32 = 86;
const EMR_POLYLINE16: u32 = 87;
const EMR_POLYPOLYGON16: u32 = 91;
const EMR_EXT_CREATE_PEN: u32 = 95;
const EMR_BITMAP_INFO_OFFSET_OFFSET: usize = 48;
const EMR_BITMAP_INFO_SIZE_OFFSET: usize = 52;
const EMR_BITMAP_BITS_OFFSET_OFFSET: usize = 56;
const EMR_BITMAP_BITS_SIZE_OFFSET: usize = 60;
const ENHMETA_STOCK_OBJECT: u32 = 0x8000_0000;
const WHITE_BRUSH: u32 = ENHMETA_STOCK_OBJECT;
const BLACK_BRUSH: u32 = ENHMETA_STOCK_OBJECT | 4;
const NULL_BRUSH: u32 = ENHMETA_STOCK_OBJECT | 5;
const WHITE_PEN: u32 = ENHMETA_STOCK_OBJECT | 6;
const BLACK_PEN: u32 = ENHMETA_STOCK_OBJECT | 7;
const NULL_PEN: u32 = ENHMETA_STOCK_OBJECT | 8;
const MWT_IDENTITY: u32 = 1;
const MWT_LEFTMULTIPLY: u32 = 2;
const MWT_RIGHTMULTIPLY: u32 = 3;
const MWT_SET: u32 = 4;
const EMR_COMMENT: u32 = 70;
const EMR_COMMENT_EMFPLUS: u32 = 0x2B46_4D45;
const EMFPLUS_RECORD_FILL_RECTS: u16 = 0x400A;
const EMFPLUS_RECORD_DRAW_RECTS: u16 = 0x400B;
const EMFPLUS_RECORD_SET_WORLD_TRANSFORM: u16 = 0x402A;
const EMFPLUS_RECORD_RESET_WORLD_TRANSFORM: u16 = 0x402B;
const EMFPLUS_RECORD_MULTIPLY_WORLD_TRANSFORM: u16 = 0x402C;
const EMFPLUS_RECORD_TRANSLATE_WORLD_TRANSFORM: u16 = 0x402D;
const EMFPLUS_RECORD_SCALE_WORLD_TRANSFORM: u16 = 0x402E;
const EMFPLUS_DIRECT_COLOR_FLAG: u16 = 0x8000;
const EMFPLUS_COMPRESSED_FLAG: u16 = 0x4000;
const EMFPLUS_POST_MULTIPLY_FLAG: u16 = 0x2000;
const LOGFONT_FACE_NAME_CHARS: usize = 32;
// values and keeps DIB scanlines aligned to four bytes.
const BITMAPINFOHEADER_SIZE: usize = 40;
const BITMAP_WIDTH_OFFSET: usize = 4;
const BITMAP_HEIGHT_OFFSET: usize = 8;
const BITMAP_PLANES_OFFSET: usize = 12;
const BITMAP_BIT_COUNT_OFFSET: usize = 14;
const BITMAP_COMPRESSION_OFFSET: usize = 16;
const DIB_PLANES: u16 = 1;
const DIB_BIT_COUNT_24: u16 = 24;
const DIB_BIT_COUNT_32: u16 = 32;
const RGB_BYTES_PER_PIXEL: usize = 3;
const BGRA_BYTES_PER_PIXEL: usize = 4;
const DIB_ROW_ALIGNMENT_BYTES: usize = 4;
const BI_RGB: u32 = 0;
const BI_JPEG: u32 = 4;
const BI_PNG: u32 = 5;

#[derive(Clone, Debug)]
pub struct DecodedMetafile {
  pub data: Vec<u8>,
  pub content_type: &'static str,
}

pub fn decode_metafile_as_raster(
  data: &[u8],
  content_type: Option<&str>,
) -> Result<Option<DecodedMetafile>, String> {
  if !looks_like_metafile(data, content_type) {
    return Ok(None);
  }

  if let Some(raster) = decode_emf_as_raster(data)? {
    return Ok(Some(raster));
  }

  Ok(None)
}

#[derive(Clone, Debug)]
pub(crate) struct MetafileTextRun {
  pub(crate) text: String,
  pub(crate) x: f32,
  pub(crate) y: f32,
  pub(crate) font_size: Option<f32>,
}

pub(crate) fn extract_metafile_text_runs(
  data: &[u8],
  content_type: Option<&str>,
) -> Vec<MetafileTextRun> {
  if !looks_like_metafile(data, content_type) || !is_emf(data) || data.len() < EMF_HEADER_SIZE {
    return Vec::new();
  }

  let mut state = match EmfTextState::new(data) {
    Ok(state) => state,
    Err(_) => return Vec::new(),
  };
  let mut runs = Vec::new();
  let mut pos = EMF_HEADER_SIZE;
  while pos + EMF_RECORD_HEADER_SIZE <= data.len() {
    let Ok(record_type) = read_u32(data, pos) else {
      break;
    };
    let Ok(record_size) = read_u32(data, pos + 4) else {
      break;
    };
    let record_size = record_size as usize;
    if record_size < EMF_RECORD_HEADER_SIZE || pos + record_size > data.len() {
      break;
    }

    match record_type {
      EMR_SET_WINDOW_ORG_EX if record_size >= 16 => {
        state.window_org_x = read_i32(data, pos + 8).unwrap_or(state.window_org_x);
        state.window_org_y = read_i32(data, pos + 12).unwrap_or(state.window_org_y);
      }
      EMR_SET_WINDOW_EXT_EX if record_size >= 16 => {
        state.window_ext_x = read_i32(data, pos + 8)
          .unwrap_or(state.window_ext_x)
          .abs()
          .max(1);
        state.window_ext_y = read_i32(data, pos + 12)
          .unwrap_or(state.window_ext_y)
          .abs()
          .max(1);
      }
      EMR_SET_VIEWPORT_ORG_EX if record_size >= 16 => {
        state.viewport_org_x = read_i32(data, pos + 8).unwrap_or(state.viewport_org_x);
        state.viewport_org_y = read_i32(data, pos + 12).unwrap_or(state.viewport_org_y);
      }
      EMR_SET_VIEWPORT_EXT_EX if record_size >= 16 => {
        state.viewport_ext_x = read_i32(data, pos + 8).unwrap_or(state.viewport_ext_x);
        state.viewport_ext_y = read_i32(data, pos + 12).unwrap_or(state.viewport_ext_y);
      }
      EMR_SET_WORLD_TRANSFORM if record_size >= 32 => {
        if let Ok(transform) = read_xform(data, pos + 8) {
          state.world_transform = transform;
        }
      }
      EMR_MODIFY_WORLD_TRANSFORM if record_size >= 36 => {
        if let (Ok(transform), Ok(mode)) = (read_xform(data, pos + 8), read_u32(data, pos + 32)) {
          state.world_transform = match mode {
            MWT_IDENTITY => EmfTransform::identity(),
            MWT_LEFTMULTIPLY => transform.multiply(state.world_transform),
            MWT_RIGHTMULTIPLY => state.world_transform.multiply(transform),
            MWT_SET => transform,
            _ => state.world_transform,
          };
        }
      }
      EMR_EXT_CREATE_FONT_INDIRECT_W if record_size >= 104 => {
        if let Some((object_id, font)) = read_logfont_object(data, pos, record_size)
          && object_id & ENHMETA_STOCK_OBJECT == 0
        {
          state.fonts.insert(object_id, font);
        }
      }
      EMR_SELECT_OBJECT if record_size >= 12 => {
        let object_id = read_u32(data, pos + 8).unwrap_or(0);
        if state.fonts.contains_key(&object_id) {
          state.current_font = Some(object_id);
        }
      }
      EMR_DELETE_OBJECT if record_size >= 12 => {
        let object_id = read_u32(data, pos + 8).unwrap_or(0);
        state.fonts.remove(&object_id);
        if state.current_font == Some(object_id) {
          state.current_font = None;
        }
      }
      EMR_EXT_TEXTOUT_W => {
        if let Some(text) = extract_emr_ext_text_out_w(data, pos, record_size)
          && !text.trim().is_empty()
          && let Some(run) = state.text_run(data, pos, record_size, text)
        {
          runs.push(run);
        }
      }
      EMR_EXT_TEXTOUT_A => {
        if let Some(text) = extract_emr_ext_text_out_a(data, pos, record_size)
          && !text.trim().is_empty()
          && let Some(run) = state.text_run(data, pos, record_size, text)
        {
          runs.push(run);
        }
      }
      EMR_EOF => break,
      _ => {}
    }

    pos += record_size;
  }

  runs
}

fn looks_like_metafile(data: &[u8], content_type: Option<&str>) -> bool {
  matches!(
    content_type,
    Some("image/x-wmf" | "image/wmf" | "image/x-emf" | "image/emf" | "application/x-msmetafile")
  ) || is_emf(data)
}

fn decode_emf_as_raster(data: &[u8]) -> Result<Option<DecodedMetafile>, String> {
  if !is_emf(data) {
    return Ok(None);
  }
  if data.len() < EMF_HEADER_SIZE {
    return Err("EMF header is truncated".into());
  }

  let mut pos = EMF_HEADER_SIZE;
  let mut bitmap_record = None;

  while pos + EMF_RECORD_HEADER_SIZE <= data.len() {
    let record_type = read_u32(data, pos)?;
    let record_size = read_u32(data, pos + 4)? as usize;
    if record_size < EMF_RECORD_HEADER_SIZE || pos + record_size > data.len() {
      return Err(format!(
        "invalid EMF record at offset {pos}: type=0x{record_type:08x} size={record_size}"
      ));
    }

    if matches!(record_type, EMR_SET_DIBITS_TO_DEVICE | EMR_STRETCH_DIBITS)
      && bitmap_record
        .replace((record_type, pos, record_size))
        .is_some()
    {
      return Err("multiple EMF bitmap records are not supported yet".into());
    }

    pos += record_size;
    if record_type == EMR_EOF {
      break;
    }
  }

  let (record_type, record_offset, record_size) = match bitmap_record {
    Some(record) => record,
    None => return decode_vector_emf_as_png(data).map(Some),
  };
  decode_bitmap_record_as_raster(data, record_type, record_offset, record_size).map(Some)
}

#[derive(Clone, Copy, Debug)]
struct EmfColor {
  r: u8,
  g: u8,
  b: u8,
}

#[derive(Clone, Copy, Debug)]
struct EmfPoint {
  x: i32,
  y: i32,
}

#[derive(Clone, Copy, Debug)]
struct EmfTransform {
  m11: f32,
  m12: f32,
  m21: f32,
  m22: f32,
  dx: f32,
  dy: f32,
}

impl EmfTransform {
  fn identity() -> Self {
    Self {
      m11: 1.0,
      m12: 0.0,
      m21: 0.0,
      m22: 1.0,
      dx: 0.0,
      dy: 0.0,
    }
  }

  fn apply(self, point: EmfPoint) -> (f32, f32) {
    let x = point.x as f32;
    let y = point.y as f32;
    (
      x * self.m11 + y * self.m21 + self.dx,
      x * self.m12 + y * self.m22 + self.dy,
    )
  }

  fn multiply(self, other: Self) -> Self {
    Self {
      m11: self.m11 * other.m11 + self.m12 * other.m21,
      m12: self.m11 * other.m12 + self.m12 * other.m22,
      m21: self.m21 * other.m11 + self.m22 * other.m21,
      m22: self.m21 * other.m12 + self.m22 * other.m22,
      dx: self.dx * other.m11 + self.dy * other.m21 + other.dx,
      dy: self.dx * other.m12 + self.dy * other.m22 + other.dy,
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct EmfPen {
  color: EmfColor,
  width: usize,
}

#[derive(Clone, Debug)]
struct EmfFont {
  height: i32,
}

#[derive(Clone, Debug)]
struct EmfTextState {
  width: usize,
  height: usize,
  window_org_x: i32,
  window_org_y: i32,
  window_ext_x: i32,
  window_ext_y: i32,
  viewport_org_x: i32,
  viewport_org_y: i32,
  viewport_ext_x: i32,
  viewport_ext_y: i32,
  world_transform: EmfTransform,
  fonts: std::collections::HashMap<u32, EmfFont>,
  current_font: Option<u32>,
}

impl EmfTextState {
  fn new(data: &[u8]) -> Result<Self, String> {
    let left = read_i32(data, 8)?;
    let top = read_i32(data, 12)?;
    let right = read_i32(data, 16)?;
    let bottom = read_i32(data, 20)?;
    let width = (right - left + 1).max(1) as usize;
    let height = (bottom - top + 1).max(1) as usize;

    Ok(Self {
      width,
      height,
      window_org_x: 0,
      window_org_y: 0,
      window_ext_x: width as i32,
      window_ext_y: height as i32,
      viewport_org_x: 0,
      viewport_org_y: 0,
      viewport_ext_x: width as i32,
      viewport_ext_y: height as i32,
      world_transform: EmfTransform::identity(),
      fonts: std::collections::HashMap::new(),
      current_font: None,
    })
  }

  fn map_point(&self, point: EmfPoint) -> (f32, f32) {
    let (x, y) = self.world_transform.apply(point);
    let scale_x = self.viewport_ext_x as f32 / self.window_ext_x.max(1) as f32;
    let scale_y = self.viewport_ext_y as f32 / self.window_ext_y.max(1) as f32;
    (
      self.viewport_org_x as f32 + (x - self.window_org_x as f32) * scale_x,
      self.viewport_org_y as f32 + (y - self.window_org_y as f32) * scale_y,
    )
  }

  fn map_height(&self, height: i32) -> f32 {
    let (_, y0) = self.map_point(EmfPoint { x: 0, y: 0 });
    let (_, y1) = self.map_point(EmfPoint {
      x: 0,
      y: height.abs(),
    });
    (y1 - y0).abs()
  }

  fn text_run(
    &self,
    data: &[u8],
    record_offset: usize,
    record_size: usize,
    text: String,
  ) -> Option<MetafileTextRun> {
    let text_record = ext_text_record(data, record_offset, record_size)?;
    let (x, y) = self.map_point(EmfPoint {
      x: text_record.x,
      y: text_record.y,
    });
    let font_size = self
      .current_font
      .and_then(|id| self.fonts.get(&id))
      .map(|font| self.map_height(font.height));
    Some(MetafileTextRun {
      text,
      x: x / self.width.max(1) as f32,
      y: y / self.height.max(1) as f32,
      font_size: font_size.map(|height| height / self.height.max(1) as f32),
    })
  }
}

#[derive(Clone, Debug)]
struct EmfVectorState {
  width: usize,
  height: usize,
  window_org_x: i32,
  window_org_y: i32,
  window_ext_x: i32,
  window_ext_y: i32,
  viewport_org_x: i32,
  viewport_org_y: i32,
  viewport_ext_x: i32,
  viewport_ext_y: i32,
  world_transform: EmfTransform,
  brush_colors: std::collections::HashMap<u32, EmfColor>,
  pens: std::collections::HashMap<u32, EmfPen>,
  current_brush: Option<EmfColor>,
  current_pen: Option<EmfPen>,
  rgb: Vec<u8>,
}

impl EmfVectorState {
  fn new(data: &[u8]) -> Result<Self, String> {
    let left = read_i32(data, 8)?;
    let top = read_i32(data, 12)?;
    let right = read_i32(data, 16)?;
    let bottom = read_i32(data, 20)?;
    let width = (right - left + 1).max(1) as usize;
    let height = (bottom - top + 1).max(1) as usize;

    Ok(Self {
      width,
      height,
      window_org_x: 0,
      window_org_y: 0,
      window_ext_x: width as i32,
      window_ext_y: height as i32,
      viewport_org_x: 0,
      viewport_org_y: 0,
      viewport_ext_x: width as i32,
      viewport_ext_y: height as i32,
      world_transform: EmfTransform::identity(),
      brush_colors: std::collections::HashMap::new(),
      pens: std::collections::HashMap::new(),
      current_brush: None,
      current_pen: Some(EmfPen {
        color: EmfColor { r: 0, g: 0, b: 0 },
        width: 1,
      }),
      rgb: vec![255; width * height * RGB_BYTES_PER_PIXEL],
    })
  }

  fn map_point(&self, point: EmfPoint) -> (f32, f32) {
    let (x, y) = self.world_transform.apply(point);
    let scale_x = self.viewport_ext_x as f32 / self.window_ext_x.max(1) as f32;
    let scale_y = self.viewport_ext_y as f32 / self.window_ext_y.max(1) as f32;
    (
      self.viewport_org_x as f32 + (x - self.window_org_x as f32) * scale_x,
      self.viewport_org_y as f32 + (y - self.window_org_y as f32) * scale_y,
    )
  }

  fn set_pixel(&mut self, x: i32, y: i32, color: EmfColor) {
    if x < 0 || y < 0 {
      return;
    }
    let (x, y) = (x as usize, y as usize);
    if x >= self.width || y >= self.height {
      return;
    }
    let offset = (y * self.width + x) * RGB_BYTES_PER_PIXEL;
    self.rgb[offset] = color.r;
    self.rgb[offset + 1] = color.g;
    self.rgb[offset + 2] = color.b;
  }

  fn fill_polygon(&mut self, points: &[EmfPoint]) {
    let Some(color) = self.current_brush else {
      return;
    };
    if points.len() < 3 {
      return;
    }

    let mapped = points
      .iter()
      .map(|point| self.map_point(*point))
      .collect::<Vec<_>>();
    for y in 0..self.height {
      let scan_y = y as f32 + 0.5;
      let mut intersections = Vec::new();
      for index in 0..mapped.len() {
        let (x1, y1) = mapped[index];
        let (x2, y2) = mapped[(index + 1) % mapped.len()];
        if (y1 <= scan_y && y2 > scan_y) || (y2 <= scan_y && y1 > scan_y) {
          let t = (scan_y - y1) / (y2 - y1);
          intersections.push(x1 + t * (x2 - x1));
        }
      }
      intersections.sort_by(|a, b| a.total_cmp(b));
      for pair in intersections.chunks_exact(2) {
        let start = pair[0].floor().max(0.0) as usize;
        let end = pair[1].ceil().min(self.width as f32) as usize;
        for x in start..end {
          self.set_pixel(x as i32, y as i32, color);
        }
      }
    }
  }

  fn draw_polyline(&mut self, points: &[EmfPoint], closed: bool) {
    let Some(pen) = self.current_pen else {
      return;
    };
    if points.len() < 2 {
      return;
    }
    for pair in points.windows(2) {
      self.draw_line(pair[0], pair[1], pen);
    }
    if closed {
      self.draw_line(points[points.len() - 1], points[0], pen);
    }
  }

  fn draw_line(&mut self, a: EmfPoint, b: EmfPoint, pen: EmfPen) {
    let (x0, y0) = self.map_point(a);
    let (x1, y1) = self.map_point(b);
    let mut x0 = x0.round() as i32;
    let mut y0 = y0.round() as i32;
    let x1 = x1.round() as i32;
    let y1 = y1.round() as i32;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;
    loop {
      self.set_pen_pixel(x0, y0, pen);
      if x0 == x1 && y0 == y1 {
        break;
      }
      let e2 = 2 * error;
      if e2 >= dy {
        error += dy;
        x0 += sx;
      }
      if e2 <= dx {
        error += dx;
        y0 += sy;
      }
    }
  }

  fn set_pen_pixel(&mut self, x: i32, y: i32, pen: EmfPen) {
    let radius = (pen.width.max(1) / 2) as i32;
    for yy in y - radius..=y + radius {
      for xx in x - radius..=x + radius {
        self.set_pixel(xx, yy, pen.color);
      }
    }
  }

  fn fill_rect(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
    let points = [
      EmfPoint { x: left, y: top },
      EmfPoint { x: right, y: top },
      EmfPoint {
        x: right,
        y: bottom,
      },
      EmfPoint { x: left, y: bottom },
    ];
    self.fill_polygon(&points);
    self.draw_polyline(&points, true);
  }

  fn fill_ellipse(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
    let steps = 72;
    let cx = (left + right) as f32 / 2.0;
    let cy = (top + bottom) as f32 / 2.0;
    let rx = (right - left).abs() as f32 / 2.0;
    let ry = (bottom - top).abs() as f32 / 2.0;
    let mut points = Vec::with_capacity(steps);
    for index in 0..steps {
      let theta = index as f32 * std::f32::consts::TAU / steps as f32;
      points.push(EmfPoint {
        x: (cx + theta.cos() * rx).round() as i32,
        y: (cy + theta.sin() * ry).round() as i32,
      });
    }
    self.fill_polygon(&points);
    self.draw_polyline(&points, true);
  }

  fn select_object(&mut self, object_id: u32) {
    match object_id {
      WHITE_BRUSH => {
        self.current_brush = Some(EmfColor {
          r: 255,
          g: 255,
          b: 255,
        })
      }
      BLACK_BRUSH => self.current_brush = Some(EmfColor { r: 0, g: 0, b: 0 }),
      NULL_BRUSH => self.current_brush = None,
      WHITE_PEN => {
        self.current_pen = Some(EmfPen {
          color: EmfColor {
            r: 255,
            g: 255,
            b: 255,
          },
          width: 1,
        })
      }
      BLACK_PEN => {
        self.current_pen = Some(EmfPen {
          color: EmfColor { r: 0, g: 0, b: 0 },
          width: 1,
        })
      }
      NULL_PEN => self.current_pen = None,
      _ => {
        if let Some(brush) = self.brush_colors.get(&object_id).copied() {
          self.current_brush = Some(brush);
        }
        if let Some(pen) = self.pens.get(&object_id).copied() {
          self.current_pen = Some(pen);
        }
      }
    }
  }
}

fn decode_vector_emf_as_png(data: &[u8]) -> Result<DecodedMetafile, String> {
  let mut state = EmfVectorState::new(data)?;
  let mut pos = EMF_HEADER_SIZE;

  while pos + EMF_RECORD_HEADER_SIZE <= data.len() {
    let record_type = read_u32(data, pos)?;
    let record_size = read_u32(data, pos + 4)? as usize;
    if record_size < EMF_RECORD_HEADER_SIZE || pos + record_size > data.len() {
      return Err(format!(
        "invalid EMF record at offset {pos}: type=0x{record_type:08x} size={record_size}"
      ));
    }

    match record_type {
      EMR_SET_WINDOW_ORG_EX if record_size >= 16 => {
        state.window_org_x = read_i32(data, pos + 8)?;
        state.window_org_y = read_i32(data, pos + 12)?;
      }
      EMR_SET_WINDOW_EXT_EX if record_size >= 16 => {
        state.window_ext_x = read_i32(data, pos + 8)?.abs().max(1);
        state.window_ext_y = read_i32(data, pos + 12)?.abs().max(1);
      }
      EMR_SET_VIEWPORT_ORG_EX if record_size >= 16 => {
        state.viewport_org_x = read_i32(data, pos + 8)?;
        state.viewport_org_y = read_i32(data, pos + 12)?;
      }
      EMR_SET_VIEWPORT_EXT_EX if record_size >= 16 => {
        state.viewport_ext_x = read_i32(data, pos + 8)?;
        state.viewport_ext_y = read_i32(data, pos + 12)?;
      }
      EMR_SAVE_DC => {}
      EMR_RESTORE_DC => {}
      EMR_SET_WORLD_TRANSFORM if record_size >= 32 => {
        state.world_transform = read_xform(data, pos + 8)?;
      }
      EMR_MODIFY_WORLD_TRANSFORM if record_size >= 36 => {
        let transform = read_xform(data, pos + 8)?;
        let mode = read_u32(data, pos + 32)?;
        state.world_transform = match mode {
          MWT_IDENTITY => EmfTransform::identity(),
          MWT_LEFTMULTIPLY => transform.multiply(state.world_transform),
          MWT_RIGHTMULTIPLY => state.world_transform.multiply(transform),
          MWT_SET => transform,
          _ => state.world_transform,
        };
      }
      EMR_CREATE_PEN if record_size >= 28 => {
        let object_id = read_u32(data, pos + 8)?;
        if object_id & ENHMETA_STOCK_OBJECT == 0 {
          let width = read_i32(data, pos + 16)?.unsigned_abs().max(1) as usize;
          state.pens.insert(
            object_id,
            EmfPen {
              color: read_color_ref(data, pos + 24)?,
              width,
            },
          );
        }
      }
      EMR_CREATE_BRUSH_INDIRECT if record_size >= 24 => {
        let object_id = read_u32(data, pos + 8)?;
        state
          .brush_colors
          .insert(object_id, read_color_ref(data, pos + 16)?);
      }
      EMR_EXT_CREATE_PEN if record_size >= 56 => {
        let object_id = read_u32(data, pos + 8)?;
        if object_id & ENHMETA_STOCK_OBJECT == 0 {
          let width = read_u32(data, pos + 32)?.max(1) as usize;
          state.pens.insert(
            object_id,
            EmfPen {
              color: read_color_ref(data, pos + 40)?,
              width,
            },
          );
        }
      }
      EMR_SELECT_OBJECT if record_size >= 12 => {
        state.select_object(read_u32(data, pos + 8)?);
      }
      EMR_DELETE_OBJECT if record_size >= 12 => {
        let object_id = read_u32(data, pos + 8)?;
        state.brush_colors.remove(&object_id);
        state.pens.remove(&object_id);
      }
      EMR_POLYGON if record_size >= 28 => {
        if let Some(points) = read_points_i32(data, pos + 28, read_u32(data, pos + 24)? as usize) {
          state.fill_polygon(&points);
          state.draw_polyline(&points, true);
        }
      }
      EMR_POLYGON16 if record_size >= 28 => {
        if let Some(points) = read_points_i16(data, pos + 28, read_u32(data, pos + 24)? as usize) {
          state.fill_polygon(&points);
          state.draw_polyline(&points, true);
        }
      }
      EMR_POLYLINE if record_size >= 28 => {
        if let Some(points) = read_points_i32(data, pos + 28, read_u32(data, pos + 24)? as usize) {
          state.draw_polyline(&points, false);
        }
      }
      EMR_POLYLINE16 if record_size >= 28 => {
        if let Some(points) = read_points_i16(data, pos + 28, read_u32(data, pos + 24)? as usize) {
          state.draw_polyline(&points, false);
        }
      }
      EMR_POLYPOLYGON if record_size >= 36 => {
        for points in read_poly_polygons_i32(data, pos, record_size)? {
          state.fill_polygon(&points);
          state.draw_polyline(&points, true);
        }
      }
      EMR_POLYPOLYGON16 if record_size >= 36 => {
        for points in read_poly_polygons_i16(data, pos, record_size)? {
          state.fill_polygon(&points);
          state.draw_polyline(&points, true);
        }
      }
      EMR_RECTANGLE if record_size >= 24 => {
        state.fill_rect(
          read_i32(data, pos + 8)?,
          read_i32(data, pos + 12)?,
          read_i32(data, pos + 16)?,
          read_i32(data, pos + 20)?,
        );
      }
      EMR_ELLIPSE if record_size >= 24 => {
        state.fill_ellipse(
          read_i32(data, pos + 8)?,
          read_i32(data, pos + 12)?,
          read_i32(data, pos + 16)?,
          read_i32(data, pos + 20)?,
        );
      }
      EMR_COMMENT if record_size >= 16 => {
        process_emf_plus_comment(data, pos, record_size, &mut state)?;
      }
      EMR_EOF => break,
      _ => {}
    }

    pos += record_size;
  }

  Ok(DecodedMetafile {
    data: rgb_to_png(&state.rgb, state.width as u32, state.height as u32)?,
    content_type: "image/png",
  })
}

fn decode_bitmap_record_as_raster(
  data: &[u8],
  record_type: u32,
  record_offset: usize,
  _record_size: usize,
) -> Result<DecodedMetafile, String> {
  let (off_bmi_src, cb_bmi_src, off_bits_src, cb_bits_src) = match record_type {
    EMR_STRETCH_DIBITS => (
      read_u32(data, record_offset + EMR_BITMAP_INFO_OFFSET_OFFSET)? as usize,
      read_u32(data, record_offset + EMR_BITMAP_INFO_SIZE_OFFSET)? as usize,
      read_u32(data, record_offset + EMR_BITMAP_BITS_OFFSET_OFFSET)? as usize,
      read_u32(data, record_offset + EMR_BITMAP_BITS_SIZE_OFFSET)? as usize,
    ),
    EMR_SET_DIBITS_TO_DEVICE => (
      read_u32(data, record_offset + EMR_BITMAP_INFO_OFFSET_OFFSET)? as usize,
      read_u32(data, record_offset + EMR_BITMAP_INFO_SIZE_OFFSET)? as usize,
      read_u32(data, record_offset + EMR_BITMAP_BITS_OFFSET_OFFSET)? as usize,
      read_u32(data, record_offset + EMR_BITMAP_BITS_SIZE_OFFSET)? as usize,
    ),
    _ => {
      return Err(format!(
        "unsupported EMF bitmap record type 0x{record_type:08x}"
      ));
    }
  };

  let bmi_start = record_offset + off_bmi_src;
  let bits_start = record_offset + off_bits_src;
  let bmi_end = bmi_start
    .checked_add(cb_bmi_src)
    .ok_or_else(|| "bitmap info range overflows".to_string())?;
  let bits_end = bits_start
    .checked_add(cb_bits_src)
    .ok_or_else(|| "bitmap bits range overflows".to_string())?;
  if bmi_end > data.len() || bits_end > data.len() {
    return Err("EMF bitmap record points outside the file".into());
  }
  if cb_bmi_src < BITMAPINFOHEADER_SIZE {
    return Err("EMF bitmap info header is too small".into());
  }

  let header_size = read_u32(data, bmi_start)? as usize;
  if header_size < BITMAPINFOHEADER_SIZE {
    return Err(format!("unsupported BITMAPINFOHEADER size: {header_size}"));
  }

  let width = read_i32(data, bmi_start + BITMAP_WIDTH_OFFSET)?;
  let height = read_i32(data, bmi_start + BITMAP_HEIGHT_OFFSET)?;
  let planes = read_u16(data, bmi_start + BITMAP_PLANES_OFFSET)?;
  let bit_count = read_u16(data, bmi_start + BITMAP_BIT_COUNT_OFFSET)?;
  let compression = read_u32(data, bmi_start + BITMAP_COMPRESSION_OFFSET)?;

  if planes != DIB_PLANES {
    return Err(format!("unsupported DIB planes value: {planes}"));
  }

  let bits = &data[bits_start..bits_end];
  match compression {
    BI_JPEG => Ok(DecodedMetafile {
      data: bits.to_vec(),
      content_type: "image/jpeg",
    }),
    BI_PNG => Ok(DecodedMetafile {
      data: bits.to_vec(),
      content_type: "image/png",
    }),
    BI_RGB => dib_to_png(bits, width, height, bit_count),
    other => Err(format!("unsupported DIB compression: {other}")),
  }
}

fn dib_to_png(
  bits: &[u8],
  width: i32,
  height: i32,
  bit_count: u16,
) -> Result<DecodedMetafile, String> {
  if width <= 0 || height == 0 {
    return Err(format!("unsupported DIB size {width}x{height}"));
  }

  let width = width as usize;
  let top_down = height < 0;
  let height_abs = height.unsigned_abs() as usize;

  let bytes_per_pixel = match bit_count {
    DIB_BIT_COUNT_24 => RGB_BYTES_PER_PIXEL,
    DIB_BIT_COUNT_32 => BGRA_BYTES_PER_PIXEL,
    other => return Err(format!("unsupported BI_RGB bit depth: {other}")),
  };
  let row_stride = (width * bytes_per_pixel).next_multiple_of(DIB_ROW_ALIGNMENT_BYTES);
  let required_size = row_stride
    .checked_mul(height_abs)
    .ok_or_else(|| "bitmap dimensions overflow".to_string())?;
  if bits.len() < required_size {
    return Err(format!(
      "bitmap payload is truncated: need {required_size} bytes, got {}",
      bits.len()
    ));
  }

  let mut rgb = vec![0u8; width * height_abs * RGB_BYTES_PER_PIXEL];
  for row in 0..height_abs {
    let src_row = if top_down { row } else { height_abs - 1 - row };
    let src_offset = src_row * row_stride;
    let dest_offset = row * width * RGB_BYTES_PER_PIXEL;
    let src = &bits[src_offset..src_offset + row_stride];
    let dest = &mut rgb[dest_offset..dest_offset + width * RGB_BYTES_PER_PIXEL];

    match bit_count {
      DIB_BIT_COUNT_24 => {
        for col in 0..width {
          let src_pixel =
            &src[col * RGB_BYTES_PER_PIXEL..col * RGB_BYTES_PER_PIXEL + RGB_BYTES_PER_PIXEL];
          let dest_pixel =
            &mut dest[col * RGB_BYTES_PER_PIXEL..col * RGB_BYTES_PER_PIXEL + RGB_BYTES_PER_PIXEL];
          dest_pixel[0] = src_pixel[2];
          dest_pixel[1] = src_pixel[1];
          dest_pixel[2] = src_pixel[0];
        }
      }
      DIB_BIT_COUNT_32 => {
        for col in 0..width {
          let src_pixel =
            &src[col * BGRA_BYTES_PER_PIXEL..col * BGRA_BYTES_PER_PIXEL + BGRA_BYTES_PER_PIXEL];
          let dest_pixel =
            &mut dest[col * RGB_BYTES_PER_PIXEL..col * RGB_BYTES_PER_PIXEL + RGB_BYTES_PER_PIXEL];
          dest_pixel[0] = src_pixel[2];
          dest_pixel[1] = src_pixel[1];
          dest_pixel[2] = src_pixel[0];
        }
      }
      _ => unreachable!(),
    }
  }

  Ok(DecodedMetafile {
    data: rgb_to_png(&rgb, width as u32, height_abs as u32)?,
    content_type: "image/png",
  })
}

fn process_emf_plus_comment(
  data: &[u8],
  record_offset: usize,
  record_size: usize,
  state: &mut EmfVectorState,
) -> Result<(), String> {
  // EMR_COMMENT_EMFPLUS chunks as a stream of 12-byte EMF+ record headers.
  let data_size = read_u32(data, record_offset + 8)? as usize;
  let comment_identifier = read_u32(data, record_offset + 12)?;
  if comment_identifier != EMR_COMMENT_EMFPLUS || data_size < 4 {
    return Ok(());
  }
  let mut cursor = record_offset + 16;
  let end = record_offset
    .checked_add(12)
    .and_then(|offset| offset.checked_add(data_size))
    .map(|end| end.min(record_offset + record_size))
    .ok_or_else(|| "EMF+ comment range overflows".to_string())?;
  while cursor + 12 <= end {
    let record_type = read_u16(data, cursor)?;
    let flags = read_u16(data, cursor + 2)?;
    let size = read_u32(data, cursor + 4)? as usize;
    let payload_size = read_u32(data, cursor + 8)? as usize;
    if size < 12 || cursor + size > end {
      break;
    }
    let payload = cursor + 12;
    if payload + payload_size <= cursor + size {
      process_emf_plus_record(data, payload, payload_size, record_type, flags, state)?;
    }
    cursor += size;
  }
  Ok(())
}

fn process_emf_plus_record(
  data: &[u8],
  payload: usize,
  payload_size: usize,
  record_type: u16,
  flags: u16,
  state: &mut EmfVectorState,
) -> Result<(), String> {
  match record_type {
    EMFPLUS_RECORD_FILL_RECTS | EMFPLUS_RECORD_DRAW_RECTS => {
      let mut cursor = payload;
      let mut fill = None;
      if record_type == EMFPLUS_RECORD_FILL_RECTS {
        if payload_size < 8 {
          return Ok(());
        }
        let brush = read_u32(data, cursor)?;
        cursor += 4;
        if flags & EMFPLUS_DIRECT_COLOR_FLAG != 0 {
          fill = Some(argb_color(brush));
        }
      }
      let count = read_u32(data, cursor)? as usize;
      cursor += 4;
      for _ in 0..count {
        let (left, top, width, height, next) =
          read_emf_plus_rect(data, cursor, flags & EMFPLUS_COMPRESSED_FLAG != 0)?;
        cursor = next;
        let old_brush = state.current_brush;
        if let Some(color) = fill {
          state.current_brush = Some(color);
          state.fill_rect(left, top, left + width, top + height);
          state.current_brush = old_brush;
        } else {
          state.fill_rect(left, top, left + width, top + height);
        }
      }
    }
    EMFPLUS_RECORD_SET_WORLD_TRANSFORM if payload_size >= 24 => {
      state.world_transform = read_xform(data, payload)?;
    }
    EMFPLUS_RECORD_RESET_WORLD_TRANSFORM => {
      state.world_transform = EmfTransform::identity();
    }
    EMFPLUS_RECORD_MULTIPLY_WORLD_TRANSFORM if payload_size >= 24 => {
      let transform = read_xform(data, payload)?;
      state.world_transform = if flags & EMFPLUS_POST_MULTIPLY_FLAG != 0 {
        state.world_transform.multiply(transform)
      } else {
        transform.multiply(state.world_transform)
      };
    }
    EMFPLUS_RECORD_TRANSLATE_WORLD_TRANSFORM if payload_size >= 8 => {
      let transform = EmfTransform {
        dx: read_f32(data, payload)?,
        dy: read_f32(data, payload + 4)?,
        ..EmfTransform::identity()
      };
      state.world_transform = if flags & EMFPLUS_POST_MULTIPLY_FLAG != 0 {
        state.world_transform.multiply(transform)
      } else {
        transform.multiply(state.world_transform)
      };
    }
    EMFPLUS_RECORD_SCALE_WORLD_TRANSFORM if payload_size >= 8 => {
      let transform = EmfTransform {
        m11: read_f32(data, payload)?,
        m22: read_f32(data, payload + 4)?,
        ..EmfTransform::identity()
      };
      state.world_transform = if flags & EMFPLUS_POST_MULTIPLY_FLAG != 0 {
        state.world_transform.multiply(transform)
      } else {
        transform.multiply(state.world_transform)
      };
    }
    _ => {}
  }
  Ok(())
}

fn read_emf_plus_rect(
  data: &[u8],
  offset: usize,
  compressed: bool,
) -> Result<(i32, i32, i32, i32, usize), String> {
  if compressed {
    Ok((
      i32::from(read_i16(data, offset)?),
      i32::from(read_i16(data, offset + 2)?),
      i32::from(read_i16(data, offset + 4)?),
      i32::from(read_i16(data, offset + 6)?),
      offset + 8,
    ))
  } else {
    Ok((
      read_f32(data, offset)?.round() as i32,
      read_f32(data, offset + 4)?.round() as i32,
      read_f32(data, offset + 8)?.round() as i32,
      read_f32(data, offset + 12)?.round() as i32,
      offset + 16,
    ))
  }
}

fn argb_color(value: u32) -> EmfColor {
  EmfColor {
    r: ((value >> 16) & 0xff) as u8,
    g: ((value >> 8) & 0xff) as u8,
    b: (value & 0xff) as u8,
  }
}

fn read_poly_polygons_i32(
  data: &[u8],
  record_offset: usize,
  record_size: usize,
) -> Result<Vec<Vec<EmfPoint>>, String> {
  let polygon_count = read_u32(data, record_offset + 24)? as usize;
  let total_points = read_u32(data, record_offset + 28)? as usize;
  let counts_offset = record_offset + 32;
  let points_offset = counts_offset
    .checked_add(polygon_count * 4)
    .ok_or_else(|| "EMF polygon counts overflow".to_string())?;
  if points_offset > record_offset + record_size {
    return Ok(Vec::new());
  }
  let mut counts = Vec::with_capacity(polygon_count);
  for index in 0..polygon_count {
    counts.push(read_u32(data, counts_offset + index * 4)? as usize);
  }
  let Some(points) = read_points_i32(data, points_offset, total_points) else {
    return Ok(Vec::new());
  };
  Ok(split_polygons(points, counts))
}

fn read_poly_polygons_i16(
  data: &[u8],
  record_offset: usize,
  record_size: usize,
) -> Result<Vec<Vec<EmfPoint>>, String> {
  let polygon_count = read_u32(data, record_offset + 24)? as usize;
  let total_points = read_u32(data, record_offset + 28)? as usize;
  let counts_offset = record_offset + 32;
  let points_offset = counts_offset
    .checked_add(polygon_count * 4)
    .ok_or_else(|| "EMF polygon counts overflow".to_string())?;
  if points_offset > record_offset + record_size {
    return Ok(Vec::new());
  }
  let mut counts = Vec::with_capacity(polygon_count);
  for index in 0..polygon_count {
    counts.push(read_u32(data, counts_offset + index * 4)? as usize);
  }
  let Some(points) = read_points_i16(data, points_offset, total_points) else {
    return Ok(Vec::new());
  };
  Ok(split_polygons(points, counts))
}

fn split_polygons(points: Vec<EmfPoint>, counts: Vec<usize>) -> Vec<Vec<EmfPoint>> {
  let mut polygons = Vec::with_capacity(counts.len());
  let mut cursor = 0usize;
  for count in counts {
    let end = cursor.saturating_add(count).min(points.len());
    polygons.push(points[cursor..end].to_vec());
    cursor = end;
  }
  polygons
}

fn read_points_i32(data: &[u8], offset: usize, count: usize) -> Option<Vec<EmfPoint>> {
  let end = offset.checked_add(count.checked_mul(8)?)?;
  if end > data.len() {
    return None;
  }
  let mut points = Vec::with_capacity(count);
  for index in 0..count {
    let point_offset = offset + index * 8;
    points.push(EmfPoint {
      x: read_i32(data, point_offset).ok()?,
      y: read_i32(data, point_offset + 4).ok()?,
    });
  }
  Some(points)
}

fn read_points_i16(data: &[u8], offset: usize, count: usize) -> Option<Vec<EmfPoint>> {
  let end = offset.checked_add(count.checked_mul(4)?)?;
  if end > data.len() {
    return None;
  }
  let mut points = Vec::with_capacity(count);
  for index in 0..count {
    let point_offset = offset + index * 4;
    points.push(EmfPoint {
      x: i32::from(read_i16(data, point_offset).ok()?),
      y: i32::from(read_i16(data, point_offset + 2).ok()?),
    });
  }
  Some(points)
}

fn read_color_ref(data: &[u8], offset: usize) -> Result<EmfColor, String> {
  let color_ref = read_u32(data, offset)?;
  Ok(EmfColor {
    r: (color_ref & 0xff) as u8,
    g: ((color_ref >> 8) & 0xff) as u8,
    b: ((color_ref >> 16) & 0xff) as u8,
  })
}

fn read_xform(data: &[u8], offset: usize) -> Result<EmfTransform, String> {
  Ok(EmfTransform {
    m11: read_f32(data, offset)?,
    m12: read_f32(data, offset + 4)?,
    m21: read_f32(data, offset + 8)?,
    m22: read_f32(data, offset + 12)?,
    dx: read_f32(data, offset + 16)?,
    dy: read_f32(data, offset + 20)?,
  })
}

fn rgb_to_png(rgb: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
  let mut output = Vec::new();
  let encoder = PngEncoder::new(&mut output);
  encoder
    .write_image(rgb, width, height, ColorType::Rgb8.into())
    .map_err(|err| err.to_string())?;
  Ok(output)
}

fn is_emf(data: &[u8]) -> bool {
  data.len() >= EMF_HEADER_SIZE
    && matches!(read_u32(data, 0), Ok(1))
    && matches!(read_u32(data, 4), Ok(size) if size as usize == EMF_HEADER_SIZE)
}

fn extract_emr_ext_text_out_w(
  data: &[u8],
  record_offset: usize,
  record_size: usize,
) -> Option<String> {
  let text = ext_text_record(data, record_offset, record_size)?;
  let byte_len = text.characters.checked_mul(2)?;
  let start = record_offset.checked_add(text.string_offset)?;
  let end = start.checked_add(byte_len)?;
  let bytes = data.get(start..end)?;
  let units = bytes
    .chunks_exact(2)
    .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
    .collect::<Vec<_>>();
  Some(
    String::from_utf16_lossy(&units)
      .trim_end_matches('\0')
      .to_string(),
  )
}

fn extract_emr_ext_text_out_a(
  data: &[u8],
  record_offset: usize,
  record_size: usize,
) -> Option<String> {
  let text = ext_text_record(data, record_offset, record_size)?;
  let start = record_offset.checked_add(text.string_offset)?;
  let end = start.checked_add(text.characters)?;
  let bytes = data.get(start..end)?;
  Some(
    bytes
      .iter()
      .take_while(|byte| **byte != 0)
      .map(|byte| char::from(*byte))
      .collect(),
  )
}

#[derive(Clone, Copy, Debug)]
struct ExtTextRecord {
  x: i32,
  y: i32,
  characters: usize,
  string_offset: usize,
}

fn ext_text_record(data: &[u8], record_offset: usize, record_size: usize) -> Option<ExtTextRecord> {
  // with rclBounds, graphics mode, scales, then EMRTEXT. EMRTEXT::offString is
  // relative to the record start.
  const EMRTEXT_OFFSET: usize = 36;
  const EMRTEXT_REFERENCE_X_OFFSET: usize = EMRTEXT_OFFSET;
  const EMRTEXT_REFERENCE_Y_OFFSET: usize = EMRTEXT_OFFSET + 4;
  const EMRTEXT_CHARS_OFFSET: usize = EMRTEXT_OFFSET + 8;
  const EMRTEXT_STRING_OFFSET: usize = EMRTEXT_OFFSET + 12;
  let minimum_size = EMRTEXT_OFFSET + 40;
  if record_size < minimum_size {
    return None;
  }
  let characters = read_u32(data, record_offset + EMRTEXT_CHARS_OFFSET).ok()? as usize;
  let string_offset = read_u32(data, record_offset + EMRTEXT_STRING_OFFSET).ok()? as usize;
  if characters == 0 || string_offset >= record_size {
    return None;
  }
  Some(ExtTextRecord {
    x: read_i32(data, record_offset + EMRTEXT_REFERENCE_X_OFFSET).ok()?,
    y: read_i32(data, record_offset + EMRTEXT_REFERENCE_Y_OFFSET).ok()?,
    characters,
    string_offset,
  })
}

fn read_logfont_object(
  data: &[u8],
  record_offset: usize,
  record_size: usize,
) -> Option<(u32, EmfFont)> {
  // EMR_EXTCREATEFONTINDIRECTW reads an object index followed by LOGFONTW.
  const OBJECT_ID_OFFSET: usize = 8;
  const LOGFONT_OFFSET: usize = 12;
  const LOGFONT_HEIGHT_OFFSET: usize = LOGFONT_OFFSET;
  const LOGFONT_FACE_NAME_OFFSET: usize = LOGFONT_OFFSET + 28;
  let face_end = LOGFONT_FACE_NAME_OFFSET.checked_add(LOGFONT_FACE_NAME_CHARS * 2)?;
  if record_size < face_end {
    return None;
  }
  let object_id = read_u32(data, record_offset + OBJECT_ID_OFFSET).ok()?;
  let height = read_i32(data, record_offset + LOGFONT_HEIGHT_OFFSET).ok()?;
  Some((object_id, EmfFont { height }))
}

fn read_u16(data: &[u8], offset: usize) -> Result<u16, String> {
  let bytes = data
    .get(offset..offset + 2)
    .ok_or_else(|| format!("read past end of buffer at offset {offset}"))?;
  Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_i16(data: &[u8], offset: usize) -> Result<i16, String> {
  let bytes = data
    .get(offset..offset + 2)
    .ok_or_else(|| format!("read past end of buffer at offset {offset}"))?;
  Ok(i16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32(data: &[u8], offset: usize) -> Result<u32, String> {
  let bytes = data
    .get(offset..offset + 4)
    .ok_or_else(|| format!("read past end of buffer at offset {offset}"))?;
  Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_i32(data: &[u8], offset: usize) -> Result<i32, String> {
  let bytes = data
    .get(offset..offset + 4)
    .ok_or_else(|| format!("read past end of buffer at offset {offset}"))?;
  Ok(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_f32(data: &[u8], offset: usize) -> Result<f32, String> {
  let bytes = data
    .get(offset..offset + 4)
    .ok_or_else(|| format!("read past end of buffer at offset {offset}"))?;
  Ok(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}
