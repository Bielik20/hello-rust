use std::ops;

#[derive(Debug)]
pub struct Price {
  pub net: f32,
  pub gross: f32,
  pub commission: f32,
}

impl Price {
  pub fn noop() -> Price {
    return Price {
      net: 0.0,
      gross: 0.0,
      commission: 0.0,
    };
  }
}

impl ops::Add for Price {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    return Price {
      net: self.net + other.net,
      gross: self.gross + other.gross,
      commission: self.commission + other.commission,
    };
  }
}
