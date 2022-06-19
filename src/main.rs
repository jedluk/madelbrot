mod rgb;
mod mandelbrot;

fn main() {
    let color1 = rgb::RGB::new(10.0, 11.0, 12.0);
    let color2 = rgb::RGB::new(230.0, 122.0, 12.0);

    let color3 = color1 - color2;

    let iterations = mandelbrot::get_iterations(0.3, 0.2);

    println!("color: {}", color3);
    println!("iterations {}", iterations.to_string());
}
