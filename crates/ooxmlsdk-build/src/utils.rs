pub fn escape_snake_case(name: String) -> String {
  match name.as_str() {
    "if" | "else" | "ref" | "type" | "macro" | "loop" | "mod" | "override" | "for" | "in"
    | "box" | "final" | "break" => {
      format!("r#{}", name)
    }
    _ => name,
  }
}

pub fn escape_upper_camel_case(name: String) -> String {
  match name.as_str() {
    "self" | "Self" => {
      format!("_{}", name)
    }
    _ => name,
  }
}
