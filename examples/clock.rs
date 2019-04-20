use cg_util::geometry::Geometry;
use cg_util::load::GpuObjectHandle;
use cg_util::load::LoadOnGpu;
use cg_util::main_loop;
use cg_util::main_loop::State;
use cg_util::manifold;
use cg_util::navigator::Navigator;
use cg_util::prelude::Coord3d;
use cg_util::prelude::Trans4d;
use cg_util::render;
use cg_util::shaders;
use cg_util::solids;
use cg_util::transform;
use cgmath::Deg;
use glium::backend::glutin::Display;
use glium::glutin::Event;
use glium::index::NoIndices;
use glium::Frame;
use glium::Program;
use glium::Surface;

type ClockElement = (Trans4d, Coord3d);

struct ClockState {
    center: GpuObjectHandle<Geometry>,
    hour_marker: GpuObjectHandle<Geometry>,
    hand: GpuObjectHandle<Geometry>,
    shaders: Program,
    navigator: Navigator,
    perspective: Trans4d,
}

fn main() {
    main_loop::start::<ClockState>();
}

impl State for ClockState {
    fn init(display: &Display) -> Self {
        Self {
            center: Geometry::from_manifold(manifold::sphere(180, 90)).load(display),
            hour_marker: solids::cube().load(display),
            hand: solids::cube().load(display),
            shaders: Program::from_source(
                display,
                include_str!("../shaders/vertex.glsl"),
                include_str!("../shaders/fragment.glsl"),
                None,
            )
            .unwrap(),
            perspective: transform::reasonable_perspective(0.01, 100.0),
            navigator: Navigator::default(),
        }
    }

    fn process_event(&mut self, event: Event) {
        self.navigator.handle_event(event)
    }

    fn render(&mut self, frame: &mut Frame) {
        let scene_transform = transform::fix_aspect_ratio_y(
            frame,
            self.perspective * self.navigator.calculate_transform(),
        );

        let mut scene = Vec::new();
        scene.push((&self.center, get_center_transform()));
        for transform in get_clock_face_transforms() {
            scene.push((&self.hour_marker, transform));
        }
        scene.push((&self.hand, get_second_hand_transform()));
        scene.push((&self.hand, get_minute_hand_transform()));
        scene.push((&self.hand, get_hour_hand_transform()));

        for &(ref loaded, ref clock_element) in &scene {
            let object_transform = clock_element.0;
            let total_transform = scene_transform * object_transform;
            let normal_transform = transform::transform_normals(&object_transform);
            let uniforms = shaders::create_uniforms(
                total_transform,
                normal_transform,
                Coord3d::new(-1.0, -1.0, -1.0),
                clock_element.1,
            );
            let draw_parameters = render::default_draw_parameters();
            frame
                .draw(
                    &loaded.buffer,
                    &NoIndices(loaded.loaded_object.primitive_type),
                    &self.shaders,
                    &uniforms,
                    &draw_parameters,
                )
                .unwrap();
        }
    }
}

fn get_center_transform() -> ClockElement {
    (Trans4d::from_scale(0.1), Coord3d::new(1.0, 1.0, 1.0))
}

fn get_clock_face_transforms() -> Vec<ClockElement> {
    let mut transforms: Vec<ClockElement> = Vec::new();
    transforms.push((
        get_hour_marker_transform(0, 0.05),
        Coord3d::new(1.0, 1.0, 1.0),
    ));

    for &hour in [3, 6, 9].iter() {
        transforms.push((
            get_hour_marker_transform(hour, 0.05),
            Coord3d::new(0.33, 0.33, 0.33),
        ));
    }

    for &hour in [1, 2, 4, 5, 7, 8, 10, 11].iter() {
        transforms.push((
            get_hour_marker_transform(hour, 0.025),
            Coord3d::new(0.33, 0.33, 0.33),
        ));
    }

    transforms
}

fn get_hour_marker_transform(hour: usize, scale: f32) -> Trans4d {
    let angle = hour as f32 * -30.0;
    let constant_part =
        Trans4d::from_translation(Coord3d::new(0.0, 1.0, 0.0)) * Trans4d::from_scale(scale);
    Trans4d::from_angle_z(Deg(angle)) * constant_part
}

fn get_second_hand_transform() -> ClockElement {
    let constant_part = Trans4d::from_translation(Coord3d::new(0.0, 0.45, 0.02))
        * Trans4d::from_nonuniform_scale(0.02, 0.45, 0.01);
    (
        Trans4d::from_angle_z(second_hand_angle()) * constant_part,
        Coord3d::new(0.7, 0.7, 0.0),
    )
}

fn get_minute_hand_transform() -> ClockElement {
    let constant_part = Trans4d::from_translation(Coord3d::new(0.0, 0.35, 0.0))
        * Trans4d::from_nonuniform_scale(0.03, 0.35, 0.01);
    (
        Trans4d::from_angle_z(minute_hand_angle()) * constant_part,
        Coord3d::new(0.0, 0.0, 1.0),
    )
}

fn get_hour_hand_transform() -> ClockElement {
    let constant_part = Trans4d::from_translation(Coord3d::new(0.0, 0.25, -0.02))
        * Trans4d::from_nonuniform_scale(0.04, 0.25, 0.01);
    (
        Trans4d::from_angle_z(hour_hand_angle()) * constant_part,
        Coord3d::new(1.0, 0.0, 0.0),
    )
}

fn second_hand_angle() -> Deg<f32> {
    let now = time::now();
    -Deg(now.tm_sec as f32 * 360.0 / 60.0)
}

fn minute_hand_angle() -> Deg<f32> {
    let now = time::now();
    -Deg(now.tm_sec as f32 * 360.0 / 60.0 / 60.0 + now.tm_min as f32 * 360.0 / 60.0)
}

fn hour_hand_angle() -> Deg<f32> {
    let now = time::now();
    -Deg(now.tm_sec as f32 * 360.0 / 12.0 / 60.0 / 60.0
        + now.tm_min as f32 * 360.0 / 12.0 / 60.0
        + now.tm_hour as f32 * 360.0 / 12.0)
}
