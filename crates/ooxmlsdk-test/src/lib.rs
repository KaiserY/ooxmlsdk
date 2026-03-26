pub mod fixtures;

#[track_caller]
pub fn assert_stable_roundtrip<T>(xml: &str) -> (T, String, T)
where
  T: std::fmt::Display + std::str::FromStr,
  T::Err: std::fmt::Debug,
{
  let parsed = xml.parse::<T>().unwrap();
  let serialized_once = parsed.to_string();
  let reparsed = serialized_once.parse::<T>().unwrap();
  let serialized_twice = reparsed.to_string();

  assert_eq!(serialized_once, serialized_twice);

  (parsed, serialized_once, reparsed)
}
