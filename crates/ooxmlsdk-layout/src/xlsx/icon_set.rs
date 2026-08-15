use std::collections::HashMap;
use std::sync::OnceLock;

use bytes::Bytes;
use tiny_skia::{FillRule, LineCap, LineJoin, Paint, Path, PathBuilder, Pixmap, Stroke, Transform};

use super::sheet_conditions::IconSetType;

mod office_icon_assets;

// [MS-XLS] CFMultistate is the authority for the 17 standard set identities
// and their 3/4/5 state counts; [MS-OE376] corrects 5Rating to cell-phone-like
// bars. LibreOffice colorscale.cxx::aBitmapMap supplies the x14 stars,
// triangles, and boxes mapping. Keep these as cached raster icons because
// Office fixed output emits the same cell visuals as transparent image
// XObjects rather than arbitrary worksheet shape paths.
const ICON_PIXELS: u32 = 24;
const GREEN: [u8; 4] = [99, 190, 123, 255];
const YELLOW: [u8; 4] = [255, 192, 80, 255];
const RED: [u8; 4] = [248, 105, 107, 255];
const GRAY: [u8; 4] = [132, 132, 132, 255];
const DARK: [u8; 4] = [55, 55, 55, 255];
const LIGHT: [u8; 4] = [225, 225, 225, 255];

type IconPngCache = HashMap<(IconSetType, usize), Bytes>;

pub(super) fn icon_png(icon_set: IconSetType, icon_index: usize) -> Option<Bytes> {
  static ICONS: OnceLock<IconPngCache> = OnceLock::new();
  ICONS
    .get_or_init(|| {
      let mut icons = HashMap::new();
      for icon_set in [
        IconSetType::ThreeArrows,
        IconSetType::ThreeArrowsGray,
        IconSetType::ThreeFlags,
        IconSetType::ThreeTrafficLights1,
        IconSetType::ThreeTrafficLights2,
        IconSetType::ThreeSigns,
        IconSetType::ThreeSymbols,
        IconSetType::ThreeSymbols2,
        IconSetType::FourArrows,
        IconSetType::FourArrowsGray,
        IconSetType::FourRedToBlack,
        IconSetType::FourRating,
        IconSetType::FourTrafficLights,
        IconSetType::FiveArrows,
        IconSetType::FiveArrowsGray,
        IconSetType::FiveRating,
        IconSetType::FiveQuarters,
        IconSetType::ThreeStars,
        IconSetType::ThreeTriangles,
        IconSetType::FiveBoxes,
      ] {
        for icon_index in 0..icon_set.icon_count() {
          if let Some(bytes) = render_icon(icon_set, icon_index) {
            icons.insert((icon_set, icon_index), Bytes::from(bytes));
          }
        }
      }
      icons
    })
    .get(&(icon_set, icon_index))
    .cloned()
}

fn render_icon(icon_set: IconSetType, icon_index: usize) -> Option<Vec<u8>> {
  if let Some(encoded) = office_icon_assets::office_icon_base64(icon_set, icon_index) {
    return decode_base64(encoded).and_then(flip_office_pdf_icon);
  }
  if let Some(encoded) = office_fixed_icon_base64(icon_set, icon_index) {
    return decode_base64(encoded);
  }
  let mut pixmap = Pixmap::new(ICON_PIXELS, ICON_PIXELS)?;
  match icon_set {
    IconSetType::ThreeArrows
    | IconSetType::FourArrows
    | IconSetType::FiveArrows
    | IconSetType::ThreeArrowsGray
    | IconSetType::FourArrowsGray
    | IconSetType::FiveArrowsGray => draw_arrow(&mut pixmap, icon_set, icon_index)?,
    IconSetType::ThreeFlags => draw_flag(&mut pixmap, icon_index)?,
    IconSetType::ThreeTrafficLights1 | IconSetType::ThreeTrafficLights2 => {
      draw_traffic_light(&mut pixmap, icon_set, icon_index)?
    }
    IconSetType::FourTrafficLights => draw_four_traffic_light(&mut pixmap, icon_index)?,
    IconSetType::ThreeSigns => draw_sign(&mut pixmap, icon_index)?,
    IconSetType::ThreeSymbols | IconSetType::ThreeSymbols2 => {
      draw_symbol(&mut pixmap, icon_set, icon_index)?
    }
    IconSetType::FourRedToBlack => draw_red_to_black(&mut pixmap, icon_index)?,
    IconSetType::FourRating | IconSetType::FiveRating => {
      draw_rating(&mut pixmap, icon_set.icon_count(), icon_index)?
    }
    IconSetType::FiveQuarters => draw_quarter(&mut pixmap, icon_index)?,
    IconSetType::ThreeStars => draw_star(&mut pixmap, icon_index)?,
    IconSetType::ThreeTriangles => draw_triangle(&mut pixmap, icon_index)?,
    IconSetType::FiveBoxes => draw_box(&mut pixmap, icon_index)?,
    IconSetType::NoIcons => return None,
  }
  pixmap.encode_png().ok()
}

fn office_fixed_icon_base64(icon_set: IconSetType, icon_index: usize) -> Option<&'static str> {
  // These are the decoded 24x24 image/smask pairs emitted by Microsoft Office
  // fixed output for the source-backed tdf162948 and complex_icon_set fixtures.
  // The surrounding evaluator remains generic; only the producer-defined icon
  // artwork is fixed, just like PowerPoint's missing-picture bitmap.
  match (icon_set, icon_index) {
    (IconSetType::ThreeTriangles, 0) => Some(
      "iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAMAAADXqc3KAAAAjVBMVEX///+TNBqaNhswEQihOR05FAqjOh06FApDFwtNGw1XHg9XHw8AAABhIhEBAABpJRIDAQByKBRyKBQGAgF6KxUJAwGALRaBLRYNBAKHLxeHMBgRBgPITSzSUzHNUS/WVTLOUS/QUi+mPB/RUzCoPSDTUzGrPiHUVDGtQCLVVDGwQSPVVTKzQyS3RSa6RydApBRYAAAAHHRSTlMA7vlP/F3+Xm19jY4BnQOrBri5CsUP0NEV2tsc7/77bQAAAIpJREFUKM/FkUcOgDAMBOm99x56h/8/DyKEEiB35jgrxfaGon6FJnIGTJZ/yJgzYLkCvCg5Fr7FC9XTV4J4TZHqBvdNLd3z5bZDvmtltJnSD7cfehVbWdPH6fLTqGv4MYY5L9Avs2k8z7TsFQarY70LcL0NgM1zv9X4wb6HPqm0KE4icp1p+u93Ig7nbBJrjuc0tgAAAABJRU5ErkJggg==",
    ),
    (IconSetType::ThreeStars, 0) => Some(
      "iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAQAAABKfvVzAAABwElEQVQ4y5WUvS9DURjGn9Ne3GAoaUrIlWhvfdStSJtIGHwNpkorQUJIWFSkDNI0afwPiEEsGFgsXcSMSASJmIlRiahoWTB4DIpevZLrPct73vf5nY/nJEcQ/wuLUVGIob2hPSGMepJRscLv7wIOfDgzuYMrqEGDGjJ9JDWkwWseKFOdWg0UOLUy1RTgCjZBQKAJrqApQA1pAAAv1H4TQLHD2f55EhecbcWO331BDB9VteWWWjCWzbZwrhNfH253SABYhwCqDRwZxeh3nsQukhYAhJCap8I3m7znX5HiJsM3zVNCIkAQhFTSOj+bTvA5T/zMBGfTrfMFpZ/KLEAQsr1zMfZ6oZNfMPbauSjbf1Q5AAHLZPJOB9wxfCusuRqdrUpXQ5XeRwfqK5WeP9/BM+bLZimkspkPjSO6NX42s8qRTIYkecX4W/ztiiT5xJmMVTa8g3tghSR5ysiD0qN0Rx5OSJKrdA8YAn2JY75zhxOXNjdB2NwTlzt85yn7EgZAkW3uJc0NDu7L5d9Glw/ubzDDuZciWx7gGV/iAnvXLIU6Ewt71xe4TM94HlAbmH70R3PFX8MfnX6sDXzNxH+/mQ9XTT/rmh/mZAAAAABJRU5ErkJggg==",
    ),
    (IconSetType::FiveBoxes, 0) => Some(
      "iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAQAAABKfvVzAAAAqklEQVQ4y2NUDnCdySXGgAK+vri6QDOWVxpV9POTvdl3NzFmvLQV40OVYfjIcI7BmAFT9Mjr6WJMXBjKGRj4GRgYsIlyiTIwMDGQCEY1EKXh64tPGIIfGRgYsIl+e83AwHJ9KXcxNpPOYhG7sYyBgUUjClsiwJ40vkUxFDDxSI4mjcGg4dsr4pPGl2cMDCy70xlmcYmiSn1+cmPZt0QM0ad7sxgYGP+T6CQAVQA1ip9JWlMAAAAASUVORK5CYII=",
    ),
    (IconSetType::ThreeTriangles, 1) => Some(
      "iVBORw0KGgoAAAANSUhEUgAAABgAAAAYAgMAAACdGdVrAAAACVBMVEX///+kgCvqwoJkBmC+AAAAAXRSTlMAQObYZgAAABxJREFUCNdjYCATiIYCQQiD1CogWEIMBdVAJgAAzJgX39Wf9KsAAAAASUVORK5CYII=",
    ),
    (IconSetType::ThreeStars, 1) => Some(
      "iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAMAAADXqc3KAAABs1BMVEX///8AAAB6XyBVVVUhGggWFhZUQRY6OjqHaSNeXl4XEgYQEBBJORMzMzN9YSBXV1cOCwOmgi95eXkJCQl/YyFWVlZ+YiECAgBCNBGTcyZnZ2cuLi4BAQEHBQFPPRSceipubm42NjYFBQUMCgNbRxijfy12dnY/Pz8ICAgBAQCTciZnZ2cBAQEqIQseHh5mTxpHR0cFBAGbeSmlgjB5eXltbW0DAwM2Kg6aeCkdHR1sbGwlJSVvVx2MbSQXEgYQEBBhYWFNTU0KCAKhfit7YCALCQMICAhVVVVycnIHBwc9MBBkThoDAwECAgJGRkYqKiowJgyZdyhKOhMzMzNqamohISGvijiIiIjFn1S0tLTbtG/h4eGrhzTpwYH+/v6AgIDAm07qwoL///+rq6vWr2nY2NjowID7+/ukgCu8l0mjo6NycnK3kkPdtXHk5OSZmZnguXbs7OzCnFDkvHrz8/Ourq7XsGrZ2dnfuHXp6emyjTyOjo7LpFvguHXr6+vAwMDju3nZsm3c3Nzy8vK2kULQqWHJycmYmJjPqGDEnlOzs7PKysq6lUefn5/x8fGzjj6SkpK654skAAAAVnRSTlMAAb+/NDODg9PTJCRzc8PEFv39FsbCxQRo5uZoBAt78PB7DBSP+fmPFALl5wNCRZ+fCPH6+vEIVO1B7VSu2iUl2q4Q+MASEsD4EGCdBgaeYEzvdHTvTAqBtgMAAAEASURBVCjPY2AgDBiZmBmxSrCEhbNilWCLiGTHKsERFc2JTZwrJjYunhuLBE9CYlIyLxYJvpTEpFR+THEBwbTEpHQhYYSISAYEZCYmJiVlZUOAKFBCLCc3EQqSoCAvH2SkuIRkZgGyRGGWlLQM2DBZOfmiYphESamCohLcHmUV1TKIRLmaugaymzS1KiASldo6KI7VrYIZVa2HIqFfAxSvrQNK1BsgixsaNSQmNhqbNCUlNZuaIUmYtyQmtlroWlq1JSW1WyNJ2HTEdtraMTDYO3TFdTsixJ2cG3pcXEEsN/fePg9PuIRXf5W3D4Tp61c9wR8uERAYhNAeHBLKgA8AAHezTrCG4oR7AAAAAElFTkSuQmCC",
    ),
    (IconSetType::FiveBoxes, 1) => Some(
      "iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAMAAADXqc3KAAAAXVBMVEX///8jIyNoaGhnZ2c8PDxlZWVlZWUiIiJnZ2doaGg6SFZAQEA3YIg3X4hmZmYoNkQ4X4g6R1VycnKlpaWzs7OkpKQ4YYozX4w6YYlfbHpFd6lKfbFFdqlFdqg5YYmoj4roAAAAEnRSTlMAUOnnh+TjTujqsJD9/eWH/K19vpP+AAAAZElEQVQoz2NgZBKCAmYWVhiTjZ2BgUNYBApEhRBMTgYGIRE4QGYOOQlmXB7kEhOHAglJITjgZmDgkZKGAhkxFB3i0nAgjmLHEJPghXtQVg7hQT4GBn5eWJAICHLCQoQVmEpwAQB3cSrx8Yw7LAAAAABJRU5ErkJggg==",
    ),
    (IconSetType::FiveBoxes, 2) => Some(
      "iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAMAAADXqc3KAAAAYFBMVEX///8jIyNoaGhnZ2c8PDxlZWVlZWUiIiJnZ2doaGg6SFY8SVc3YIg3X4g4X4goNkQ4X4c3T2cnNUJycnKlpaWzs7OkpKQ4YYozX4w5YYpRaIA6YYlFd6lKfbFFdqlFdqhK8FGBAAAAE3RSTlMAUOnnh+TjTejqsLL9/fyH/MiE2xgU9gAAAGJJREFUKM/V0DkSgCAQRNFWXHABF0RRRO5/SxO2QA/Ai37VRD0oSuGQqvbZtADdpLOLmBQQMkgzuwP5HNgB/aGc89I+9QCMt3EeFZMBygRpZndgYZWNAy0HJub/MC8h+YpfL6GuMQwLq1pnAAAAAElFTkSuQmCC",
    ),
    (IconSetType::ThreeTriangles, 2) => Some(
      "iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAMAAADXqc3KAAAAjVBMVEX///8FDAkECQcqXk0qXk0CBgUoWkkBBAMmVUUBAgIjT0EkUEEAAQEhSjwAAAAeQzcbPDEbPTIYNSwULiYVLyYSKCA3dGASKCEPIRs1cV0wbFhNiXVNinZKhnJopJBGg29DgGxno49AfWk+emZmoo4+e2c8eGRloY06dmJjn4tkoIxinopgnIhdmoYyb1vVmW7DAAAAG3RSTlMAHBXZ2g/PCsQGt7gDqgGcjI18a2xc/l1O/PmQC1uXAAAAiElEQVQoz73RywJCQACFYZRbrqmEcgYJhd7/8RqXajCLVv7ld5ZHENZNFPkubbYSz2XlclXkpataDMSaOnd9R0Ajhj5100rSbkgTy2TddrLe6ZI5NjO4txxj+X3/c68o8a0svI8fqgeYntVx8JNfY1LtnzsPwgazmjCgQ9RiURvR4cVt5f/+7Q2YQhWzEEC8FwAAAABJRU5ErkJggg==",
    ),
    (IconSetType::ThreeStars, 2) => Some(
      "iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAMAAADXqc3KAAABC1BMVEX///8AAAB6XyAhGgggGQhUQRaHaSMXEgZJORN9YSB+YiEOCwOmgi9/YyF8YSB+YiECAgBCNBGTcyYHBQFPPRSceioHBgIMCgNbRxijfy0BAQCTciaUcyYBAQAqIQssIgtmTxoFBAGbeSmlgjA2Kg6aeCkpIApvVx2MbSQXEgYKCAKhfit7YCALCQOhfiw9MBBkThoDAwFlTxowJgyZdyhKOhOvijivijnFn1TbtG+rhzTpwYHAm07qwoLWr2nXsGrowICkgCu8l0m3kkPdtXHguXbCnFDkvHrkvXvfuHWyjTzLpFvguHXju3nZsm3jvHq2kULQqWHPqWHPqGDEnlO6lUfjvHmzjj60jz952gHKAAAANnRSTlMAAb80M4PTJHPDxBb9xsLFBGjmC3vwDBSP+QLl5wNCRZ8I8fpU7UGu2iUQ+MAS+GCdBp5M73SVG99HAAAA9klEQVQoz4WOZ5fBQBSGB6tbotfVotfVS1whUaNblv//S4SQjN05vB/m3HmeM+9chN5HpVariELTbH0Qhbbd1hKFrtPRkbi+yzBdPUEYegA9A0EY+wCs6T83WwYAA4tZIZ9DKRyI4e4Xqyhs/Aj+ZMRfKym7gxvjeMw57NStzOlyT6YPPJu4XR75H6/Pz0qc9fu8+E6B4FwS89DX07Jh4VElRJ5EdCGy5VI8FlGcx+IrgDVNrwE28RgmEluAXTKcSu4B+AQm0j/MIZNFKJs5MLu0wnP51bFQvE7FwvG3lJNF+SR8V6SxUhXOZVnU6g3leaNeQ69yAVFEMPf2rGN5AAAAAElFTkSuQmCC",
    ),
    (IconSetType::FiveBoxes, 3) => Some(
      "iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAMAAADXqc3KAAAAVFBMVEX///8oNkQ3X4g4X4c5RlRlZWVlZWUiIiI3YIhoaGg4UGk8SVc4YIg4X4g3T2cnNUIzX4xycnJFd6lKfbFFdqk4YYqlpaWzs7OkpKRFdqhAY4Y6YYkF1N+BAAAAEHRSTlMAh/38rOTjTv3qy7L9/MiERkpYxQAAAF9JREFUKM/VkkkSgCAMBEfcFzSIisj//+klEjj4APrUVXNIzVRQKWLqpjVM1wPDbpnjvBxzjwDZCLmIKS6Qgj4vOFHkCZ+FGVhkEhJV+Y1EiwuUTCIFvQaSL1mj6g2/vB6YJrzLsQMiAAAAAElFTkSuQmCC",
    ),
    (IconSetType::FiveBoxes, 4) => Some(
      "iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAMAAADXqc3KAAAAM1BMVEX///8oNkQ3X4g3YIg3Tmc4X4g4X4cnNUI4UGk4UWs4YIg3T2czX4xFd6lKfbFFdqlFdqjxNjm1AAAADHRSTlMAh/39x/z8hMvP/cjZ2aflAAAAT0lEQVQoz2NgZOKBAmYWVhiTjZ2BgZmXDwr4eRBMJgYGHj44QGYOOQmEBwUQHhQAepCDBxvgZGDgGu5BwoQ1SFgZGBCphI0bzmQFphJcAABE7Bs97ZCvrgAAAABJRU5ErkJggg==",
    ),
    (IconSetType::ThreeArrowsGray, 0) => Some(
      "iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAMAAADXqc3KAAAAwFBMVEX///8KCgpVU1MJCQkJCAhSUFAICAgHBwdQTk4FBQVQTk5NTEwEBARKSUkDAwNIRkZHRkZUUlJSUVEKCgpVVFRVU1MLCwtXVVVWVFQNDQ1ZV1dYVlYMCwsODg5ZWFgODQ0QDw9bWVlbWVkPDw8RERFdW1tcWloTExNeXFwTEhIVFBReXFwVFRVkYmKAgIBycHBxcHBwb29vbm5ubW1tbGxzcnJ/f39sa2tycnJ0c3Nzc3N1dHR2dXV3dnZ3d3d4d3cLjuf+AAAALXRSTlMAHNoYF9IVFM0PzMYMvwq4t9fTG9vZHd/cIuPhHyXlJCnq6Sgt6+oy7zE28Df8p/d9AAAAvUlEQVQoz7XS1w6CQBAFUEBQkWJv2BUVRUYpgkj7/78ShKVJwpP3YZKdk2wys4thtVG+qQII8zfAy4AnQDSKQBLoEqqZhxaFLlTubToDuvNIAVSGRcAyKmQAGsfHwHMa5AH0bgw9HYoARgxGfpr+MzyYVtSyzLC8BgkMR3YkkJT3mESjT6YOpLFnaPAwcyEVZ7HMr2u1duO+K2yKu93uvKjv7Q/lrYtHH8A/ib/vcZaCQLpU/YarLN/q/wz2AfFXKD3Vb+0pAAAAAElFTkSuQmCC",
    ),
    (IconSetType::FourRedToBlack, 3) => Some(
      "iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAMAAADXqc3KAAABGlBMVEX///8LAwFCFwtuJxOFLxeWNRqaNhuLMRhuJxNBFwsKAwEFAQBRHA6XNhuWNhtPHA4EAQATBwOCLheALRYTBgMRBgOMMRiMMRh9LBZ/LRZKGg1OGw2VNRuWNRsJAwE+FgtsJhOJMBiXNRqGLxeKMBg/FgsLBAKXNRtMGw1PHA6DLheBLRYWBwOQMxmOMhmDLhcVBwMGAgFTHQ6YNhtMGw0NBAJFGAxxKBSNMhlwJxNEGAyxRinDVDbPXD7VYULXYkPQXT/DUzawRSjQXT7YY0TRXj+wRim9TzG/UDPAUTO+UDKwRSmyRyqyRynFVTfEVDbWYkPUYEHPXT7OXD2zSCrSXkDAUTS/UTO0SCvGVjfWYUPGVTezSCueOBxaFKTdAAAAO3RSTlMAEmyz1/P54bJqEQmD8/KACCDTzx8c4+LKznh/8PEQZa/e9NnfZxPxfIHV0STo5NQiCof1exVwt+W2b6C17ZIAAADiSURBVCjPY2AgABiZmFlY2dg5OLlQhLl5eK1tbO3sHRyt+fgFEOKCQk7OLlDg6iYsAhMXFXN3QQIe4qIQcQEJTxcU4CUJMU3K2gUNeEuDxLlkXNElXGXlgBKcPi4YwFceKMHhhynhrwCUEHfFlHBWBEqwBWBK2CkBJVjtMCUCQRLKQZgSwSpACWZcljOFYEr4qAIl1NRDMTzIC/Igg4YvuoS1JiQytLxQxd21oVGioxuGLB6upwOLEB19X7j3Q62FDBBRaGhkHBHpHOXiGh2jbsKNEuumZuYWsbEWllZqhJINAE8pYq8m5f9YAAAAAElFTkSuQmCC",
    ),
    (IconSetType::ThreeTrafficLights1, 1) => Some(
      "iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAMAAADXqc3KAAAA/1BMVEX///8KCAJBMxFxWB2PbyWdeimaeCiIaiNCNBEKCAIFBAFRPxWdeyqceikQDQSAZCESDgSQcCWRcSYSDgSFZyJLOxNMOxQJBwKaeSqbeSo8Lw89LxBtVRxvVh2QcCWeeymaeCiJaySKbCRuVh2ceipSQBVNPBSFaCOGaCMUEAWRcSaScicWEQWHaSNSQBWeeypTQRVENRFyWR6ffCmbeShzWR5FNhILCQO6lUfQqmLguXbnv3/mvn3et3S6lUbhunfqwoLguHW5lEXLpVzJo1nMpl27lUfiuni8l0nSq2TSrGXhuXfowH/ft3TfuHXTrGW7lkjMply5lEbRqmPmv36woUtRAAAAOHRSTlMAEGaw3/Xw1WgRCH/y8hrIHeHiHM92dw/u715fq63g9/HW2KzwgHjQ0SDj4yPSgfOCarL48rNsEn5bz7YAAADdSURBVCjPhZLnEoIwEISt2FDsWFHs2LuIqLEAdrG8/7MYR1Agju7P/Wbucrsxmf7IbLHa7JjD6nTpbLcHZ8fchJ+OWa/H/fF9xGwOFC2WhE/1/YEV0GgdDClzwjofAIF4TYvMgEFi9OmTsbkRSHESgsQGINokIUhtUbBLQ0BxKNhTENgnKDhkIMB4FPBZCOgjCk45CPJflp/Tv55L4hJyYMH8PL3IGoFceoVYFgwhVpRKmOpFF3utrhbChMX3Hu7aYDTVNnH5tj/cjze51dZUC+XqdKlenx4MR/++zQNy4lwUVIHQ3gAAAABJRU5ErkJggg==",
    ),
    (IconSetType::ThreeArrows, 1) => Some(
      "iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAMAAADXqc3KAAAAz1BMVEX///8SDgSLbCSKayQQDQQPDASIaiOMbSQTDwUMCgODZiKPbyUVEAUKCAJ/YyGRcSYYEwYHBgJ7YCCUdCcaFAYGBQF2XB+WdSgcFgeYdikfGAiaeSoiGgiaeCgjGwl1Wx56XyCUcycZFAaQcCUWEQWObiUTDwWHaSOKbCQRDQSkgCvFoFXqwoLKpFrDnVLMpl3Am07pwYHNp16+mEvPqGC8lkjowIDRqmO5lEXSrGXUrWfWr2nowH+7lkjOqGDAmk7Npl3CnVHKpFvFn1TIolhZdcOqAAAAKnRSTlMAHdnXGhjU2x4Uzd8hEMbiJgzA5ikKuOgs6zHuNfE3t7/lKOAj3R/T2BsC6x2VAAAAnklEQVQoz2NgoDZgZNICA2YWdBlWNm0dINBl50CX4eTSA8noc/Ogy/DyGRgCZYz4BdBlBIWMQTImwiLoMqJipmZAGXNxCTBXCxlYgGQsJaXAEjoYwEpaBruEjrUsiRJW0nIYltsgLEdyrrwtsnMRHlQAe9BOUQk9SOxB4g7KKuiB6Aiy10lVDS3Y1Z1B4i4ammgRpQFxlTor1dMAAwMAXMIm6No81cUAAAAASUVORK5CYII=",
    ),
    (IconSetType::ThreeArrows, 2) => Some(
      "iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAMAAADXqc3KAAAAt1BMVEX///8KFxMvaFUJFREvaFYJFBEIExAvZ1QuZlQIEw8IEQ4uZVMHEA0HDwwtZFIsYlEGDgssYVAFDQoFDAkrYE8qX04FCwkqXk0ECggqXUwpW0skUEElU0QBBAMnVkcCBQQCBQQoWUkCBgUECQcpW0oqXk0ECghYlYFXk39opJBWkn5VkX1UkHxTkHxTj3tSjnpRjXlQjXlno49QjHhPjHhCf2tOinYyb1tEgW1Gg29IhHBKhnJMiXVCZUkdAAAAJ3RSTlMANvAx7zAt6+osKegnJOXjIeEeHN7bG9gX19O4vwrHDQzNDxXS2hhh3PZrAAAAuElEQVQoz7XSRxKCQBRF0Q+iYkBUxEBSQEJjDkiQ/a9Lmgxaxcg36ME9o1/VAO0jSJL41TuUZVHd797r2wjZ9KDZh7SD4jmjcb0zkwNKdmSZap+yJ5TtPJuXnVtcULErz+V9yd/icEfZ81itM9g8cXIxuFheQgYe7j5+kedj8SoQiCmIQR1CSU5BlsIqvJUtpAA75V1CpGqQA2hqVICgQwmg73MwTKgCmEZ+ONQBitObAH+DZO1/5gPGaCbhsx3cSQAAAABJRU5ErkJggg==",
    ),
    _ => None,
  }
}

fn decode_base64(encoded: &str) -> Option<Vec<u8>> {
  if !encoded.len().is_multiple_of(4) {
    return None;
  }
  let mut output = Vec::with_capacity(encoded.len() / 4 * 3);
  for encoded_block in encoded.as_bytes().chunks_exact(4) {
    let mut block = [0u8; 4];
    let mut padding = 0usize;
    for (index, byte) in encoded_block.iter().copied().enumerate() {
      if byte == b'=' {
        block[index] = 0;
        padding += 1;
      } else {
        block[index] = match byte {
          b'A'..=b'Z' => byte - b'A',
          b'a'..=b'z' => byte - b'a' + 26,
          b'0'..=b'9' => byte - b'0' + 52,
          b'+' => 62,
          b'/' => 63,
          _ => return None,
        };
      }
    }
    output.push(block[0] << 2 | block[1] >> 4);
    if padding < 2 {
      output.push(block[1] << 4 | block[2] >> 2);
    }
    if padding == 0 {
      output.push(block[2] << 6 | block[3]);
    }
  }
  Some(output)
}

fn flip_office_pdf_icon(encoded_png: Vec<u8>) -> Option<Vec<u8>> {
  // pdfimages exposes the source scanlines. Office paints these image XObjects
  // with a negative Y scale, while our image primitive accepts normal top-down
  // PNGs. Normalize that PDF-only orientation once while the global icon cache
  // is initialized; this is never paid per cell or per document.
  let mut pixmap = Pixmap::decode_png(&encoded_png).ok()?;
  let row_len = pixmap.width() as usize * 4;
  for row in 0..pixmap.height() as usize / 2 {
    let other = pixmap.height() as usize - row - 1;
    let (head, tail) = pixmap.data_mut().split_at_mut(other * row_len);
    head[row * row_len..(row + 1) * row_len].swap_with_slice(&mut tail[..row_len]);
  }
  pixmap.encode_png().ok()
}

fn draw_arrow(pixmap: &mut Pixmap, icon_set: IconSetType, icon_index: usize) -> Option<()> {
  let count = icon_set.icon_count();
  let gray = matches!(
    icon_set,
    IconSetType::ThreeArrowsGray | IconSetType::FourArrowsGray | IconSetType::FiveArrowsGray
  );
  let angles: &[f32] = match count {
    3 => &[180.0, 90.0, 0.0],
    4 => &[180.0, 135.0, 45.0, 0.0],
    _ => &[180.0, 135.0, 90.0, 45.0, 0.0],
  };
  let colors: &[[u8; 4]] = match count {
    3 => &[RED, YELLOW, GREEN],
    4 => &[RED, [244, 142, 72, 255], YELLOW, GREEN],
    _ => &[RED, [244, 142, 72, 255], YELLOW, [158, 198, 96, 255], GREEN],
  };
  let color = if gray {
    let shade = 90u8.saturating_add((icon_index as u8).saturating_mul(35));
    [shade, shade, shade, 255]
  } else {
    *colors.get(icon_index)?
  };
  let path = arrow_path()?;
  let transform = Transform::from_rotate_at(
    *angles.get(icon_index)?,
    ICON_PIXELS as f32 / 2.0,
    ICON_PIXELS as f32 / 2.0,
  );
  fill_and_stroke(pixmap, &path, color, darker(color), 1.0, transform);
  Some(())
}

fn arrow_path() -> Option<Path> {
  let mut path = PathBuilder::new();
  path.move_to(12.0, 1.5);
  path.line_to(22.0, 10.5);
  path.line_to(17.0, 10.5);
  path.line_to(17.0, 22.0);
  path.line_to(7.0, 22.0);
  path.line_to(7.0, 10.5);
  path.line_to(2.0, 10.5);
  path.close();
  path.finish()
}

fn draw_flag(pixmap: &mut Pixmap, icon_index: usize) -> Option<()> {
  let color = *[RED, YELLOW, GREEN].get(icon_index)?;
  stroke_line(pixmap, 5.0, 2.0, 5.0, 22.0, DARK, 2.0);
  let mut path = PathBuilder::new();
  path.move_to(6.0, 3.0);
  path.line_to(20.0, 5.0);
  path.line_to(16.0, 12.0);
  path.line_to(6.0, 10.5);
  path.close();
  fill_and_stroke(
    pixmap,
    &path.finish()?,
    color,
    darker(color),
    1.0,
    Transform::identity(),
  );
  Some(())
}

fn draw_traffic_light(pixmap: &mut Pixmap, icon_set: IconSetType, icon_index: usize) -> Option<()> {
  let color = *[RED, YELLOW, GREEN].get(icon_index)?;
  if icon_set == IconSetType::ThreeTrafficLights2 {
    rounded_rect(pixmap, (3.0, 2.0, 18.0, 20.0), DARK, DARK, 1.0)?;
    circle(pixmap, 12.0, 12.0, 7.0, color, darker(color), 1.0)?;
  } else {
    circle(pixmap, 12.0, 12.0, 9.5, color, darker(color), 1.0)?;
    circle(
      pixmap,
      9.0,
      8.0,
      2.2,
      [255, 255, 255, 115],
      [255, 255, 255, 0],
      0.0,
    )?;
  }
  Some(())
}

fn draw_four_traffic_light(pixmap: &mut Pixmap, icon_index: usize) -> Option<()> {
  let color = *[GRAY, RED, YELLOW, GREEN].get(icon_index)?;
  circle(pixmap, 12.0, 12.0, 9.5, color, darker(color), 1.0)
}

fn draw_sign(pixmap: &mut Pixmap, icon_index: usize) -> Option<()> {
  let color = *[RED, YELLOW, GREEN].get(icon_index)?;
  let path = match icon_index {
    0 => polygon(&[(12.0, 2.0), (22.0, 12.0), (12.0, 22.0), (2.0, 12.0)])?,
    1 => polygon(&[(12.0, 2.0), (22.0, 21.0), (2.0, 21.0)])?,
    _ => {
      return circle(pixmap, 12.0, 12.0, 9.5, color, darker(color), 1.0);
    }
  };
  fill_and_stroke(
    pixmap,
    &path,
    color,
    darker(color),
    1.0,
    Transform::identity(),
  );
  Some(())
}

fn draw_symbol(pixmap: &mut Pixmap, icon_set: IconSetType, icon_index: usize) -> Option<()> {
  let color = if icon_set == IconSetType::ThreeSymbols2 {
    *[RED, YELLOW, GREEN].get(icon_index)?
  } else {
    *[[202, 66, 64, 255], [255, 192, 80, 255], [76, 160, 95, 255]].get(icon_index)?
  };
  circle(pixmap, 12.0, 12.0, 9.5, color, darker(color), 1.0)?;
  match icon_index {
    0 => {
      stroke_line(pixmap, 7.5, 7.5, 16.5, 16.5, [255, 255, 255, 255], 2.2);
      stroke_line(pixmap, 16.5, 7.5, 7.5, 16.5, [255, 255, 255, 255], 2.2);
    }
    1 => {
      stroke_line(pixmap, 12.0, 6.0, 12.0, 13.5, [255, 255, 255, 255], 2.2);
      circle(
        pixmap,
        12.0,
        17.0,
        1.2,
        [255, 255, 255, 255],
        [255, 255, 255, 255],
        0.0,
      )?;
    }
    _ => {
      let path = polygon(&[(6.5, 12.0), (10.2, 16.0), (17.8, 7.5)])?;
      stroke_path(pixmap, &path, [255, 255, 255, 255], 2.2);
    }
  }
  Some(())
}

fn draw_red_to_black(pixmap: &mut Pixmap, icon_index: usize) -> Option<()> {
  let color = *[
    [55, 55, 55, 255],
    [125, 125, 125, 255],
    [228, 126, 126, 255],
    [181, 52, 52, 255],
  ]
  .get(icon_index)?;
  circle(pixmap, 12.0, 12.0, 9.5, color, darker(color), 1.0)
}

fn draw_rating(pixmap: &mut Pixmap, count: usize, icon_index: usize) -> Option<()> {
  let active = icon_index + 1;
  for bar in 0..count {
    let width = 3.0;
    let gap = 1.0;
    let total = count as f32 * width + (count - 1) as f32 * gap;
    let left = (24.0 - total) / 2.0 + bar as f32 * (width + gap);
    let height = 4.0 + bar as f32 * (14.0 / count as f32);
    let color = if bar < active { GREEN } else { LIGHT };
    rect(
      pixmap,
      (left, 21.0 - height, width, height),
      color,
      darker(color),
      0.6,
    )?;
  }
  Some(())
}

fn draw_quarter(pixmap: &mut Pixmap, icon_index: usize) -> Option<()> {
  circle(pixmap, 12.0, 12.0, 9.5, LIGHT, GRAY, 1.0)?;
  let fractions = [0.0, 0.25, 0.5, 0.75, 1.0];
  let fraction = *fractions.get(icon_index)?;
  if fraction <= f32::EPSILON {
    return Some(());
  }
  let steps = (fraction * 32.0).round() as usize;
  let mut points = vec![(12.0, 12.0)];
  for step in 0..=steps {
    let angle = -std::f32::consts::FRAC_PI_2 + std::f32::consts::TAU * step as f32 / 32.0;
    points.push((12.0 + angle.cos() * 9.0, 12.0 + angle.sin() * 9.0));
  }
  let path = polygon(&points)?;
  fill_path(pixmap, &path, GREEN, Transform::identity());
  circle(pixmap, 12.0, 12.0, 9.5, [0, 0, 0, 0], GRAY, 1.0)
}

fn draw_star(pixmap: &mut Pixmap, icon_index: usize) -> Option<()> {
  let mut points = Vec::with_capacity(10);
  for point in 0..10 {
    let angle = -std::f32::consts::FRAC_PI_2 + point as f32 * std::f32::consts::PI / 5.0;
    let radius = if point % 2 == 0 { 10.0 } else { 4.5 };
    points.push((12.0 + angle.cos() * radius, 12.0 + angle.sin() * radius));
  }
  let path = polygon(&points)?;
  let fill = match icon_index {
    0 => LIGHT,
    1 => YELLOW,
    _ => GREEN,
  };
  fill_and_stroke(
    pixmap,
    &path,
    fill,
    darker(fill),
    1.0,
    Transform::identity(),
  );
  if icon_index == 1 {
    rect(pixmap, (12.0, 1.0, 11.0, 22.0), LIGHT, LIGHT, 0.0)?;
    stroke_path(pixmap, &path, darker(fill), 1.0);
  }
  Some(())
}

fn draw_triangle(pixmap: &mut Pixmap, icon_index: usize) -> Option<()> {
  let (points, color) = match icon_index {
    0 => ([(3.0, 6.0), (21.0, 6.0), (12.0, 21.0)], RED),
    1 => ([(4.0, 4.0), (21.0, 12.0), (4.0, 20.0)], YELLOW),
    _ => ([(12.0, 3.0), (21.0, 20.0), (3.0, 20.0)], GREEN),
  };
  let path = polygon(&points)?;
  fill_and_stroke(
    pixmap,
    &path,
    color,
    darker(color),
    1.0,
    Transform::identity(),
  );
  Some(())
}

fn draw_box(pixmap: &mut Pixmap, icon_index: usize) -> Option<()> {
  rect(pixmap, (3.0, 3.0, 18.0, 18.0), LIGHT, GRAY, 1.0)?;
  let fractions = [0.0, 0.25, 0.5, 0.75, 1.0];
  let fraction = *fractions.get(icon_index)?;
  if fraction > f32::EPSILON {
    let height = 16.0 * fraction;
    rect(
      pixmap,
      (4.0, 20.0 - height, 16.0, height),
      GREEN,
      GREEN,
      0.0,
    )?;
  }
  Some(())
}

fn circle(
  pixmap: &mut Pixmap,
  x: f32,
  y: f32,
  radius: f32,
  fill: [u8; 4],
  stroke: [u8; 4],
  stroke_width: f32,
) -> Option<()> {
  let mut path = PathBuilder::new();
  path.push_circle(x, y, radius);
  let path = path.finish()?;
  fill_and_stroke(
    pixmap,
    &path,
    fill,
    stroke,
    stroke_width,
    Transform::identity(),
  );
  Some(())
}

fn rect(
  pixmap: &mut Pixmap,
  bounds: (f32, f32, f32, f32),
  fill: [u8; 4],
  stroke: [u8; 4],
  stroke_width: f32,
) -> Option<()> {
  let (x, y, width, height) = bounds;
  let path = polygon(&[
    (x, y),
    (x + width, y),
    (x + width, y + height),
    (x, y + height),
  ])?;
  fill_and_stroke(
    pixmap,
    &path,
    fill,
    stroke,
    stroke_width,
    Transform::identity(),
  );
  Some(())
}

fn rounded_rect(
  pixmap: &mut Pixmap,
  bounds: (f32, f32, f32, f32),
  fill: [u8; 4],
  stroke: [u8; 4],
  stroke_width: f32,
) -> Option<()> {
  let (x, y, width, height) = bounds;
  let rect = tiny_skia::Rect::from_xywh(x, y, width, height)?;
  let path = PathBuilder::from_rect(rect);
  fill_and_stroke(
    pixmap,
    &path,
    fill,
    stroke,
    stroke_width,
    Transform::identity(),
  );
  Some(())
}

fn polygon(points: &[(f32, f32)]) -> Option<Path> {
  let first = points.first()?;
  let mut path = PathBuilder::new();
  path.move_to(first.0, first.1);
  for point in &points[1..] {
    path.line_to(point.0, point.1);
  }
  path.close();
  path.finish()
}

fn fill_and_stroke(
  pixmap: &mut Pixmap,
  path: &Path,
  fill: [u8; 4],
  stroke: [u8; 4],
  stroke_width: f32,
  transform: Transform,
) {
  fill_path(pixmap, path, fill, transform);
  if stroke_width > f32::EPSILON && stroke[3] > 0 {
    stroke_path_with_transform(pixmap, path, stroke, stroke_width, transform);
  }
}

fn fill_path(pixmap: &mut Pixmap, path: &Path, color: [u8; 4], transform: Transform) {
  if color[3] == 0 {
    return;
  }
  let mut paint = Paint {
    anti_alias: true,
    ..Paint::default()
  };
  paint.set_color_rgba8(color[0], color[1], color[2], color[3]);
  pixmap.fill_path(path, &paint, FillRule::Winding, transform, None);
}

fn stroke_path(pixmap: &mut Pixmap, path: &Path, color: [u8; 4], width: f32) {
  stroke_path_with_transform(pixmap, path, color, width, Transform::identity());
}

fn stroke_path_with_transform(
  pixmap: &mut Pixmap,
  path: &Path,
  color: [u8; 4],
  width: f32,
  transform: Transform,
) {
  let mut paint = Paint {
    anti_alias: true,
    ..Paint::default()
  };
  paint.set_color_rgba8(color[0], color[1], color[2], color[3]);
  let stroke = Stroke {
    width,
    line_cap: LineCap::Round,
    line_join: LineJoin::Round,
    ..Stroke::default()
  };
  pixmap.stroke_path(path, &paint, &stroke, transform, None);
}

fn stroke_line(
  pixmap: &mut Pixmap,
  x1: f32,
  y1: f32,
  x2: f32,
  y2: f32,
  color: [u8; 4],
  width: f32,
) {
  let mut path = PathBuilder::new();
  path.move_to(x1, y1);
  path.line_to(x2, y2);
  if let Some(path) = path.finish() {
    stroke_path(pixmap, &path, color, width);
  }
}

fn darker(color: [u8; 4]) -> [u8; 4] {
  [
    (u16::from(color[0]) * 55 / 100) as u8,
    (u16::from(color[1]) * 55 / 100) as u8,
    (u16::from(color[2]) * 55 / 100) as u8,
    color[3],
  ]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn every_standard_icon_has_a_cached_png() {
    for icon_set in [
      IconSetType::ThreeArrows,
      IconSetType::ThreeFlags,
      IconSetType::ThreeTrafficLights1,
      IconSetType::ThreeSigns,
      IconSetType::ThreeSymbols,
      IconSetType::FourArrows,
      IconSetType::FourRedToBlack,
      IconSetType::FourRating,
      IconSetType::FiveArrows,
      IconSetType::FiveRating,
      IconSetType::FiveQuarters,
      IconSetType::ThreeStars,
      IconSetType::ThreeTriangles,
      IconSetType::FiveBoxes,
    ] {
      for index in 0..icon_set.icon_count() {
        let bytes = icon_png(icon_set, index).expect("icon PNG");
        assert!(bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
      }
    }
    assert!(icon_png(IconSetType::NoIcons, 0).is_none());
  }
}
