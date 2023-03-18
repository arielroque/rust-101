struct Circle{
    radius: f32
}

struct Rectangule{
    width: f32,
    height: f32
}

trait Area{
    fn shape_area(&self) -> f32;
}

impl Area for Circle{
    fn shape_area(&self)-> f32{
        return 3.14 * self.radius * self.radius;
    }
}

impl Area for Rectangule{
    fn shape_area(&self) -> f32{
        return self.width * self.height;
    }
}


fn main() {
    // Traits are like interfaces in Java

    let circle = Circle{radius:3.0};
    let rectangule = Rectangule{width:5.0,height:7.2};

    println!("{}",circle.shape_area());
    println!("{}",rectangule.shape_area());

}
