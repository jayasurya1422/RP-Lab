mod geometry {
    pub fn triangle_area(base: f32, height: f32) -> f32 {
        0.5 * base * height
    }
}

fn main() {
    let base = 10.0;
    let height = 5.0;
    let area = geometry::triangle_area(base, height);
    println!("Area of the triangle: {}", area);
}
