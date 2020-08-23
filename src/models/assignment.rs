use super::report::MakeReport;
use super::report::Report;
use super::price::Price;

pub struct Assignment {
  pub extra_commission: f32,
  pub tax: f32,
  pub gross_price: f32
}

impl MakeReport for Assignment {
  fn make_report(&self, commission_rate: f32) -> Report {
    let net = self.gross_price / (1.0 + self.tax);
    let price = Price {
      net: net,
      gross: self.gross_price,
      commission: net * commission_rate,
    };

    return Report {
      extra_commission: self.extra_commission,
      price: price,
    };
  }
}
