use crate::geometry::Geometry;
use crate::prelude::Coord3d;
use glium::index::PrimitiveType;

pub fn tetrahedron() -> Geometry {
    let mut tetrahedron = Geometry::new(PrimitiveType::TrianglesList);

    let a = Coord3d::new(1.0, 1.0, 1.0);
    let b = Coord3d::new(-1.0, -1.0, 1.0);
    let c = Coord3d::new(1.0, -1.0, -1.0);
    let d = Coord3d::new(-1.0, 1.0, -1.0);

    tetrahedron.push_triangle(a, b, c);
    tetrahedron.push_triangle(a, c, d);
    tetrahedron.push_triangle(a, d, b);
    tetrahedron.push_triangle(d, c, b);

    tetrahedron
}

pub fn cube() -> Geometry {
    let mut cube = Geometry::new(PrimitiveType::TrianglesList);

    let f_111 = Coord3d::new(1.0, 1.0, 1.0);
    let f_110 = Coord3d::new(1.0, 1.0, -1.0);
    let f_101 = Coord3d::new(1.0, -1.0, 1.0);
    let f_100 = Coord3d::new(1.0, -1.0, -1.0);
    let f_011 = Coord3d::new(-1.0, 1.0, 1.0);
    let f_010 = Coord3d::new(-1.0, 1.0, -1.0);
    let f_001 = Coord3d::new(-1.0, -1.0, 1.0);
    let f_000 = Coord3d::new(-1.0, -1.0, -1.0);

    cube.push_triangle(f_111, f_011, f_101);
    cube.push_triangle(f_101, f_011, f_001);
    cube.push_triangle(f_110, f_100, f_010);
    cube.push_triangle(f_010, f_100, f_000);

    cube.push_triangle(f_111, f_101, f_110);
    cube.push_triangle(f_110, f_101, f_100);
    cube.push_triangle(f_011, f_010, f_001);
    cube.push_triangle(f_001, f_010, f_000);

    cube.push_triangle(f_111, f_110, f_011);
    cube.push_triangle(f_011, f_110, f_010);
    cube.push_triangle(f_101, f_001, f_100);
    cube.push_triangle(f_100, f_001, f_000);

    cube
}
