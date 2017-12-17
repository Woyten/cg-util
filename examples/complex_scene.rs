extern crate rand;
extern crate rust_3d;

use rust_3d::cgmath::One;
use rust_3d::cgmath::Rad;
use rust_3d::geometry;
use rust_3d::geometry::Geometry;
use rust_3d::glium::BlitTarget;
use rust_3d::glium::Program;
use rust_3d::glium::Rect;
use rust_3d::glium::Surface;
use rust_3d::glium::VertexBuffer;
use rust_3d::glium::glutin::Event;
use rust_3d::glium::index::NoIndices;
use rust_3d::glium::index::PrimitiveType;
use rust_3d::glium::texture::Texture2d;
use rust_3d::glium::uniforms::MagnifySamplerFilter;
use rust_3d::manifold;
use rust_3d::navigator::Navigator;
use rust_3d::prelude::*;
use rust_3d::render;
use rust_3d::shaders;
use rust_3d::solids;
use rust_3d::transform;
use rust_3d::window::Window;
use std::ops::Deref;
use std::time::Instant;

fn main() {
    let window = Window::show();
    let shaders = Program::from_source(window.deref(), include_str!("../shaders/vertex.glsl"), include_str!("../shaders/fragment.glsl"), None).unwrap();
    let texture_shaders = Program::from_source(window.deref(), include_str!("../shaders/tex_vertex.glsl"), include_str!("../shaders/tex_fragment.glsl"), None)
        .unwrap();

    let (mut width, mut height) = window.deref().get_framebuffer_dimensions();
    let mut texture = Texture2d::empty(window.deref(), width, height).unwrap();

    let mut screen_frame = window.load(create_screen_frame(width, height));
    let mut screen = VertexBuffer::new(window.deref(), &create_screen(width, height)).unwrap();
    let torus1 = window.load(Geometry::from_manifold(manifold::torus(180, 180, 0.2)));
    let torus2 = window.load(Geometry::from_manifold(manifold::torus(180, 180, 0.5)));
    let torus3 = window.load(Geometry::from_manifold(manifold::torus(180, 180, 1.0)));
    let sphere = window.load(Geometry::from_manifold(manifold::sphere(180, 90)));
    let tetrahedron = window.load(solids::tetrahedron());
    let cube = window.load(solids::cube());
    let grid = window.load(geometry::xy_grid((9, 9)));
    let transparent_polygon = window.load(Geometry::from_manifold(manifold::sphere(8, 4)));

    let colors = (random_color(), random_color(), random_color(), random_color(), random_color());

    let perspective = transform::reasonable_perspective(0.01, 1000.0);

    let mut navigator = Navigator::default();
    let start = Instant::now();
    window.start_loop(|window| {
        for event in window.poll_events() {
            navigator.handle_event(&event)?;
            if let Event::Resized(x, y) = event {
                width = x;
                height = y;
                screen_frame = window.load(create_screen_frame(width, height));
                screen = VertexBuffer::new(window.deref(), &create_screen(width, height)).unwrap();
                texture = Texture2d::empty(window.deref(), width, height).unwrap();
            }
        }
        let scene_transform = window.fix_aspect_ratio_x(perspective * navigator.calculate_transform());

        let elapsed = start.elapsed();
        let seconds = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 * 1e-9;

        window.draw_whole_frame_with_depth(|frame| {
            {
                let total_transform = scene_transform;
                let normal_transform = Trans3d::one();

                let uniforms = shaders::create_tex_uniforms(total_transform, normal_transform, Coord3d::new(0.0, 0.0, 0.0), &texture);
                frame
                    .draw(&screen, &NoIndices(PrimitiveType::TriangleStrip), &texture_shaders, &uniforms, &render::default_draw_parameters())
                    .unwrap();
            }

            let mut objects = Vec::new();
            objects.push((&screen_frame, false, Coord3d::new(1.0, 1.0, 1.0), Trans4d::one()));
            objects.push((&torus1, false, colors.0, Trans4d::from_translation(Coord3d::new(3.0, 3.0, 0.0)) * Trans4d::from_scale(1.25)));
            objects.push((&torus2, false, colors.1, Trans4d::from_translation(Coord3d::new(0.0, 3.0, 0.0)) * Trans4d::from_scale(1.0)));
            objects.push((&torus3, false, colors.2, Trans4d::from_translation(Coord3d::new(-3.0, 3.0, 0.0)) * Trans4d::from_scale(0.75)));
            objects.push((
                &sphere,
                false,
                colors.3,
                Trans4d::from_translation(Coord3d::new(3.0, 0.0, 0.0)) * Trans4d::from_scale(0.5 + 0.1 * (4.0 * seconds).sin()),
            ));
            objects.push((&sphere, false, colors.4, Trans4d::from_translation(Coord3d::new(-3.0, 0.0, 0.0)) * Trans4d::from_scale(1.5)));
            objects.push((
                &tetrahedron,
                false,
                colors.0,
                Trans4d::from_translation(Coord3d::new(3.0, -3.0, 0.0)) * Trans4d::from_angle_z(Rad(2.0 * seconds)),
            ));
            objects.push((
                &tetrahedron,
                false,
                colors.1,
                Trans4d::from_translation(Coord3d::new(-0.0, -3.0, 0.0)) * Trans4d::from_angle_y(Rad(1.0 * seconds)),
            ));
            objects.push((
                &cube,
                false,
                colors.2,
                Trans4d::from_translation(Coord3d::new(-3.0, -3.0, 0.0)) * Trans4d::from_angle_y(Rad(3.0 * seconds)),
            ));
            objects.push((&grid, false, colors.3, Trans4d::from_scale(4.5)));
            objects.push((&transparent_polygon, true, colors.4, Trans4d::one()));

            for (loaded, transparent, color, object_transform) in objects {
                let total_transform = scene_transform * object_transform;
                let normal_transform = transform::transform_normals(&object_transform);

                let uniforms = shaders::create_uniforms(total_transform, normal_transform, Coord3d::new(seconds.sin(), seconds.cos(), -1.0), color);
                let draw_parameters = if transparent { render::transparency_draw_parameters(0.3) } else { render::default_draw_parameters() };
                frame
                    .draw(&loaded.buffer, &NoIndices(loaded.loaded_object.primitive_type), &shaders, &uniforms, &draw_parameters)
                    .unwrap();
            }

            frame.blit_color(
                &Rect {
                    left: 0,
                    bottom: 0,
                    width: width,
                    height: height,
                },
                &texture.as_surface(),
                &BlitTarget {
                    left: 0,
                    bottom: 0,
                    width: width as i32,
                    height: height as i32,
                },
                MagnifySamplerFilter::Linear,
            );
        });
        Ok(())
    });
}

fn random_color() -> Coord3d {
    Coord3d::new(rand::random(), rand::random(), rand::random())
}

fn create_screen_frame(width: u32, height: u32) -> Geometry {
    let aspect_ratio = width as f32 / height as f32;
    let normal = Coord3d::new(0.0, 0.0, 0.0);
    let mut attributes = Vec::new();
    attributes.push(VertexAttribute::new(Coord3d::new(-4.5, 4.5, 0.0), normal));
    attributes.push(VertexAttribute::new(Coord3d::new(4.5, 4.5, 0.0), normal));
    attributes.push(VertexAttribute::new(Coord3d::new(4.5, 4.5, 9.0 / aspect_ratio), normal));
    attributes.push(VertexAttribute::new(Coord3d::new(-4.5, 4.5, 9.0 / aspect_ratio), normal));

    Geometry {
        primitive_type: PrimitiveType::LineLoop,
        vertices: attributes,
    }
}

fn create_screen(width: u32, height: u32) -> Vec<TexVertexAttribute> {
    let aspect_ratio = width as f32 / height as f32;
    let normal = Coord3d::new(0.0, 0.0, 0.0);
    let mut attributes = Vec::new();
    attributes.push(TexVertexAttribute::new(Coord3d::new(-4.5, 4.5, 0.0), normal, Coord2d::new(0.0, 0.0)));
    attributes.push(TexVertexAttribute::new(Coord3d::new(4.5, 4.5, 0.0), normal, Coord2d::new(1.0, 0.0)));
    attributes.push(TexVertexAttribute::new(Coord3d::new(-4.5, 4.5, 9.0 / aspect_ratio), normal, Coord2d::new(0.0, 1.0)));
    attributes.push(TexVertexAttribute::new(Coord3d::new(4.5, 4.5, 9.0 / aspect_ratio), normal, Coord2d::new(1.0, 1.0)));
    attributes
}
