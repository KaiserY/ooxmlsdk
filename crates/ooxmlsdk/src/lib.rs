#![recursion_limit = "768"]

pub mod common {
  include!(concat!(env!("OUT_DIR"), "/common/mod.rs"));
}

#[cfg(feature = "deserializers")]
pub mod deserializers {
  include!(concat!(env!("OUT_DIR"), "/deserializers/mod.rs"));
}

#[cfg(feature = "parts")]
pub mod parts {
  include!(concat!(env!("OUT_DIR"), "/parts/mod.rs"));
}

#[cfg(feature = "schemas")]
pub mod schemas {
  include!(concat!(env!("OUT_DIR"), "/schemas/mod.rs"));
}

#[cfg(feature = "serializers")]
pub mod serializers {
  include!(concat!(env!("OUT_DIR"), "/serializers/mod.rs"));
}

#[cfg(feature = "validators")]
pub mod validators {
  include!(concat!(env!("OUT_DIR"), "/validators/mod.rs"));
}

#[cfg(feature = "packages")]
pub mod packages {
  include!(concat!(env!("OUT_DIR"), "/packages/mod.rs"));
}
