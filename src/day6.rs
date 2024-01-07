pub type Time = u64;
pub type Distance = u64;

// Represents ax² + bx + c
struct SecondOrderPolynomial(i64, i64, i64);

pub fn beat_record(total_time: Time, target_distance: Distance) -> u64 {
    // t * (T - t) >= D
    // -t² + Tt - D >= 0
    // t² - Tt + D >= 0
    let polynomial = SecondOrderPolynomial(1, -(total_time as i64), target_distance as i64);

    match solve_equation(polynomial) {
        Some((a, b)) => {
            // The inequality is strict so the edges are exclusive.
            // If they are already integers we need to pad them.
            let left: i64 = if is_integer(a) { a as i64 + 1 } else { a.ceil() as i64 };
            let right: i64 = if is_integer(b) { b as i64 - 1 } else { b.floor() as i64 };
            (right - left) as u64 + 1
        },
        None => 0,
    }
}

pub fn is_integer(x: f64) -> bool {
    x == (x as u64) as f64
}

// Returns the (real) roots of a second-order equation.
// If the roots are complex, it returns None.
// Double roots are duplicated, it is up to the caller to handle them.
fn solve_equation(polynomial: SecondOrderPolynomial) -> Option<(f64, f64)> {
    let (a, b, c) = (polynomial.0, polynomial.1, polynomial.2);
    let minus_b: f64 = -b as f64;
    let discriminant: f64 = (b * b - 4 * a * c) as f64;
    let denominator: f64 = (2 * a) as f64;

    if discriminant <= 0.0 {
        None
    } else {
        Some((
            (minus_b - discriminant.sqrt()) / denominator,
            (minus_b + discriminant.sqrt()) / denominator,
        ))
    }
}
