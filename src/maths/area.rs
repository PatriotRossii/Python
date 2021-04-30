pub fn surface_area_cube(side_length: f64) -> Result<f64, &'static str> {
    if side_length < 0_f64 {
        Err("surface_area_cube() only accepts non-negative values")
    } else {
        Ok(6_f64 * side_length.powi(2))
    }
}

pub fn surface_area_sphere(radius: f64) -> Result<f64, &'static str> {
    if radius < 0_f64 {
        Err("surface_area_sphere() only accepts non-negative values")
    } else {
        Ok(4_f64 * std::f64::consts::PI * radius.powi(2))
    }
}

pub fn area_rectangle(length: f64, width: f64) -> Result<f64, &'static str> {
    if length < 0_f64 || width < 0_f64 {
        Err("area_rectangle() only accepts non-negative values")
    } else {
        Ok(length * width)
    }
}

pub fn area_square(side_length: f64) -> Result<f64, &'static str> {
    if side_length < 0_f64 {
        Err("area_square() only accepts non-negative values")
    } else {
        Ok(side_length.powi(2))
    }
}

pub fn area_triangle(base: f64, height: f64) -> Result<f64, &'static str> {
    if base < 0_f64 || height < 0_f64 {
        Err("area_triangle() only accepts non-negative values")
    } else {
        Ok((base * height) / 2_f64)
    }
}

pub fn area_triangle_three_sides(side1: f64, side2: f64, side3: f64) -> Result<f64, &'static str> {
    if side1 < 0_f64 || side2 < 0_f64 || side3 < 0_f64 {
        Err("area_triangle_three_sides() only accepts non-negative values")
    } else if side1 + side2 < side3 || side1 + side3 < side2 || side2 + side3 < side1 {
        Err("Given three sides do not form a triangle")
    } else {
        let semi_perimeter = (side1 + side2 + side3) / 2_f64;
        Ok((semi_perimeter
            * (semi_perimeter - side1)
            * (semi_perimeter - side2)
            * (semi_perimeter - side3))
            .sqrt())
    }
}

pub fn area_parallelogram(base: f64, height: f64) -> Result<f64, &'static str> {
    if base < 0_f64 || height < 0_f64 {
        Err("area_parallelogram() only accepts non-negative values")
    } else {
        Ok(base * height)
    }
}

pub fn area_trapezium(base1: f64, base2: f64, height: f64) -> Result<f64, &'static str> {
    if base1 < 0_f64 || base2 < 0_f64 || height < 0_f64 {
        Err("area_trapezium() only accepts non-negative values")
    } else {
        Ok(0.5_f64 * (base1 + base2) * height)
    }
}

pub fn area_circle(radius: f64) -> Result<f64, &'static str> {
    if radius < 0_f64 {
        Err("area_circle() only accepts non-negative values")
    } else {
        Ok(std::f64::consts::PI * radius.powi(2))
    }
}

pub fn area_ellipse(radius_x: f64, radius_y: f64) -> Result<f64, &'static str> {
    if radius_x < 0_f64 || radius_y < 0_f64 {
        Err("area_ellipse() only accepts non-negative values")
    } else {
        Ok(std::f64::consts::PI * radius_x * radius_y)
    }
}

pub fn area_rhombus(diag1: f64, diag2: f64) -> Result<f64, &'static str> {
    if diag1 < 0_f64 || diag2 < 0_f64 {
        Err("area_rhombus() only accepts non-negative values")
    } else {
        Ok(0.5_f64 * diag1 * diag2)
    }
}