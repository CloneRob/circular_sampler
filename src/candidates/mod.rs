#[allow(unused_imports, dead_code)]
extern crate rand;

use std::f64;
use std::ops::Add;
use rand::distributions::{IndependentSample, Range};
use std::cmp::Ordering;

#[derive (Copy, Clone)]
pub struct Point {
    x: f64, 
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point {
            x: x,
            y: y,
        }
    }

    fn distance(&self, p: &Point) -> f64 {
        let x_component = (self.x - p.x) * (self.x - p.x);
        let y_component = (self.y - p.y) * (self.y - p.y);
        return f64::sqrt(x_component + y_component);
    }

    fn avg(self, n: f64) -> Point {
        Point::new(self.x / n, self.y / n)
    }

    pub fn as_int(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }

}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }

}

pub fn generate_candidates(n: usize, scale: f64, threshold: f64) -> Vec<Point> {
    remove_centroids(gen_coords(n, scale), threshold)
}

fn gen_coords(n: usize, scale: f64) -> Vec<Point> {
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
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut indices = Vec::new();
    for &(i, distance) in distances.iter() {
        if let Some(Ordering::Greater) = distance.partial_cmp(&threshold) {
            break;
        }
        indices.push(i);
    }
    indices.sort_by(|a, b| b.cmp(a));
    let mut centroid = point_slice[indices[0]]; 
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
        centroids.push(centroid);

        for i in indices.iter() {
            points.swap_remove(*i);
        }
        counter = counter - indices.len();

    }
    points.extend_from_slice(&centroids[..]);
    points
}
