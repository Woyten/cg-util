use crate::load::GpuObjectHandle;
use crate::load::LoadOnGpu;
use crate::manifold::Manifold2d;
use crate::prelude::Coord3d;
use crate::prelude::VertexAttribute;
use cgmath::InnerSpace;
use glium::backend::glutin::Display;
use glium::index::PrimitiveType;
use glium::VertexBuffer;

pub struct Geometry {
    pub primitive_type: PrimitiveType,
    pub vertices: Vec<VertexAttribute>,
}

impl Geometry {
    pub fn new(primitive_type: PrimitiveType) -> Geometry {
        Geometry {
            primitive_type,
            vertices: Vec::new(),
        }
    }

    pub fn from_line_points(coordinates: Vec<Coord3d>) -> Geometry {
        let mut geometry = Geometry::new(PrimitiveType::LineStrip);
        geometry.vertices.extend(
            coordinates
                .iter()
                .map(|&x| VertexAttribute::without_normal(x)),
        );
        geometry
    }

    pub fn from_manifold<M: Manifold2d<Coord = Coord3d>>(coordinates: M) -> Geometry {
        let mut surface = Geometry::new(PrimitiveType::TrianglesList);

        for x in 0..coordinates.width() - 1 {
            for y in 0..coordinates.height() - 1 {
                let nw = coordinates.get(x, y);
                let sw = coordinates.get(x, y + 1);
                let ne = coordinates.get(x + 1, y);
                let se = coordinates.get(x + 1, y + 1);
                let mid = (nw + sw + ne + se) / 4.0;

                surface.push_triangle(mid, nw, sw);
                surface.push_triangle(mid, sw, se);
                surface.push_triangle(mid, se, ne);
                surface.push_triangle(mid, ne, nw);
            }
        }

        surface
    }

    pub fn push_triangle(&mut self, v1: Coord3d, v2: Coord3d, v3: Coord3d) {
        let normal = (v3 - v1).cross(v3 - v2).normalize();

        for &vertex in [v1, v2, v3].iter() {
            self.vertices.push(VertexAttribute::new(vertex, normal));
        }
    }
}

impl LoadOnGpu for Geometry {
    fn load(self, facade: &Display) -> GpuObjectHandle<Self> {
        GpuObjectHandle {
            buffer: VertexBuffer::new(facade, &self.vertices).unwrap(),
            loaded_object: self,
        }
    }
}

pub fn xy_grid(divisions: (usize, usize)) -> Geometry {
    let mut geometry = Geometry::new(PrimitiveType::LinesList);

    let number_of_x_lines = divisions.0 + 1;
    for i in 0..number_of_x_lines {
        let x = -1.0 + (2 * i) as f32 / divisions.0 as f32;
        geometry
            .vertices
            .push(VertexAttribute::without_normal(Coord3d::new(x, -1.0, 0.0)));
        geometry
            .vertices
            .push(VertexAttribute::without_normal(Coord3d::new(x, 1.0, 0.0)));
    }

    let number_of_y_lines = divisions.1 + 1;
    for i in 0..number_of_y_lines {
        let x = -1.0 + (2 * i) as f32 / divisions.1 as f32;
        geometry
            .vertices
            .push(VertexAttribute::without_normal(Coord3d::new(-1.0, x, 0.0)));
        geometry
            .vertices
            .push(VertexAttribute::without_normal(Coord3d::new(1.0, x, 0.0)));
    }

    geometry
}
