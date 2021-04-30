type Function = dyn Fn(f64) -> f64;

pub fn trapezoidal_area(fnc: &Function, x_start: f64, x_end: f64, steps: i32) -> f64 {
    let mut x1 = x_start;
    let mut fx1 = fnc(x1);

    let step  = (x_end - x_start) / (steps as f64);
    let mut area = 0.0;

    for _ in 0..steps {
        let x2 = x1 + step;
        let fx2 = fnc(x2);

        area += (fx1 + fx2).abs() * (x2 - x1);
        x1 = x2;
        fx1 = fx2;
    }
    area / 2_f64
}

fn main() {
    fn f(x: f64) -> f64 {
        x.powi(3) + x.powi(2)
    }

    println!("f(x) = x^3 + x^2");
    println!("The area between the curve, x = -5, x = 5 and the x axis is:");

    let mut i = 10;
    while i <= 100000 {
        println!("with {} steps: {}", i, trapezoidal_area(&f, -5_f64, 5_f64, i));
        i *= 10;
    }
}