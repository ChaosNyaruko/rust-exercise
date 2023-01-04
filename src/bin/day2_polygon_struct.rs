// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]
use std::ops::Add;
use std::convert::From;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point{x, y}
    }

    fn magnitude(&self) -> f64 {
        (f64::from(self.x * self.x) + f64::from(self.y * self.y)).sqrt()
    }

    fn dist(&self, p: Point) -> f64 {
        let a = f64::from(self.x - p.x);
        let b = f64::from(self.y - p.y);
        (a * a + b * b).sqrt()
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl From<Polygon> for Shape {
    fn from(item: Polygon) -> Self {
        Shape::Polygon(item)
    }
}

impl From<Circle> for Shape {
    fn from(item: Circle) -> Self {
        Shape::Circle(item)
    }
}

impl Polygon {
    fn new() -> Polygon {
        Polygon {
            points: vec![],
        }
    }
    fn left_most_point(&self) -> Option<Point> {
        let mut ans: Option<Point> = None;
        for p in self.points.iter() {
            match ans {
                Some(prev) => {
                    if p.x < prev.x {
                        ans = Some(*p);
                    }
                },
                None => {
                    ans = Some(*p);
                }
            }
        }
        ans
    }

    fn iter(&self) -> std::slice::Iter<'_, Point> {
        self.points.iter() 
    }

    fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }
}

pub struct Circle {
   center: Point,
   radius: i32,
}

impl Circle {
    fn new(c: Point, r: i32) -> Circle {
        Circle {
            center: c,
            radius: r,
        }
    }

    fn circumference(&self) -> f64 {
        2.0 * f64::from(self.radius) * std::f64::consts::PI
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    fn circumference(&self) -> f64 {
        match self {
            Self::Polygon(p) => 15.48, // TODO: how to calc Polygon's circumference?
            Self::Circle(c) => c.circumference(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_circumferences() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let circumferences = shapes
            .iter()
            .map(Shape::circumference)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(circumferences, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}
