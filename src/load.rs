use crate::prelude::*;
use glium::backend::glutin::Display;
use glium::VertexBuffer;

pub trait LoadOnGpu: Sized {
    fn load(self, facade: &Display) -> GpuObjectHandle<Self>;
}

pub struct GpuObjectHandle<O> {
    pub buffer: VertexBuffer<VertexAttribute>,
    pub loaded_object: O,
}
