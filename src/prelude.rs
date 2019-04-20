use glium::implement_vertex;

pub type Coord2d = cgmath::Vector2<f32>;
pub type Coord3d = cgmath::Vector3<f32>;
pub type Trans3d = cgmath::Matrix3<f32>;
pub type Trans4d = cgmath::Matrix4<f32>;

#[derive(Copy, Clone)]
pub struct VertexAttribute {
    vertex_position: (f32, f32, f32),
    vertex_normal: (f32, f32, f32),
}

implement_vertex!(VertexAttribute, vertex_position, vertex_normal);

impl VertexAttribute {
    pub fn new(position: Coord3d, normal: Coord3d) -> Self {
        VertexAttribute {
            vertex_position: position.into(),
            vertex_normal: normal.into(),
        }
    }

    pub fn without_normal(position: Coord3d) -> Self {
        Self::new(position, Coord3d::new(0.0, 0.0, 0.0))
    }
}

#[derive(Copy, Clone)]
pub struct TexVertexAttribute {
    vertex_position: (f32, f32, f32),
    vertex_normal: (f32, f32, f32),
    tex_coord: (f32, f32),
}

implement_vertex!(
    TexVertexAttribute,
    vertex_position,
    vertex_normal,
    tex_coord
);

impl TexVertexAttribute {
    pub fn new(position: Coord3d, normal: Coord3d, tex_coord: Coord2d) -> Self {
        TexVertexAttribute {
            vertex_position: position.into(),
            vertex_normal: normal.into(),
            tex_coord: tex_coord.into(),
        }
    }
}
