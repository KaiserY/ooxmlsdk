use super::*;

#[test]
fn texture_source_removes_only_bitmap_guards_through_every_nested_container() {
  let glow = ImageEffect::Glow {
    radius_px: 12.0,
    raster_length_scale: 0.6,
    bounds_radius_scale: 0.6,
    bounds_radius_offset_px: 0.96,
    spread_ratio: 0.5,
    spread_kernel: GlowSpreadKernel::AlphaOutset,
    spread_radius_rounding: GlowSpreadRadiusRounding::Inward,
    blur_kernel: GlowBlurKernel::WordStatic3dGaussian,
    color: ResolvedEffectColor {
      color: RgbColor {
        r: 10,
        g: 20,
        b: 30,
      },
      alpha: 102,
    },
  };
  let shadow = ImageEffect::OuterShadow {
    blur_radius_px: 12.0,
    distance_px: 31.0,
    raster_length_scale: 0.7,
    distance_length_scale: 0.8,
    bounds_radius_scale: 0.7,
    bounds_radius_offset_px: 0.24,
    blur_kernel: ShadowBlurKernel::WordTextBalanced {
      prescale_divisor: 3,
    },
    direction_degrees: 212.0,
    distance_mode: ShadowDistanceMode::PreTransformOffset,
    transform: ImageEffectTransform {
      scale_x: 0.7,
      scale_y: 0.5,
      skew_x: 0.1,
      skew_y: -0.2,
      shift_x_px: 2.0,
      shift_y_px: 3.0,
    },
    alignment: (0.0, 0.5),
    rotate_with_shape: true,
    color: ResolvedEffectColor {
      color: RgbColor {
        r: 90,
        g: 100,
        b: 110,
      },
      alpha: 153,
    },
  };
  let ImageEffect::Reflection(mut reflection) = reflection(&a::Reflection::default()) else {
    unreachable!()
  };
  reflection.bounds_radius_offset_px = 0.16;
  for (seed, mut expected) in [
    (glow.clone(), glow.clone()),
    (shadow.clone(), shadow),
    (
      ImageEffect::Reflection(reflection),
      ImageEffect::Reflection(reflection),
    ),
  ] {
    match &mut expected {
      ImageEffect::Glow {
        bounds_radius_offset_px,
        ..
      }
      | ImageEffect::OuterShadow {
        bounds_radius_offset_px,
        ..
      } => *bounds_radius_offset_px = 0.0,
      ImageEffect::Reflection(r) => r.bounds_radius_offset_px = 0.0,
      _ => unreachable!(),
    }
    let wrap = |effect| {
      let leaf = ImageEffectContainer {
        kind: ImageEffectContainerKind::Tree,
        effects: vec![effect, ImageEffect::Identity],
      };
      ImageEffectContainer {
        kind: ImageEffectContainerKind::Sibling,
        effects: vec![ImageEffect::Container(ImageEffectContainer {
          kind: ImageEffectContainerKind::Tree,
          effects: vec![ImageEffect::AlphaModulate(ImageEffectContainer {
            kind: ImageEffectContainerKind::Tree,
            effects: vec![ImageEffect::Blend {
              container: leaf,
              blend_mode: ImageEffectBlendMode::Multiply,
            }],
          })],
        })],
      }
    };
    let allocation = wrap(seed);
    let snapshot = allocation.clone();
    let texture = allocation.without_bitmap_allocation_guards();
    assert_eq!(allocation, snapshot);
    assert_eq!(texture, wrap(expected));
    assert_eq!(texture.without_bitmap_allocation_guards(), texture);
  }
  let allocation = ImageEffectContainer {
    kind: ImageEffectContainerKind::Tree,
    effects: vec![glow],
  };
  let texture = allocation.without_bitmap_allocation_guards();
  let outer = container_output_bounds(&allocation, 30.0, 10.0).unwrap();
  let inner = container_output_bounds(&texture, 30.0, 10.0).unwrap();
  assert!((inner.left_pt - outer.left_pt - 0.72).abs() < 0.00001);
  assert!((outer.right_pt - inner.right_pt - 0.72).abs() < 0.00001);
  assert_eq!(
    container_output_bounds(&allocation, 30.0, 10.0),
    Some(outer)
  );
}

fn logical_geometry() -> ImageEffectSourceGeometry {
  ImageEffectSourceGeometry {
    paint_left_px: 0.0,
    paint_top_px: 0.0,
    paint_width_px: 10.0,
    paint_height_px: 10.0,
    shadow_anchor_left_px: 0.0,
    shadow_anchor_top_px: 0.0,
    shadow_anchor_width_px: 10.0,
    shadow_anchor_height_px: 10.0,
    anchor_left_px: 0.0,
    anchor_top_px: 0.0,
    anchor_width_px: 10.0,
    anchor_height_px: 10.0,
    ramp_left_px: 0.0,
    ramp_top_px: 0.0,
    ramp_width_px: 10.0,
    ramp_height_px: 10.0,
  }
}

#[test]
fn mapped_graph_nested_sources_keep_logical_root_anchor_ownership() {
  for scale in [
    EffectRasterScale { x: 2.0, y: 5.0 },
    EffectRasterScale { x: 5.0, y: 2.0 },
  ] {
    let mut source = image::RgbaImage::new(100, 100);
    source.put_pixel(20, 20, image::Rgba([90, 120, 180, 255]));
    let child = ImageEffectContainer {
      kind: ImageEffectContainerKind::Tree,
      effects: vec![
        ImageEffect::SourceReference(ImageEffectSourceReference::Children),
        ImageEffect::RelativeOffset {
          offset_x: 0.5,
          offset_y: 0.5,
        },
      ],
    };
    // Sibling, Tree, Blend and AlphaModulate must all propagate the map.
    for nested in [
      ImageEffect::Container(child.clone()),
      ImageEffect::Blend {
        container: child.clone(),
        blend_mode: ImageEffectBlendMode::Over,
      },
      ImageEffect::AlphaModulate(child),
    ] {
      let mut result = if matches!(nested, ImageEffect::AlphaModulate(_)) {
        image::RgbaImage::from_pixel(100, 100, image::Rgba([90, 120, 180, 255]))
      } else {
        image::RgbaImage::new(100, 100)
      };
      let graph = ImageEffectContainer {
        kind: ImageEffectContainerKind::Sibling,
        effects: vec![nested],
      };
      apply_container_to_padded_image_with_sources_on_raster(
        &mut result,
        &graph,
        logical_geometry(),
        ImageEffectSourceImages {
          children: Some(&source),
          bounds: ImageEffectSourcePixelBounds {
            children: Some(ImageEffectContentBounds {
              left_px: 7.0,
              top_px: 9.0,
              width_px: 4.0,
              height_px: 6.0,
            }),
            ..Default::default()
          },
          ..Default::default()
        },
        scale,
      )
      .unwrap();
      // A source binding replaces paint bounds, not the root alignment
      // anchor. Relative offsets therefore retain half of the 10-unit root.
      let expected = (20 + (5.0 * scale.x) as u32, 20 + (5.0 * scale.y) as u32);
      for (x, y, p) in result.enumerate_pixels() {
        assert_eq!(
          p[3],
          if (x, y) == expected { 255 } else { 0 },
          "scale={scale:?}, pixel={x},{y}"
        );
      }
    }
  }
}

#[test]
fn mapped_graph_rejects_invalid_grids_without_mutating_input() {
  let graph = ImageEffectContainer {
    kind: ImageEffectContainerKind::Tree,
    effects: vec![ImageEffect::AlphaReplace(0)],
  };
  let source = image::RgbaImage::from_pixel(4, 5, image::Rgba([30, 50, 70, 255]));
  let wrong = image::RgbaImage::new(5, 4);
  for (scale, child) in [
    (EffectRasterScale { x: 0.0, y: 1.0 }, None),
    (
      EffectRasterScale {
        x: 1.0,
        y: f32::NAN,
      },
      None,
    ),
    (EffectRasterScale { x: 1.0, y: 1.0 }, Some(&wrong)),
  ] {
    let mut result = source.clone();
    assert!(
      apply_container_to_padded_image_with_sources_on_raster(
        &mut result,
        &graph,
        logical_geometry(),
        ImageEffectSourceImages {
          children: child,
          ..Default::default()
        },
        scale
      )
      .is_none()
    );
    assert_eq!(result, source);
  }
}

#[test]
fn mapped_elliptical_spread_has_exact_integer_boundary_membership() {
  let mut alpha = image::GrayImage::new(51, 51);
  alpha.put_pixel(25, 25, image::Luma([1]));
  for (rx, ry) in [(15, 5), (5, 15), (0, 9), (9, 0), (7, 7), (2, 5)] {
    let result = dilate_nontransparent_alpha_ellipse(&alpha, rx, ry);
    for (x, y, p) in result.enumerate_pixels() {
      let dx = x.abs_diff(25) as usize;
      let dy = y.abs_diff(25) as usize;
      let inside =
        dx <= rx && dy <= ry && dx * dx * ry * ry + dy * dy * rx * rx <= rx * rx * ry * ry;
      assert_eq!(
        p[0],
        if inside { 255 } else { 0 },
        "radii={rx},{ry}, delta={dx},{dy}"
      );
    }
  }
}

#[test]
fn mapped_effect_affine_commutes_with_logical_to_texture_coordinates() {
  for (sx, sy) in [(1.0, 1.0), (2.0, 5.0), (5.0, 2.0), (0.5, 0.25)] {
    let mapping = EffectRasterScale { x: sx, y: sy };
    for (kx, ky) in [(0.0, 0.0), (0.25, -0.5), (-0.5, 0.25)] {
      let logical = ImageEffectTransform {
        scale_x: 0.5,
        scale_y: -1.0,
        skew_x: kx,
        skew_y: ky,
        shift_x_px: 3.0,
        shift_y_px: -2.0,
      };
      let raster = mapping.transform(logical);
      for (x, y) in [(-3.0, 4.0), (0.0, 0.0), (11.0, -9.0)] {
        let expected = (
          sx * (logical.scale_x * x + logical.skew_x * y + logical.shift_x_px),
          sy * (logical.skew_y * x + logical.scale_y * y + logical.shift_y_px),
        );
        let actual = (
          raster.scale_x * (sx * x) + raster.skew_x * (sy * y) + raster.shift_x_px,
          raster.skew_y * (sx * x) + raster.scale_y * (sy * y) + raster.shift_y_px,
        );
        assert!((actual.0 - expected.0).abs() < 0.00001);
        assert!((actual.1 - expected.1).abs() < 0.00001);
      }
    }
  }
}

#[test]
fn mapped_reflection_retains_logical_fade_direction() {
  let bounds = PixelBounds {
    left: 0.0,
    top: 0.0,
    right: 10.0,
    bottom: 10.0,
  };
  for (sx, sy) in [(1.0, 1.0), (2.0, 5.0), (5.0, 2.0)] {
    let mapping = EffectRasterScale { x: sx, y: sy };
    let source = image::RgbaImage::from_pixel(
      (10.0 * sx) as u32,
      (10.0 * sy) as u32,
      image::Rgba([80, 120, 160, 255]),
    );
    for angle in [0.0_f32, 33.0, 90.0, 147.0] {
      for flip_y in [1.0, -1.0] {
        let effect = ImageReflectionEffect {
          source_is_transformed: false,
          blur_kernel: ReflectionBlurKernel::RadialGaussian,
          blur_radius_px: 0.0,
          raster_length_scale: 1.0,
          bounds_radius_scale: 1.0,
          bounds_radius_offset_px: 0.0,
          start_opacity: 1.0,
          start_position: 0.0,
          end_opacity: 0.0,
          end_position: 1.0,
          fade_direction_degrees: angle,
          distance_px: 0.0,
          distance_length_scale: 1.0,
          distance_mode: ReflectionDistanceMode::PostTransformOffset,
          reference: ReflectionReference::WordRunMetrics {
            ascent_px: Some(10.0),
            ramp_extension_px: 0.0,
          },
          direction_degrees: 0.0,
          transform: ImageEffectTransform {
            scale_x: 1.0,
            scale_y: flip_y,
            skew_x: 0.0,
            skew_y: 0.0,
            shift_x_px: 0.0,
            shift_y_px: 0.0,
          },
          alignment: (0.5, 0.5),
          rotate_with_shape: false,
        };
        let result = reflection_image_with_scale(&source, effect, bounds, bounds, bounds, mapping);
        let dx = angle.to_radians().cos();
        let dy = angle.to_radians().sin();
        let minimum = (10.0 * dx).min(0.0) + (10.0 * dy).min(0.0);
        let maximum = (10.0 * dx).max(0.0) + (10.0 * dy).max(0.0);
        for (x, y, pixel) in result.enumerate_pixels() {
          let distance = dx * ((x as f32 + 0.5) / sx) + dy * ((y as f32 + 0.5) / sy);
          let opacity = 1.0 - ((distance - minimum) / (maximum - minimum)).clamp(0.0, 1.0);
          assert_eq!(
            pixel[3],
            (255.0 * opacity).round() as u8,
            "scale={sx},{sy}, angle={angle}, flip={flip_y}, pixel={x},{y}"
          );
        }
      }
    }
  }
}

#[test]
fn mapped_shadow_uses_independent_gaussian_support_axes() {
  let mut source = image::RgbaImage::new(31, 31);
  source.put_pixel(15, 15, image::Rgba([0, 0, 0, 255]));
  for (sx, sy) in [(2.0, 5.0), (5.0, 2.0)] {
    for blur_kernel in [
      ShadowBlurKernel::Direct2dGaussian,
      ShadowBlurKernel::WordTextBalanced {
        prescale_divisor: 1,
      },
    ] {
      let output = outer_shadow_image_with_scale(
        &source,
        OuterShadowOptions {
          blur_radius_px: 1.0,
          blur_kernel,
          distance_px: 0.0,
          direction_degrees: 0.0,
          distance_mode: ShadowDistanceMode::PostTransformOffset,
          transform: ImageEffectTransform {
            scale_x: 1.0,
            scale_y: 1.0,
            skew_x: 0.0,
            skew_y: 0.0,
            shift_x_px: 0.0,
            shift_y_px: 0.0,
          },
          alignment: (0.0, 0.0),
          color: ResolvedEffectColor {
            color: RgbColor {
              r: 10,
              g: 20,
              b: 30,
            },
            alpha: 255,
          },
          anchor_bounds: PixelBounds {
            left: 0.0,
            top: 0.0,
            right: 31.0,
            bottom: 31.0,
          },
        },
        EffectRasterScale { x: sx, y: sy },
      );
      for (x, y, pixel) in output.enumerate_pixels() {
        if x.abs_diff(15) > sx as u32 || y.abs_diff(15) > sy as u32 {
          assert_eq!(pixel[3], 0);
        }
      }
      assert!(output.get_pixel(15, 15)[3] > 0);
      if sx > sy {
        assert!(output.get_pixel(18, 15)[3] > 0);
        assert_eq!(output.get_pixel(15, 18)[3], 0);
      } else {
        assert_eq!(output.get_pixel(18, 15)[3], 0);
        assert!(output.get_pixel(15, 18)[3] > 0);
      }
    }
  }
}
