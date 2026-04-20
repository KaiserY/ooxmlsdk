pub mod fixtures;

#[track_caller]
pub fn assert_roundtrip<T>(xml: &str) -> (T, String, T)
where
  T: std::fmt::Display + std::str::FromStr,
  T::Err: std::fmt::Debug,
{
  let parsed = xml.parse::<T>().unwrap();
  let serialized_once = parsed.to_string();
  let reparsed = serialized_once.parse::<T>().unwrap();

  (parsed, serialized_once, reparsed)
}

#[track_caller]
pub fn assert_stable_roundtrip<T>(xml: &str) -> (T, String, T)
where
  T: std::fmt::Display + std::str::FromStr,
  T::Err: std::fmt::Debug,
{
  let (parsed, serialized_once, reparsed) = assert_roundtrip::<T>(xml);
  let serialized_twice = reparsed.to_string();

  assert_eq!(serialized_once, serialized_twice);

  (parsed, serialized_once, reparsed)
}

#[track_caller]
pub fn trim_xml_declaration(xml: &str) -> &str {
  const XML_DECL_END: &str = "?>";

  let xml = xml.trim();

  if let Some(stripped) = xml.strip_prefix("<?xml") {
    let end = stripped
      .find(XML_DECL_END)
      .expect("unterminated xml declaration");
    stripped[end + XML_DECL_END.len()..].trim()
  } else {
    xml
  }
}
