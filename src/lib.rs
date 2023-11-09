use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

pub struct WindowSettings {
    pub title: String,
    pub size: (u32, u32),
    pub gl_version: (u8, u8),
}

pub struct ProcLoader<'a> {
    windowed_context: &'a glutin::WindowedContext<glutin::PossiblyCurrent>,
}

impl ProcLoader<'_> {
    pub fn get_proc_address(&self, s: &str) -> *const std::ffi::c_void {
        self.windowed_context.get_proc_address(s) as *const _
    }
}

pub enum Event<'a> {
    WindowInitialized(ProcLoader<'a>),
    Resized((u32, u32)),
    RedrawRequested,
}

pub fn run<F>(window_settings: WindowSettings, event_handler: F)
where
    F: 'static + FnMut(Event),
{
    let mut event_handler = event_handler;

    let el = EventLoop::new();
    let wb = WindowBuilder::new();
    let wb = wb.with_title(window_settings.title);

    let inner_size = glutin::dpi::LogicalSize::new(window_settings.size.0, window_settings.size.1);
    let wb = wb.with_inner_size(inner_size);

    let windowed_context = ContextBuilder::new();
    let windowed_context = windowed_context.with_gl_profile(glutin::GlProfile::Core);
    let windowed_context = windowed_context.with_gl(glutin::GlRequest::Specific(
        glutin::Api::OpenGl,
        window_settings.gl_version,
    ));

    let windowed_context = windowed_context.build_windowed(wb, &el).unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    event_handler(Event::WindowInitialized(ProcLoader {
        windowed_context: &windowed_context,
    }));

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        use glutin::event::Event as Ev;
        use glutin::event::WindowEvent as WinEv;

        match event {
            Ev::LoopDestroyed => (),
            Ev::WindowEvent { event, .. } => match event {
                WinEv::Resized(physical_size) => {
                    windowed_context.resize(physical_size);
                    event_handler(Event::Resized(physical_size.into()));
                }
                WinEv::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Ev::RedrawRequested(_) => {
                event_handler(Event::RedrawRequested);

                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
