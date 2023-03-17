use std::f64::consts::PI;

fn main() {
    println!("The area of a circle of radius 3 is {}",circle_area(3_f64));
}

fn circle_area(radius: f64) -> f64 {
    radius * radius * PI
    }
