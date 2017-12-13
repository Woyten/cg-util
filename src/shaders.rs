use glium::texture::Texture2d;
use glium::uniforms::EmptyUniforms;
use glium::uniforms::UniformsStorage;
use prelude::*;

type Cons<'a, Next, T> = UniformsStorage<'a, T, Next>;
type Nil = EmptyUniforms;

pub fn create_uniforms<'a>(total_transform: Trans4d,
                           normal_transform: Trans3d,
                           light_direction: Coord3d,
                           color: Coord3d)
                           -> Cons<'a, Cons<'a, Cons<'a, Cons<'a, Nil, [[f32; 4]; 4]>, [[f32; 3]; 3]>, [f32; 3]>, [f32; 3]> {
    uniform! {
        position_transform: total_transform.into(),
        normal_transform: normal_transform.into(),
        light_direction: light_direction.into(),
        color: color.into(),
    }
}

pub fn create_tex_uniforms<'a>(total_transform: Trans4d,
                               normal_transform: Trans3d,
                               light_direction: Coord3d,
                               texture: &Texture2d)
                               -> Cons<'a, Cons<'a, Cons<'a, Cons<'a, Nil, [[f32; 4]; 4]>, [[f32; 3]; 3]>, [f32; 3]>, &Texture2d> {
    uniform! {
        position_transform: total_transform.into(),
        normal_transform: normal_transform.into(),
        light_direction: light_direction.into(),
        color: texture,
    }
}
