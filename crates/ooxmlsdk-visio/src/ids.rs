/// Separate counters for pages, masters, and shapes. Each sequence starts at 1.
#[derive(Clone, Debug)]
pub struct Ids {
  next_page: u32,
  next_master: u32,
  next_shape: u32,
}

impl Default for Ids {
  fn default() -> Self {
    Self::new()
  }
}

impl Ids {
  pub fn new() -> Self {
    Self {
      next_page: 1,
      next_master: 1,
      next_shape: 1,
    }
  }

  pub fn page(&mut self) -> u32 {
    let id = self.next_page;
    self.next_page += 1;
    id
  }

  pub fn master(&mut self) -> u32 {
    let id = self.next_master;
    self.next_master += 1;
    id
  }

  pub fn shape(&mut self) -> u32 {
    let id = self.next_shape;
    self.next_shape += 1;
    id
  }
}
