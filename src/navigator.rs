use cgmath::Rad;
use glium::glutin::ElementState;
use glium::glutin::Event;
use glium::glutin::MouseButton;
use glium::glutin::WindowEvent;
use prelude::Coord3d;
use prelude::Trans4d;
use std::default::Default;
use std::f32::consts::PI;

pub struct Navigator {
    pub phi: f32,
    pub theta: f32,
    pub distance: f32,

    pub pixels_per_revolution: f64,
    pub pixels_per_double_distance: f64,

    mouse_position: (f64, f64),
    rotation_start_position: Option<(f64, f64)>,
    translation_start_position: Option<(f64, f64)>,
}

impl Navigator {
    pub fn new(initial_phi: f32, initial_theta: f32, initial_distance: f32, pixels_per_revolution: f64, pixels_per_double_distance: f64) -> Self {
        Navigator {
            phi: initial_phi,
            theta: initial_theta,
            distance: initial_distance,

            pixels_per_revolution,
            pixels_per_double_distance,

            mouse_position: Default::default(),
            rotation_start_position: None,
            translation_start_position: None,
        }
    }

    pub fn calculate_transform(&self) -> Trans4d {
        let rotation = Trans4d::from_angle_x(-Rad(self.theta)) * Trans4d::from_angle_z(-Rad(self.phi));
        Trans4d::from_translation(Coord3d::new(0.0, 0.0, -self.distance)) * rotation
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                WindowEvent::CursorMoved {
                    device_id: _,
                    position,
                } => self.mouse_moved(position),
                WindowEvent::MouseInput {
                    device_id: _,
                    state: ElementState::Pressed,
                    button: MouseButton::Left,
                } => self.rotation_start(),
                WindowEvent::MouseInput {
                    device_id: _,
                    state: ElementState::Released,
                    button: MouseButton::Left,
                } => self.rotation_stop(),
                WindowEvent::MouseInput {
                    device_id: _,
                    state: ElementState::Pressed,
                    button: MouseButton::Right,
                } => self.translation_start(),
                WindowEvent::MouseInput {
                    device_id: _,
                    state: ElementState::Released,
                    button: MouseButton::Right,
                } => self.translation_stop(),
                _ => {}
            },
            _ => {}
        }
    }

    fn rotation_start(&mut self) {
        self.rotation_start_position = Some(self.mouse_position);
    }

    fn rotation_stop(&mut self) {
        self.rotation_start_position = None;
    }

    fn translation_start(&mut self) {
        self.translation_start_position = Some(self.mouse_position);
    }

    fn translation_stop(&mut self) {
        self.translation_start_position = None;
    }

    fn mouse_moved(&mut self, new_position: (f64, f64)) {
        self.mouse_position = new_position;
        if let Some(old_position) = self.rotation_start_position {
            self.phi += 2.0 * PI * (old_position.0 - new_position.0) as f32 / self.pixels_per_revolution as f32;
            self.theta += 2.0 * PI * (old_position.1 - new_position.1) as f32 / self.pixels_per_revolution as f32;
            self.theta = self.theta.min(PI).max(0.0);
            self.rotation_start_position = Some(new_position);
        }
        if let Some(old_position) = self.translation_start_position {
            self.distance *= 2.0f32.powf((new_position.1 - old_position.1) as f32 / self.pixels_per_double_distance as f32);
            self.translation_start_position = Some(new_position);
        }
    }
}

impl Default for Navigator {
    fn default() -> Self {
        Self::new(0.15 * PI, 0.35 * PI, 5.0, 500.0, 200.0)
    }
}
