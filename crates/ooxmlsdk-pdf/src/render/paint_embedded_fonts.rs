//! Word's bitmap realization for CFF and variable font faces.
//!
//! The private CFF versus TrueType and bitmap-option controls distinguish this
//! branch from ordinary font substitution. Font rasters are glyph surfaces,
//! not source pictures: Screen/Print use 300/600 DPI independently of picture
//! optimization, with a Win-metric cell and one average glyph of horizontal padding.
use std::borrow::Cow;
use std::collections::hash_map::Entry;

use ooxmlsdk_layout::fonts::{FontFaceCacheKey, FontFaceData};
use resvg::tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};
use skrifa::{
  FontRef, GlyphId, MetadataProvider,
  instance::Size,
  outline::{DrawSettings, HintingInstance, Target},
  raw::TableProvider,
};

use super::{HashMap, ImageCrop, ImageItem, PaintItem, PaintText, TextMetrics, common};
use crate::PdfOptimizeFor;

#[path = "paint_cff_hints.rs"]
mod cff_hints;

const MAX_BITMAP_PIXELS: u64 = 16_777_216;

#[derive(Default)]
pub(super) struct FontBitmaps {
  bitmap_missing_fonts: bool,
  variable_fonts: bool,
  kinds: HashMap<FontFaceCacheKey, BitmapFontKind>,
  instances: HashMap<(FontFaceCacheKey, u32), RasterInstance>,
}

#[derive(Clone, Copy)]
enum BitmapFontKind {
  Ordinary,
  Variable,
  Cff {
    cid: bool,
    carrier_family: &'static str,
  },
}

struct RasterInstance {
  hinting: HintingInstance,
  glyphs: HashMap<u32, Option<RasterGlyph>>,
}

struct RasterGlyph {
  left: i32,
  top: i32,
  width: u32,
  height: u32,
  alpha: Vec<u8>,
}

impl RasterGlyph {
  fn composite(&self, target: &mut [u8], width: u32, height: u32, x: i32, y: i32) {
    let x = x + self.left;
    let y = y + self.top;
    let left = x.max(0);
    let right = (x + self.width as i32).min(width as i32);
    let top = y.max(0);
    let bottom = (y + self.height as i32).min(height as i32);
    if left >= right || top >= bottom {
      return;
    }
    for row in top..bottom {
      let source_start = ((row - y) as u32 * self.width + (left - x) as u32) as usize;
      let target_start = (row as u32 * width + left as u32) as usize;
      let length = (right - left) as usize;
      for (dest, source) in target[target_start..target_start + length]
        .iter_mut()
        .zip(&self.alpha[source_start..source_start + length])
      {
        *dest |= *source;
      }
    }
  }
}

impl FontBitmaps {
  pub(super) fn new(bitmap_missing_fonts: bool, archival: bool) -> Self {
    Self {
      bitmap_missing_fonts,
      variable_fonts: bitmap_missing_fonts || archival,
      ..Self::default()
    }
  }

  fn kind(&mut self, face: &FontFaceData) -> BitmapFontKind {
    *self.kinds.entry(face.cache_key()).or_insert_with(|| {
      let Ok(font) = FontRef::from_index(face.data.as_slice(), face.index) else {
        return BitmapFontKind::Ordinary;
      };
      if !font.axes().is_empty() {
        return BitmapFontKind::Variable;
      }
      if let Ok(cff) = font.cff() {
        return BitmapFontKind::Cff {
          cid: skrifa::raw::ps::cff::CffFontRef::new_cff(cff.offset_data().as_bytes(), 0, None)
            .is_ok_and(|font| font.is_cid()),
          // The native PDF bitmap path preserves the realized face's
          // charset and PANOSE family in its replacement LOGFONT. Office
          // CreateFontIndirectW/GetFontData observations and independent GDI
          // controls establish these GB2312 mappings for serif/sans faces.
          // The PDF resource retains the requested Calibri name separately.
          carrier_family: font.os2().ok().map_or("Calibri", |os2| {
            if os2.ul_code_page_range_1().unwrap_or_default() & (1 << 18) == 0 {
              "Calibri"
            } else if os2.panose_10()[0] == 2 && (2..=10).contains(&os2.panose_10()[1]) {
              "MingLiU-ExtB"
            } else {
              "SimSun"
            }
          }),
        };
      }
      BitmapFontKind::Ordinary
    })
  }

  fn needs_bitmap(&mut self, text: &PaintText<'_>, run: &super::PaintGlyphFontRun) -> bool {
    match self.kind(&run.font_face) {
      BitmapFontKind::Ordinary => false,
      BitmapFontKind::Variable => self.variable_fonts,
      BitmapFontKind::Cff { cid, .. } => {
        // Native CID-CFF controls retain ASCII as Type1 with the bitmap
        // option off, but force non-ASCII portions through GGO_BITMAP.
        // Name-keyed CFF is separate: e.g. Libertine's Latin-1 é remains text.
        self.bitmap_missing_fonts
          || (cid
            && run.glyphs.iter().any(|glyph| {
              text
                .item
                .text
                .get(glyph.text_range.clone())
                .is_some_and(|text| !text.is_ascii())
            }))
      }
    }
  }

  pub(super) fn realize<'doc>(
    &mut self,
    items: &mut [PaintItem<'doc>],
    quality: PdfOptimizeFor,
    page_width_pt: f32,
    page_height_pt: f32,
    metrics: &mut TextMetrics,
  ) {
    for item in items {
      match item {
        PaintItem::Text(text) => {
          if let Some(realized) =
            self.realize_text(text, quality, page_width_pt, page_height_pt, metrics)
          {
            *item = realized;
          } else if let Some(fragments) = self.bitmap_font_fragments(text) {
            let children = fragments
              .into_iter()
              .map(|fragment| {
                self
                  .realize_text(&fragment, quality, page_width_pt, page_height_pt, metrics)
                  .unwrap_or_else(|| PaintItem::Text(Box::new(fragment)))
              })
              .collect();
            *item = PaintItem::Group {
              mask: None,
              clip: None,
              transform: None,
              blend_mode: common::BlendMode::Normal,
              opacity: 1.0,
              flatten_identity: true,
              items: children,
            };
          }
        }
        PaintItem::Group { items, .. } => {
          self.realize(items, quality, page_width_pt, page_height_pt, metrics)
        }
        _ => {}
      }
    }
  }

  fn realize_text<'doc>(
    &mut self,
    text: &PaintText<'doc>,
    quality: PdfOptimizeFor,
    page_width_pt: f32,
    page_height_pt: f32,
    metrics: &mut TextMetrics,
  ) -> Option<PaintItem<'doc>> {
    let (images, baseline, semantic_x) =
      self.images(text, quality, page_width_pt, page_height_pt)?;
    let carrier_family = text
      .portions
      .iter()
      .flat_map(|portion| portion.glyphs.iter().flatten())
      .find_map(|run| match self.kind(&run.font_face) {
        BitmapFontKind::Cff { carrier_family, .. } => Some(carrier_family),
        _ => None,
      })
      .unwrap_or("Calibri");
    let mut semantic = text.item.clone();
    semantic.x_pt = semantic_x;
    if semantic.text.starts_with(' ') {
      semantic.text = Cow::Owned(semantic.text.trim_start_matches(' ').to_owned());
    }
    semantic.y_pt = baseline;
    semantic.origin_is_baseline = true;
    semantic.style.baseline_shift_pt = 0.0;
    semantic.style.embedded_fonts = None;
    semantic.style.font_family = Some(Cow::Borrowed(carrier_family));
    semantic.style.high_ansi_font_family = None;
    semantic.style.east_asia_font_family = None;
    semantic.style.complex_font_family = None;
    semantic.style.fallback_font_family = None;
    semantic.style.high_ansi_fallback_font_family = None;
    semantic.style.east_asia_fallback_font_family = None;
    semantic.style.complex_fallback_font_family = None;
    semantic.style.font_charset = None;
    semantic.style.high_ansi_font_charset = None;
    semantic.style.wordprocessingml_east_asia_font_charset = None;
    semantic.style.complex_font_charset = None;
    semantic.style.opacity = 0.0;
    semantic.style.bitmap_semantic_text = true;
    semantic.style.semantic_character_advances_pt = None;
    // Resolve the physical replacement independently of the source's font
    // slots. Its requested PDF name remains Calibri, even where the GDI
    // charset/family mapper selected another physical face.
    semantic.style.wordprocessingml_font_slots = false;
    semantic.style.wordprocessingml_font_hint = None;
    semantic.style.wordprocessingml_resolved_font_slot = None;
    semantic.style.complex_script = None;
    semantic.style.right_to_left = None;
    // Native bitmap carriers use nominal fallback-font widths, not
    // the original face's positioning features or kerned advances.
    semantic.style.kerning_minimum_size_pt = Some(f32::MAX);
    let mut semantic = PaintText::from_layout_text(semantic, None, None, page_width_pt, metrics);
    semantic.source_frame_index = text.source_frame_index;
    semantic.source_line_index = text.source_line_index;
    let mut children = images.into_iter().map(PaintItem::Image).collect::<Vec<_>>();
    children.push(PaintItem::Text(Box::new(semantic)));
    Some(PaintItem::Group {
      mask: None,
      clip: text
        .portions
        .first()
        .and_then(|portion| portion.clip)
        .or(text.item.paint_clip),
      transform: None,
      blend_mode: common::BlendMode::Normal,
      opacity: 1.0,
      flatten_identity: true,
      items: children,
    })
  }

  fn bitmap_font_fragments<'doc>(
    &mut self,
    text: &PaintText<'doc>,
  ) -> Option<Vec<PaintText<'doc>>> {
    if !supported_style(text)
      || !text.portions.iter().any(|portion| {
        portion
          .glyphs
          .as_ref()
          .is_some_and(|runs| runs.iter().any(|run| self.needs_bitmap(text, run)))
      })
    {
      return None;
    }
    let mut fragments = Vec::new();
    for portion in &text.portions {
      for run in super::merge_word_font_runs(portion.glyphs.as_ref()?).iter() {
        let start = run
          .glyphs
          .iter()
          .map(|glyph| glyph.text_range.start)
          .min()?;
        let end = run.glyphs.iter().map(|glyph| glyph.text_range.end).max()?;
        let mut fragment = text.clone();
        fragment.item.text = Cow::Owned(text.item.text.get(start..end)?.to_owned());
        fragment.item.x_pt = portion.x_pt + run.x_offset_pt;
        let width = run
          .glyphs
          .iter()
          .map(|glyph| glyph.x_advance * run.font_size_pt)
          .sum();
        fragment.width_pt = width;
        let mut single = portion.clone();
        single.x_pt = fragment.item.x_pt;
        single.text_range = 0..end - start;
        single.width_pt = width;
        let mut run = run.clone();
        run.x_offset_pt = 0.0;
        for glyph in &mut run.glyphs {
          glyph.text_range = glyph.text_range.start - start..glyph.text_range.end - start;
        }
        single.glyphs = Some(std::iter::once(run).collect());
        fragment.portions = vec![single];
        fragments.push(fragment);
      }
    }
    (fragments.len() > 1).then_some(fragments)
  }

  fn images<'doc>(
    &mut self,
    text: &PaintText<'doc>,
    quality: PdfOptimizeFor,
    page_width_pt: f32,
    page_height_pt: f32,
  ) -> Option<(Vec<ImageItem<'doc>>, f32, f32)> {
    let style = &text.item.style;
    if !supported_style(text)
      || text.portions.iter().any(|portion| {
        portion.glyphs.as_ref().is_none_or(|runs| {
          super::merge_word_font_runs(runs)
            .iter()
            .any(|run| !self.needs_bitmap(text, run))
        })
      })
    {
      return None;
    }
    let dpi = match quality {
      PdfOptimizeFor::Screen => 300.0,
      PdfOptimizeFor::Print => 600.0,
    };
    let device = common::wordprocessing_device::page_coordinates(page_width_pt, page_height_pt)?;
    let mut images = Vec::new();
    let mut semantic_baseline = None;
    let mut semantic_x = None;
    for portion in &text.portions {
      let runs = portion.glyphs.as_ref()?;
      for run in super::merge_word_font_runs(runs).iter() {
        let font = FontRef::from_index(run.font_face.data.as_slice(), run.font_face.index).ok()?;
        let variable = !font.axes().is_empty();
        if run.font_face.synthetic_bold || run.font_face.synthetic_italic {
          return None;
        }
        // Word drops a leading blank into its own unpainted portion before
        // rasterizing text; trailing and internal blanks remain in the cell.
        let skip = run
          .glyphs
          .iter()
          .take_while(|glyph| text.item.text.get(glyph.text_range.clone()) == Some(" "))
          .count();
        let glyphs = &run.glyphs[skip..];
        if glyphs.is_empty() {
          continue;
        }
        let leading = run.glyphs[..skip]
          .iter()
          .map(|glyph| glyph.x_advance * run.font_size_pt)
          .sum::<f32>();
        let origin_x = portion.x_pt + run.x_offset_pt + leading;
        semantic_x.get_or_insert(origin_x);
        let upem = f32::from(font.head().ok()?.units_per_em());
        let os2 = font.os2().ok()?;
        let ascent = f32::from(os2.us_win_ascent()) / upem;
        let cell_height = (f32::from(os2.us_win_ascent()) + f32::from(os2.us_win_descent())) / upem;
        // The font is selected in Word's 600-DPI reference DC. Natural
        // character cells own the bitmap extent, independently of the ideal
        // advances used to position the separately rasterized glyphs.
        let printer_em = (run.font_size_pt * 600.0 / 72.0).round();
        let natural = font.glyph_metrics(Size::new(printer_em), run.font_face.location());
        let advance_px = if variable {
          // Completed Word device advances, already balanced across the body
          // line. Natural metrics alone lose GPOS and neighbouring rounding.
          (glyphs
            .iter()
            .map(|glyph| glyph.x_advance * run.font_size_pt)
            .sum::<f32>()
            / 0.12)
            .round() as i64
        } else {
          glyphs.iter().try_fold(0_i64, |total, glyph| {
            Some(total + natural.advance_width(GlyphId::new(glyph.glyph_id))?.round() as i64)
          })?
        };
        let ascent_px = (ascent * printer_em).round() as i64;
        let height_px = (cell_height * printer_em).round() as i64;
        // Installed CFF portions share the completed line baseline with
        // neighbouring vector text. The private/variable route retains its
        // separately established source-cell metric contract.
        let installed_cff = !variable && !run.font_face.document_embedded;
        let source_y = common::wordprocessing_device::source_coordinate(if installed_cff {
          text.baseline_y
        } else {
          text.item.y_pt
        });
        let baseline_px = source_y
          + if installed_cff || text.item.origin_is_baseline {
            0
          } else {
            ascent_px
          };
        semantic_baseline.get_or_insert(baseline_px as f32 * 0.12);
        let baseline_pt = device.map_device_point(0, baseline_px).1;
        let ascent_pt = device.map_device_point(0, ascent_px).1;
        // MSO's bitmap allocator reserves one average glyph beyond the text.
        // Keep replay FLOAT division/multiplication before pixel truncation:
        // a 392-dot span can become 440.99997 pixels, whose width is 440.
        let (_, _, advance_pt, cell_height_pt) =
          device.map_device_bitmap_rectangle((0, 0, advance_px, height_px));
        let width_pt = advance_pt + advance_pt / glyphs.len() as f32;
        let height_pt = cell_height_pt;
        if !width_pt.is_finite() || !height_pt.is_finite() || width_pt <= 0.0 || height_pt <= 0.0 {
          return None;
        }
        let cell_width = advance_pt * dpi / 72.0;
        let width = (cell_width + cell_width / glyphs.len() as f32).floor() as u32;
        let height = (height_pt * dpi / 72.0).floor() as u32;
        let ppem = (printer_em * dpi / 600.0).round() as u32;
        // Bound generated surfaces like the existing SVG raster path. An
        // oversized private-font run can retain vector output rather than
        // allocate an unbounded bitmap while preparing the whole document.
        if width == 0
          || height == 0
          || ppem == 0
          || u64::from(width) * u64::from(height) > MAX_BITMAP_PIXELS
        {
          return None;
        }
        let mut alpha = vec![0; width as usize * height as usize];
        let baseline = (ascent * ppem as f32).round();
        let mut cursor = 0.0;
        for glyph in glyphs {
          if let Some(bitmap) = self.glyph(&run.font_face, ppem, glyph.glyph_id)? {
            // GetGlyphOutline(GGO_BITMAP) realizes one glyph independently.
            // Native CFF controls reproduce both complete golden masks with
            // zero differing pixels when those glyphs use rounded origins.
            bitmap.composite(
              &mut alpha,
              width,
              height,
              (cursor + glyph.x_offset * ppem as f32).round() as i32,
              (baseline - glyph.y_offset * ppem as f32).round() as i32,
            );
          }
          cursor += glyph.x_advance * ppem as f32;
        }
        let raster = super::super::image::PreparedRasterImage::font_bitmap(
          width,
          height,
          [style.color.r, style.color.g, style.color.b],
          alpha,
        );
        images.push(ImageItem {
          font_bitmap: Some(raster),
          x_pt: image_number(device.map_point(origin_x, 0.0).0),
          y_pt: page_height_pt
            - image_number(page_height_pt - baseline_pt + ascent_pt - height_pt)
            - image_number(height_pt),
          width_pt: image_number(width_pt),
          height_pt: image_number(height_pt),
          crop: ImageCrop::default(),
          clip_path: &[],
          rotation_deg: 0.0,
          flip_horizontal: false,
          flip_vertical: false,
          data: Cow::Borrowed(&[]),
          content_type: None,
          blip_compression_state: common::BlipCompressionState::NotApplicable,
          metafile_monochrome_dib_palette_override: None,
          metafile_background_color: None,
          metafile_external_header: None,
          metafile_fixed_output_profile: common::MetafileFixedOutputProfile::default(),
          alt_text: None,
          hyperlink_url: None,
          semantic_metafile_text: false,
          metafile_semantic_text_includes_raster_backdrop: false,
          signature_line: None,
          metafile_native_size: false,
        });
      }
    }
    Some((images, semantic_baseline?, semantic_x?))
  }

  fn glyph(&mut self, face: &FontFaceData, ppem: u32, glyph: u32) -> Option<Option<&RasterGlyph>> {
    let font = FontRef::from_index(face.data.as_slice(), face.index).ok()?;
    let outlines = font.outline_glyphs();
    let instance = match self.instances.entry((face.cache_key(), ppem)) {
      Entry::Occupied(entry) => entry.into_mut(),
      Entry::Vacant(entry) => entry.insert(RasterInstance {
        hinting: HintingInstance::new(
          &outlines,
          Size::new(ppem as f32),
          face.location(),
          Target::Mono,
        )
        .ok()?,
        glyphs: HashMap::default(),
      }),
    };
    let bitmap = match instance.glyphs.entry(glyph) {
      Entry::Occupied(entry) => entry.into_mut(),
      Entry::Vacant(entry) => {
        let mut pen = BitmapPen(PathBuilder::new());
        outlines
          .get(GlyphId::new(glyph))?
          .draw(DrawSettings::hinted(&instance.hinting, false), &mut pen)
          .ok()?;
        let bitmap = if let Some(path) = pen.0.finish() {
          let path = cff_hints::horizontal(&font, ppem, glyph, &path).unwrap_or(path);
          let bounds = path.bounds();
          let left = bounds.left().floor();
          let top = bounds.top().floor();
          let right = bounds.right().ceil();
          let bottom = bounds.bottom().ceil();
          let width = (right - left) as u32;
          let height = (bottom - top) as u32;
          if u64::from(width) * u64::from(height) > MAX_BITMAP_PIXELS {
            return None;
          }
          let mut pixmap = Pixmap::new(width, height)?;
          let mut paint = Paint::default();
          paint.set_color_rgba8(0, 0, 0, 255);
          paint.anti_alias = false;
          pixmap.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::from_row(1.0, 0.0, 0.0, -1.0, -left, bottom),
            None,
          );
          Some(RasterGlyph {
            left: left as i32,
            top: -(bottom as i32),
            width: pixmap.width(),
            height: pixmap.height(),
            alpha: pixmap
              .data()
              .as_chunks::<4>()
              .0
              .iter()
              .map(|pixel| pixel[3])
              .collect(),
          })
        } else {
          None
        };
        entry.insert(bitmap)
      }
    };
    Some(bitmap.as_ref())
  }
}

// Decorations and transformed text retain their separate composition owners.
fn supported_style(text: &PaintText<'_>) -> bool {
  let style = &text.item.style;
  !(style.semantic_only
    || style.opacity != 1.0
    || style.rotation_deg != 0.0
    || style.pdf_glyph_outlines
    || style.pdf_glyph_outline_options.is_some()
    || style.outline_color.is_some()
    || style.highlight.is_some()
    || style.underline
    || style.strikethrough
    || style.bold
    || style.italic
    || style.horizontal_scale.unwrap_or(1.0) != 1.0
    || style.character_spacing_pt != 0.0
    || text.item.word_spacing_pt != 0.0
    || text.item.paragraph_bidi
    || text.item.hyperlink_url.is_some())
}

// Word serializes image matrices with five significant digits. Quantize the
// bottom coordinate independently from height before returning to page-top
// coordinates; adding independently rounded top and height changes its phase.
fn image_number(value: f32) -> f32 {
  if value == 0.0 {
    return value;
  }
  let scale = 10.0_f64.powi(4 - f64::from(value).abs().log10().floor() as i32);
  ((f64::from(value) * scale).round() / scale) as f32
}

struct BitmapPen(PathBuilder);

impl skrifa::outline::OutlinePen for BitmapPen {
  fn move_to(&mut self, x: f32, y: f32) {
    self.0.move_to(x, y);
  }
  fn line_to(&mut self, x: f32, y: f32) {
    self.0.line_to(x, y);
  }
  fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) {
    self.0.quad_to(cx, cy, x, y);
  }
  fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
    self.0.cubic_to(cx0, cy0, cx1, cy1, x, y);
  }
  fn close(&mut self) {
    self.0.close();
  }
}
