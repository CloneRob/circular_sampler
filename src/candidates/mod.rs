#[allow(unused_imports, dead_code)]

use rand;
use std::f64;
use std::ops::Add;
use rand::distributions::{IndependentSample, Range};
use std::cmp::Ordering;

#[derive (Copy, Clone, Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    pub fn zero() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    fn distance(&self, p: &Point) -> f64 {
        let x_component = (self.x - p.x) * (self.x - p.x);
        let y_component = (self.y - p.y) * (self.y - p.y);
        return f64::sqrt(x_component + y_component);
    }

    fn avg(self, n: f64) -> Point {
        Point::new(self.x / n, self.y / n)
    }

    fn to_int(self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }

    fn to_uint(self) -> (u32, u32) {
        if self.x < 0f64 || self.y < 0f64 {
            println!("Point(x: {}, y: {})", self.x, self.y);
            panic!("Applied to_unit() on point with negative coordinates")
        }
        (self.x as u32, self.y as u32)
    }

    fn print(&self) {
        println!("Point(x: {}, y: {})", self.x, self.y);
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

pub fn transform_points(points: Vec<Point>, transformer: Point) -> Vec<Point> {
    points.iter().map(|x| (*x + transformer)).collect()
}

pub fn to_coords(points: Vec<Point>, transformer: Option<Point>) -> Vec<(u32, u32)> {
    let t = if let Some(trans) = transformer {
        trans
    } else {
        Point::new(0.0, 0.0)
    };
    points.iter().map(|x| (*x + t).to_uint()).collect()
}

pub fn generate_candidates(n: usize, scale: f64, threshold: Option<f64>) -> Vec<Point> {
    let points = circular_coords(n, scale);
    if let Some(t) = threshold {
        remove_centroids(points, t)
    } else {
        points
    }
}

fn circular_coords(n: usize, scale: f64) -> Vec<Point> {
    let mut nums = Vec::with_capacity(n);

    let theta_range = Range::new(0f64, f64::consts::PI * 2f64);
    let rho_range = Range::new(0f64, 1f64);
    let mut rng = rand::thread_rng();

    for _ in 0..n {
        let t = theta_range.ind_sample(&mut rng);
        let r = f64::sqrt(rho_range.ind_sample(&mut rng));

        let x = f64::cos(t) * r * scale;
        let y = f64::sin(t) * r * scale;

        nums.push(Point::new(x, y));
    }
    nums
}

fn centroid_candidates(point_slice: &[Point], threshold: f64) -> (Point, Vec<usize>) {
    let mut distances: Vec<(usize, f64)> = Vec::with_capacity(point_slice.len());
    for (i, point) in point_slice.iter().enumerate() {
        let d = point_slice[0].distance(&point);
        distances.push((i, d));
    }
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Less));

    let mut indices = Vec::new();
    for &(i, distance) in distances.iter() {
        if let Some(Ordering::Greater) = distance.partial_cmp(&threshold) {
            break;
        }
        indices.push(i);
    }
    indices.sort_by(|a, b| b.cmp(a));
    let mut centroid = Point::zero();
    for i in indices.iter() {
        centroid = centroid + point_slice[*i];
    }

    centroid = centroid.avg(indices.len() as f64);
    (centroid, indices)
}

fn remove_centroids(mut points: Vec<Point>, threshold: f64) -> Vec<Point> {
    let mut centroids = Vec::new();
    let mut counter = points.len();

    while counter > 0 {
        let (centroid, indices) = centroid_candidates(&points[..], threshold);
        for i in indices.iter() {
            points.swap_remove(*i);
        }
        centroids.push(centroid);
        counter = counter - indices.len();

    }
    points.extend_from_slice(&centroids[..]);
    points
}
