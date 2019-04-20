use glium::backend::glutin::Display;
use glium::glutin::ContextBuilder;
use glium::glutin::Event;
use glium::glutin::EventsLoop;
use glium::glutin::WindowBuilder;
use glium::Frame;
use glium::Surface;

pub fn start<S: State>() {
    let window_builder = WindowBuilder::new().with_title("CG Util");
    let context_builder = ContextBuilder::new().with_multisampling(4);
    let mut events_loop = EventsLoop::new();
    let display = Display::new(window_builder, context_builder, &events_loop).unwrap();

    let mut state = S::init(&display);

    loop {
        events_loop.poll_events(|event| state.process_event(event));

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame.clear_depth(1.0);
        state.render(&mut frame);
        frame.finish().unwrap();
    }
}

pub trait State {
    fn init(display: &Display) -> Self;

    fn process_event(&mut self, event: Event);

    fn render(&mut self, display: &mut Frame);
}
