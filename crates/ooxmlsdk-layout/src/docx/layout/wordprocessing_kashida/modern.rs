//! Compatibility15 uses all eligible Arabic connections and five expansion
//! priorities. Native LS ideal/device arrays establish the finite capacities,
//! font-design minimum ink, proportional residuals and final complex balancing.

use super::{
  LineJustificationContext, PIXEL_PT, PageItem, TextMetrics, common, device_target_width,
};
use icu_properties::{
  CodePointMapData,
  props::{JoiningGroup, Script},
};
use skrifa::{
  FontRef, MetadataProvider,
  instance::{LocationRef, Size},
  raw::TableProvider,
};
use std::sync::Arc;

struct Join {
  byte: usize,
  owner: usize,
  glyph_width_pt: f32,
}

struct Glyph {
  ideal: i32,
  natural: i32,
  space: bool,
  minimum_ink: i64,
  capacities: [i64; 5],
  expansion: i64,
  join: Option<Join>,
}

struct Portion {
  item: usize,
  x: f32,
  style: crate::docx::TextStyle,
  // Global logical glyph indices in this portion's shaped painting order.
  glyphs: Vec<usize>,
}

fn pixels(ideal: i64) -> i32 {
  let product = ideal * 600;
  ((product + product.signum() * (294912 / 2)) / 294912) as i32
}

pub(super) fn adjust(
  items: &mut [PageItem],
  line: LineJustificationContext,
  ink: bool,
  expanded: bool,
  metrics: &mut TextMetrics,
) -> Option<bool> {
  if line.terminal_overhang_pt != 0.0 {
    return None;
  }
  let mut glyphs = Vec::new();
  let mut portions = Vec::new();
  let mut tail_ideal = 0;
  let mut tail_pixels = 0;
  let mut right_to_left = None;
  // The caller's printable endpoint deliberately excludes blank-only LTR
  // portions. Printer realization still owns those semantic glyphs. Find
  // the complete logical suffix across formatting boundaries independently.
  let mut terminal_blanks = Vec::new();
  for (index, item) in items.iter().enumerate().skip(line.item_start).rev() {
    let PageItem::Text(text) = item else { continue };
    if (text.y_pt - line.y_pt).abs() >= 0.01 {
      continue;
    }
    let visible = text.text.trim_end_matches(' ');
    if visible.len() != text.text.len() {
      terminal_blanks.push((index, visible.len()));
    }
    if !visible.is_empty() {
      break;
    }
  }
  let groups = CodePointMapData::<JoiningGroup>::new();
  let scripts = CodePointMapData::<Script>::new();
  for (index, item) in items.iter().enumerate().skip(line.item_start) {
    let PageItem::Text(text) = item else {
      return None;
    };
    if (text.y_pt - line.y_pt).abs() >= 0.01 {
      continue;
    }
    let rtl = text.paragraph_bidi;
    if right_to_left.is_some_and(|direction| direction != rtl)
      || (rtl
        && text
          .style
          .resolved_bidi_level
          .is_some_and(|level| level % 2 == 0))
      || (!rtl
        && (!ink
          || !text.text.is_ascii()
          || text
            .style
            .resolved_bidi_level
            .is_some_and(|level| level % 2 != 0)))
      || text.style.wordprocessing_legacy_font_measurement.is_none()
      || text.style.small_caps
      || text.style.kashida_expansions.is_some()
      || text.text.contains('\u{0640}')
      || (rtl && text.text.chars().any(|ch| ch.is_ascii_alphanumeric()))
      || (rtl
        && text
          .text
          .chars()
          .any(|ch| ch.is_alphabetic() && scripts.get(ch) != Script::Arabic))
    {
      return None;
    }
    right_to_left = Some(rtl);
    let ideal = metrics.shape_text(&text.text, &text.style)?;
    let space = (f64::from(metrics.measure_text(" ", &text.style)) * 4096.0).round() as i64;
    let reference = space * 3;
    let mut style = text.style.clone();
    style.wordprocessing_kashida = Some(Arc::new(common::WordprocessingKashida {
      font_width_percent: style.wordprocessing_font_width_percent.unwrap_or(100),
      glyph_advances: Vec::new(),
      space_expansions: Vec::new(),
      retain_trailing_blank: false,
      unshaped_blanks: false,
    }));
    let natural = metrics.shape_text(&text.text, &style)?;
    let opportunities = if ink {
      metrics.modern_kashida_opportunities(&text.text, &style)
    } else {
      Vec::new()
    };
    if natural.glyphs.len() != ideal.glyphs.len() {
      return None;
    }
    let minima = ideal
      .font_faces
      .iter()
      .map(|face| {
        let font = FontRef::from_index(face.data.as_slice(), face.index).ok()?;
        let upem = font.head().ok()?.units_per_em();
        let Some(id) = font.charmap().map('\u{0640}') else {
          return Some(0);
        };
        Some(
          font
            .glyph_metrics(Size::new(f32::from(upem)), LocationRef::default())
            .advance_width(id)?
            .round() as i64,
        )
      })
      .collect::<Option<Vec<_>>>()?;
    let mut order: Vec<_> = (0..ideal.glyphs.len()).collect();
    order.sort_by_key(|&glyph| ideal.glyphs[glyph].text_range.start);
    let start = glyphs.len();
    let mut painting_order = vec![0; order.len()];
    let terminal_start = terminal_blanks
      .iter()
      .find_map(|&(owner, start)| (owner == index).then_some(start));
    for (logical, &shape) in order.iter().enumerate() {
      let source = &ideal.glyphs[shape];
      let device = &natural.glyphs[shape];
      let advance =
        (f64::from(source.x_advance_em) * f64::from(source.font_size_pt) * 4096.0).round() as i32;
      let natural = (device.x_advance_em * device.font_size_pt / PIXEL_PT).round() as i32;
      if advance <= 0 || natural <= 0 {
        // Zero/negative GPOS advances need the corresponding cluster-offset
        // realization, which remains owned by the existing shaping path.
        return None;
      }
      let cluster = text.text.get(source.text_range.clone())?;
      let space_glyph = cluster == " ";
      let tail = space_glyph
        && (rtl || !line.preserve_foreign_tail)
        && terminal_start.is_some_and(|start| source.text_range.start >= start);
      if tail {
        tail_ideal += i64::from(advance);
        tail_pixels += i64::from(natural);
      }
      let seen = cluster
        .chars()
        .next()
        .is_some_and(|ch| matches!(groups.get(ch), JoiningGroup::Seen | JoiningGroup::Sad));
      let mut capacities = [0; 5];
      if space_glyph && !tail {
        capacities[if rtl || !ink { 2 } else { 1 }] = if ink {
          (reference + 3) / 6
        } else {
          (f64::from(line.right_pt - line.left_pt) * 4096.0).round() as i64
        };
        capacities[4] = (f64::from(line.right_pt - line.left_pt) * 4096.0).round() as i64;
      }
      painting_order[shape] = start + logical;
      glyphs.push(Glyph {
        ideal: advance,
        natural,
        space: space_glyph,
        minimum_ink: if source.safe_to_insert_tatweel || seen {
          minima[source.font_index]
        } else {
          0
        },
        capacities,
        expansion: 0,
        join: None,
      });
    }
    for position in opportunities {
      let current = order
        .binary_search_by_key(&position.byte_index, |&shape| {
          ideal.glyphs[shape].text_range.start
        })
        .ok()?;
      if current == 0 {
        continue;
      }
      let previous = &mut glyphs[start + current - 1];
      if position.priority == 7 {
        previous.capacities[1] = reference;
      }
      if position.modern_secondary {
        previous.capacities[usize::from(position.secondary_priority)] = (reference + 1) / 2;
      }
      previous.capacities[4] = (f64::from(line.right_pt - line.left_pt) * 4096.0).round() as i64;
      previous.join = Some(Join {
        byte: position.byte_index,
        owner: portions.len(),
        glyph_width_pt: position.glyph_width_pt,
      });
    }
    portions.push(Portion {
      item: index,
      x: text.x_pt,
      style,
      glyphs: painting_order,
    });
  }
  if portions.is_empty()
    || (ink && right_to_left == Some(true) && !glyphs.iter().any(|glyph| glyph.join.is_some()))
  {
    return None;
  }
  let target_ideal = (f64::from(line.right_pt - line.left_pt) * 4096.0).round() as i64 + tail_ideal;
  let extra = if expanded {
    target_ideal
      - glyphs
        .iter()
        .map(|glyph| i64::from(glyph.ideal))
        .sum::<i64>()
  } else {
    0
  };
  allocate(&mut glyphs, extra);
  let advances: Vec<_> = glyphs
    .iter()
    .map(|glyph| {
      (
        glyph.ideal + glyph.expansion as i32,
        glyph.natural + pixels(glyph.expansion),
        glyph.space,
      )
    })
    .collect();
  let target_pixels = device_target_width(line) + tail_pixels;
  let overflow = advances
    .iter()
    .map(|&(_, natural, _)| i64::from(natural))
    .sum::<i64>()
    - target_pixels;
  let budget = if !expanded {
    overflow.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
  } else if overflow > 0 {
    overflow.min(i64::from(i32::MAX)) as i32
  } else {
    i32::MAX
  };
  let balanced = if ink {
    common::wordprocessing_device::balance_complex_advances(&advances, budget)
  } else {
    // Native ordinary Both lines retain the ordinary LS neighbour-error
    // balancer even when their shaped glyphs are Arabic. Kashida instead
    // uses the complex-script tolerance/budget policy above.
    common::wordprocessing_device::balance_advances(
      &advances
        .iter()
        .map(|&(ideal, natural, _)| (ideal, natural))
        .collect::<Vec<_>>(),
    )
  };
  let mut expansions: Vec<Vec<common::KashidaExpansion>> = vec![Vec::new(); portions.len()];
  for glyph in &glyphs {
    if let Some(join) = &glyph.join {
      let extra = pixels(glyph.expansion);
      if extra > 0 {
        let advance = extra as f32 * PIXEL_PT;
        expansions[join.owner].push(common::KashidaExpansion {
          byte_index: join.byte,
          advance: common::Pt(advance),
          glyph_count: (advance / join.glyph_width_pt).ceil().max(1.0) as usize,
        });
      }
    }
  }
  let total = balanced.iter().map(|&width| i64::from(width)).sum::<i64>();
  let mut cursor = line.left_pt
    + if right_to_left == Some(true) {
      (device_target_width(line) - total) as f32 * PIXEL_PT
    } else {
      0.0
    };
  let mut visual: Vec<_> = (0..portions.len()).collect();
  visual.sort_by(|&a, &b| portions[a].x.total_cmp(&portions[b].x));
  for owner in visual {
    let portion = &mut portions[owner];
    let PageItem::Text(text) = &mut items[portion.item] else {
      unreachable!("text portion")
    };
    let device = Arc::make_mut(portion.style.wordprocessing_kashida.as_mut()?);
    device.glyph_advances = portion
      .glyphs
      .iter()
      .map(|&index| {
        balanced[index]
          - if glyphs[index].join.is_some() {
            pixels(glyphs[index].expansion)
          } else {
            0
          }
      })
      .collect();
    if !expansions[owner].is_empty() {
      portion.style.kashida_expansions = Some(std::mem::take(&mut expansions[owner]).into());
    }
    text.x_pt = cursor;
    cursor += portion
      .glyphs
      .iter()
      .map(|&glyph| balanced[glyph])
      .sum::<i32>() as f32
      * PIXEL_PT;
    text.style = portion.style.clone();
    text.word_spacing_pt = 0.0;
  }
  Some(true)
}

fn allocate(glyphs: &mut [Glyph], mut extra: i64) {
  if extra < 0 {
    let count = glyphs
      .iter()
      .filter(|glyph| glyph.capacities[4] != 0 && glyph.space)
      .count() as i64;
    if count == 0 {
      return;
    }
    for (glyph, order) in glyphs
      .iter_mut()
      .filter(|glyph| glyph.capacities[4] != 0 && glyph.space)
      .zip(0_i64..)
    {
      glyph.expansion = -((order + 1) * -extra / count - order * -extra / count);
    }
    return;
  }
  for priority in 0..5 {
    let capacity = glyphs
      .iter()
      .map(|glyph| glyph.capacities[priority])
      .sum::<i64>();
    if extra >= capacity {
      for glyph in &mut *glyphs {
        glyph.expansion += glyph.capacities[priority];
      }
      extra -= capacity;
      continue;
    }
    let minimum = glyphs
      .iter()
      .filter(|glyph| glyph.capacities[priority] != 0)
      .map(|glyph| {
        if glyph.join.is_some() {
          (glyph.minimum_ink - glyph.expansion)
            .max(0)
            .min(glyph.capacities[priority])
        } else {
          0
        }
      })
      .sum::<i64>();
    if extra < minimum {
      for glyph in glyphs
        .iter_mut()
        .filter(|glyph| glyph.capacities[priority] != 0)
      {
        let minimum = if glyph.join.is_some() {
          (glyph.minimum_ink - glyph.expansion)
            .max(0)
            .min(glyph.capacities[priority])
        } else {
          0
        };
        if minimum <= extra {
          glyph.expansion += minimum;
          extra -= minimum;
        }
      }
      break;
    }
    let residual = extra - minimum;
    let total = capacity - minimum;
    let mut preceding = 0;
    for glyph in glyphs
      .iter_mut()
      .filter(|glyph| glyph.capacities[priority] != 0)
    {
      let minimum = if glyph.join.is_some() {
        (glyph.minimum_ink - glyph.expansion)
          .max(0)
          .min(glyph.capacities[priority])
      } else {
        0
      };
      let weight = glyph.capacities[priority] - minimum;
      glyph.expansion +=
        minimum + (preceding + weight) * residual / total - preceding * residual / total;
      preceding += weight;
    }
    break;
  }
}
