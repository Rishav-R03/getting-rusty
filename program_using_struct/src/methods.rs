pub struct Circle {
    pub radius: f64, // here radius is public
}

pub struct Square {
    side: f64,
}

impl Square {
    pub fn area(&self)->f64 {
        self.side.powi(2)
    }
    //perimeter
    pub fn perimeter(&self) -> f64 {
        self.side*4.0
    }
}
impl Circle {
    //A method to calculate area
    //Uses &self because we only need to read the radius
    pub fn area(&self) -> f64 {
        3.14159 * self.radius.powi(2)
    }
}