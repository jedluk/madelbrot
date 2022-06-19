use num::complex::Complex;

pub static MAX_ITERATIONS: i32 = 1000;

pub fn get_iterations(x: f64, y: f64) -> i32 {
    let mut z = Complex::new(0.0, 0.0);
    let c = Complex::new(x, y);

    let mut iterations = 0;

    while iterations < MAX_ITERATIONS {
        z = z * z + c;
        if z.norm() > 2.0 {
            break;
        }
        iterations += 1;
    }

    iterations
}
