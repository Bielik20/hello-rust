use super::assignment::Assignment;
use super::price::Price;
use std::iter;
use std::ops;

#[derive(Debug)]
pub struct Report {
  extra_commission: f32,
  price: Price,
}

impl Report {
  pub fn noop() -> Report {
    return Report {
      extra_commission: 0.0,
      price: Price::noop(),
    };
  }

  pub fn create_from_many(assignments: Vec<Assignment>, commission_rate: f32) -> Report {
    return assignments
      .iter()
      .map(|assignment| Report::create_from_one(&assignment, commission_rate))
      .sum();
  }

  pub fn create_from_one(assignment: &Assignment, commission_rate: f32) -> Report {
    let net = assignment.gross_price / (1.0 + assignment.tax);
    let price = Price {
      net: net,
      gross: assignment.gross_price,
      commission: net * commission_rate,
    };

    return Report {
      extra_commission: assignment.extra_commission,
      price: price,
    };
  }
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
