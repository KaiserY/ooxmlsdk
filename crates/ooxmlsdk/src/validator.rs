pub trait SdkValidator {
  fn validate(&self) -> Result<(), crate::common::SdkError> {
    Ok(())
  }

  fn is_valid(&self) -> bool {
    self.validate().is_ok()
  }
}
