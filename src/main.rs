use std::f32::consts::PI;
fn main() {
    println!("Hello, world!");
    let x = Circle{radius: 5.0};
    let area = x.area();
    let circ = x.circ();
    println!("Area of circle is {:#?}", area);
    println!("Circ of circle is {:#?}", circ);
}
struct  Circle{
    radius : f32,
}
trait Calculate {
    fn area(&self) -> f32;
    fn circ(&self) -> f32;
}
impl Calculate for Circle {
    fn area(&self) -> f32 {
        PI * self.radius.powf(2.0) as f32
    }
    fn circ(&self) -> f32 {
        2.0 * PI * self.radius as f32
    }
}