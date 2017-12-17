extern crate rust_3d;

use rust_3d::cgmath::One;
use rust_3d::geometry::Geometry;
use rust_3d::glium::Program;
use rust_3d::glium::Surface;
use rust_3d::glium::index::NoIndices;
use rust_3d::manifold;
use rust_3d::navigator::Navigator;
use rust_3d::prelude::Coord3d;
use rust_3d::prelude::Trans4d;
use rust_3d::render;
use rust_3d::shaders;
use rust_3d::transform;
use rust_3d::window::Window;
use std::ops::Deref;

fn main() {
    let function = |x, y| Coord3d::new(x as f32 / 100.0 - 0.5, y as f32 / 100.0 - 0.5, (x * y) as f32 / 10000.0);
    let manifold = manifold::from_fn(100, 100, function);

    let window = Window::show();

    // TODO: Create Shaders trait
    let shaders = Program::from_source(window.deref(), include_str!("../shaders/vertex.glsl"), include_str!("../shaders/fragment.glsl"), None).unwrap();

    let loaded = window.load(Geometry::from_manifold(manifold));

    let mut navigator = Navigator::default();
    let perspective = transform::reasonable_perspective(0.01, 100.0);
    window.start_loop(|window| {
        for event in window.poll_events() {
            navigator.handle_event(&event)?;
        }
        let object_transform = Trans4d::one();
        let total_transform = window.fix_aspect_ratio_x(perspective * navigator.calculate_transform() * object_transform);
        let normal_transform = transform::transform_normals(&object_transform);

        let uniforms = shaders::create_uniforms(total_transform, normal_transform, Coord3d::new(-1.0, 1.0, -1.0), Coord3d::new(1.0, 1.0, 1.0));
        window.draw_whole_frame_with_depth(|frame| {
            frame
                .draw(&loaded.buffer, &NoIndices(loaded.loaded_object.primitive_type), &shaders, &uniforms, &render::default_draw_parameters())
                .unwrap();
        });
        Ok(())
    });
}
