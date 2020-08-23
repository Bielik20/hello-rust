use super::price::Price;
use std::iter;
use std::ops;

#[derive(Debug)]
pub struct Report {
  pub extra_commission: f32,
  pub price: Price,
}

impl Report {
  pub fn noop() -> Report {
    return Report {
      extra_commission: 0.0,
      price: Price::noop(),
    };
  }

  pub fn create(items: Vec<&impl MakeReport>, commission_rate: f32) -> Report {
    return items
      .iter()
      .map(|item| item.make_report(commission_rate))
      .sum();
  }
}

pub trait MakeReport {
  fn make_report(&self, commission_rate: f32) -> Report;
}

impl ops::Add for Report {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    return Report {
      extra_commission: self.extra_commission + other.extra_commission,
      price: self.price + other.price,
    };
  }
}

impl iter::Sum for Report {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = Report>,
  {
    return iter.fold(Report::noop(), ops::Add::add);
  }
}
