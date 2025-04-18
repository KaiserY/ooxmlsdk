#![recursion_limit = "768"]

pub mod common {
  include!(concat!(env!("OUT_DIR"), "/common/mod.rs"));
}

pub mod schemas {
  include!(concat!(env!("OUT_DIR"), "/schemas/mod.rs"));
}

pub mod deserializers {
  include!(concat!(env!("OUT_DIR"), "/deserializers/mod.rs"));
}

pub mod serializers {
  include!(concat!(env!("OUT_DIR"), "/serializers/mod.rs"));
}

#[cfg(feature = "parts")]
pub mod parts {
  include!(concat!(env!("OUT_DIR"), "/parts/mod.rs"));
}

#[cfg(feature = "validators")]
pub mod validators {
  include!(concat!(env!("OUT_DIR"), "/validators/mod.rs"));
}
