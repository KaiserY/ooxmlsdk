pub fn escape_snake_case(name: String) -> String {
  match name.as_str() {
    "if" | "else" | "ref" | "type" | "macro" | "loop" | "mod" | "override" | "for" | "in"
    | "box" | "final" | "break" => {
      format!("r#{name}")
    }
    _ => name,
  }
}

pub fn escape_upper_camel_case(name: String) -> String {
  match name.as_str() {
    "self" | "Self" => {
      format!("_{name}")
    }
    _ => name,
  }
}

macro_rules! get_or_panic {
  ($map:expr, $key:expr) => {
    $map.get($key).ok_or_else(|| format!("{:?}", $key)).unwrap()
  };
}

pub(crate) use get_or_panic;
