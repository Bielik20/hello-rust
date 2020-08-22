mod models;
use models::Price;
use models::Assignment;
use models::Report;
mod say_hello;
use say_hello::say_hello;

fn main() {
    say_hello();

    let mut x = Price::noop();
    let mut y = Price::noop();
    let assignment1 = Assignment {
        extra_commission: 100.0,
        tax: 0.23,
        gross_price: 200.0,
    };
    let assignment2 = Assignment {
        extra_commission: 120.0,
        tax: 0.8,
        gross_price: 220.0,
    };
    let report = Report::create_from_one(&assignment1, 0.1);
    let full_report = Report::create_from_many(vec![assignment1, assignment2], 0.1);
    x.net = 12.0;
    y.net = 13.0;
    println!("{:?}", x + y);
    println!("{:?}", report);
    println!("{:?}", full_report);
}
