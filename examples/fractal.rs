#[macro_use]
extern crate glium;
extern crate cgmath;

use cgmath::*;
use glium::*;
use glium::glutin::*;
use glium::index::*;
use std::f32;

#[derive(Copy, Clone)]
pub struct Attribute {
    coord: (f32, f32),
}

impl Attribute {
    pub fn new(x: f32, y: f32) -> Self {
        Attribute { coord: (x, y) }
    }
}

implement_vertex!(Attribute, coord);

pub fn main() {
    let frame = WindowBuilder::new()
        .with_title("Fractalium".to_string())
        .build_glium()
        .unwrap();
    let window = frame.get_window().unwrap();

    let program = Program::from_source(&frame, include_str!("shaders/fractal_vertex.glsl"), include_str!("shaders/fractal_fragment.glsl"), None).unwrap();

    let screen_geometry = [Attribute::new(-2.0, -2.0), Attribute::new(2.0, -2.0), Attribute::new(-2.0, 2.0), Attribute::new(2.0, 2.0)];
    let screen = VertexBuffer::new(&frame, &screen_geometry).unwrap();

    let (width, height) = frame.get_framebuffer_dimensions();
    let mut navigator = Navigator::new(width, height, 100);

    loop {
        for event in window.poll_events() {
            if !navigator.handle_event(&event) {
                return;
            }
            if let Event::Resized(x, y) = event {
                navigator.width = x;
                navigator.height = y;
            }
        }

        let transform: [[f32; 4]; 4] = navigator.calculate_transform().into();
        let uniform = uniform!{ transform: transform };

        let mut framebuffer = frame.draw();
        framebuffer.clear_color(0.0, 0.0, 0.0, 1.0);
        framebuffer
            .draw(&screen, &NoIndices(PrimitiveType::TriangleStrip), &program, &uniform, &Default::default())
            .unwrap();
        framebuffer.finish().unwrap();
    }
}

struct Navigator {
    width: u32,
    height: u32,

    x: f32,
    y: f32,
    zoom: f32,

    pixels_per_double_distance: usize,

    mouse_position: (i32, i32),
    shift_start_position: Option<(i32, i32)>,
    zoom_start_position: Option<(i32, i32)>,
}

impl Navigator {
    pub fn new(width: u32, height: u32, pixels_per_double_distance: usize) -> Self {
        Navigator {
            width: width,
            height: height,

            x: 0.0,
            y: 0.0,
            zoom: 0.5,

            pixels_per_double_distance: pixels_per_double_distance,

            mouse_position: Default::default(),
            shift_start_position: None,
            zoom_start_position: None,
        }
    }

    pub fn calculate_transform(&self) -> Matrix4<f32> {
        Matrix4::from_scale(self.zoom) * Matrix4::from_translation(Vector3::new(self.x, self.y, 0.0))
    }

    fn shift_start(&mut self) {
        self.shift_start_position = Some(self.mouse_position);
    }

    fn shift_stop(&mut self) {
        self.shift_start_position = None;
    }

    fn zoom_start(&mut self) {
        self.zoom_start_position = Some(self.mouse_position);
    }

    fn zoom_stop(&mut self) {
        self.zoom_start_position = None;
    }

    fn mouse_moved(&mut self, new_position: (i32, i32)) {
        self.mouse_position = new_position;
        if let Some(old_position) = self.shift_start_position {
            self.x += 2.0 * (new_position.0 - old_position.0) as f32 / self.zoom / self.width as f32;
            self.y += 2.0 * (old_position.1 - new_position.1) as f32 / self.zoom / self.height as f32;
            self.shift_start_position = Some(new_position);
        }
        if let Some(old_position) = self.zoom_start_position {
            self.zoom *= 2.0f32.powf((old_position.1 - new_position.1) as f32 / self.pixels_per_double_distance as f32);
            self.zoom_start_position = Some(new_position);
        }
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        match *event {
            Event::MouseMoved(x, y) => self.mouse_moved((x, y)),
            Event::MouseInput(ElementState::Pressed, MouseButton::Left) => self.shift_start(),
            Event::MouseInput(ElementState::Released, MouseButton::Left) => self.shift_stop(),
            Event::MouseInput(ElementState::Pressed, MouseButton::Right) => self.zoom_start(),
            Event::MouseInput(ElementState::Released, MouseButton::Right) => self.zoom_stop(),
            Event::Closed => return false,
            _ => (),
        }
        true
    }
}
