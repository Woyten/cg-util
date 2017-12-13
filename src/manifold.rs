use prelude::Coord3d;
use std::f32::consts::PI;

pub trait Manifold2d: Sized {
    type Coord;

    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get(&self, x: usize, y: usize) -> Self::Coord;

    fn by_ref(&self) -> ByRef2d<Self> {
        ByRef2d { orig_manifold: self }
    }

    fn transformed<T, F>(self, transform: F) -> Transformed<Self, T>
        where F: 'static + Fn(Self::Coord) -> T
    {
        Transformed {
            orig_manifold: self,
            transform: Box::new(transform),
        }
    }
}

pub fn from_fn<T, F>(width: usize, height: usize, function: F) -> FromFn<T>
    where F: 'static + Fn(usize, usize) -> T
{
    FromFn {
        width: width,
        height: height,
        function: Box::new(function),
    }
}

pub struct FromFn<T> {
    width: usize,
    height: usize,
    function: Box<Fn(usize, usize) -> T>,
}

impl<T> Manifold2d for FromFn<T> {
    type Coord = T;

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&self, x: usize, y: usize) -> Self::Coord {
        (self.function)(x, y)
    }
}

pub struct ByRef2d<'a, M: 'a> {
    orig_manifold: &'a M,
}

impl<'a, M: Manifold2d> Manifold2d for ByRef2d<'a, M> {
    type Coord = M::Coord;

    fn width(&self) -> usize {
        self.orig_manifold.width()
    }

    fn height(&self) -> usize {
        self.orig_manifold.height()
    }

    fn get(&self, x: usize, y: usize) -> Self::Coord {
        self.orig_manifold.get(x, y)
    }
}

pub struct Transformed<M: Manifold2d, T> {
    orig_manifold: M,
    transform: Box<Fn(M::Coord) -> T>,
}

impl<M: Manifold2d, T> Manifold2d for Transformed<M, T> {
    type Coord = T;

    fn width(&self) -> usize {
        self.orig_manifold.width()
    }

    fn height(&self) -> usize {
        self.orig_manifold.height()
    }

    fn get(&self, x: usize, y: usize) -> T {
        (self.transform)(self.orig_manifold.get(x, y))
    }
}

pub fn sphere(phi_grid: usize, theta_grid: usize) -> FromFn<Coord3d> {
    fn sphere_function(u: usize, v: usize, phi_grid: f32, theta_grid: f32) -> Coord3d {
        let phi = 2.0 * PI * u as f32 / phi_grid;
        let theta = PI * v as f32 / theta_grid;
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let x = cos_phi * sin_theta;
        let y = sin_phi * sin_theta;
        let z = cos_theta;
        Coord3d::new(x, y, z)
    }

    let function = move |x, y| sphere_function(x, y, phi_grid as f32, theta_grid as f32);
    from_fn(phi_grid + 1, theta_grid + 1, function)
}

pub fn torus(phi_grid: usize, theta_grid: usize, thickness: f32) -> FromFn<Coord3d> {
    fn torus_function(u: usize, v: usize, phi_grid: f32, theta_grid: f32, thickness: f32) -> Coord3d {
        let phi = 2.0 * PI * u as f32 / phi_grid;
        let theta = 2.0 * PI * v as f32 / theta_grid;
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let x = cos_phi * (1.0 + thickness * sin_theta);
        let y = sin_phi * (1.0 + thickness * sin_theta);
        let z = cos_theta * thickness;
        Coord3d::new(x, y, z)
    }

    let function = move |x, y| torus_function(x, y, phi_grid as f32, theta_grid as f32, thickness);
    from_fn(phi_grid + 1, theta_grid + 1, function)
}
