use crate::platform::sdl2;
use crate::platform::opengl;

use crate::prelude::{
    qp_core::Timer,
    Registry,
    qp_data::{
        FrameState,
        FrameResponse,
        IController,
        IRenderer,
        DebugInfo
    },
    qp_profiling::Profiler,
    qp_gfx::{
        TextRenderer,
        DEFAULT_FONT
    },
    qp_ecs::{
        components::register_components,
        resources::register_resources
    }
};

pub struct QuiPi {
    pub registry: Registry,
    winapi: sdl2::QuiPiWindow,
    profiler: Profiler,

    frame_timer: Timer,
    frame_state: FrameState,

    controllers: Vec<Box<dyn IController>>,
    renderers: Vec<Box<dyn IRenderer>>
}

impl QuiPi {
    pub fn init(
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let registry = setup()?;

        let mut winapi = sdl2::QuiPiWindow::init()?;
        let _window = winapi.opengl_window(
            title,
            width,
            height,
            (4, 5)
        )?;

        let mut frame_timer = Timer::new();
        let frame_state = FrameState {
            delta: frame_timer.delta(),
            events: vec![],
            text_render: TextRenderer::new(DEFAULT_FONT)?,
            debug_mode: false,
            debug_info: DebugInfo::default(),
        };

        Ok(Self {
            registry,
            winapi,
            profiler: Profiler::new(),
            frame_timer,
            frame_state,
            controllers: vec![],
            renderers: vec![]
        })
    }

    pub fn register_controller(&mut self, controller: impl IController + 'static) {
        self.controllers.push(Box::new(controller));
    }

    pub fn register_renderer(&mut self, renderer: impl IRenderer + 'static) {
        self.renderers.push(Box::new(renderer));
    }

    pub fn run(&mut self, clear_color: (f32, f32, f32, f32)) -> Result<(), Box<dyn std::error::Error>> {
        'running: loop {
            self.registry.entities.flush();
            self.registry.flush_resources();
    
            opengl::buffer::clear_buffers(clear_color);
    
            set_frame_debug_info(&mut self.frame_state);
            self.frame_state.events = self.winapi.get_event_queue()?;

            // update controllers
            self.profiler.begin();
            for controller in self.controllers.iter_mut() {
                match controller.update(&mut self.frame_state, &mut self.registry) {
                    FrameResponse::Quit => break 'running,
                    FrameResponse::Restart => { self.frame_timer.delta(); },
                    FrameResponse::None => ()
                }
            }
            let controller_ms = self.profiler.end();

            // call renderers
            let mut draw_calls = 0;

            self.profiler.begin();
            for renderer in self.renderers.iter_mut() {
                if let Some(m_draw_calls) = renderer.draw(
                    &mut self.frame_state,
                    &mut self.registry
                ) {
                    draw_calls += m_draw_calls;
                }
            }

            if let Some(window) = &self.winapi.window {
                window.gl_swap_window();
            } else {
                return Err("There was a problem drawing the frame".into())
            }
            let render_ms = self.profiler.end();
    
            self.frame_state.debug_info.controller_ms = controller_ms as u32;
            self.frame_state.debug_info.render_ms = render_ms as u32;
            self.frame_state.debug_info.draw_calls = draw_calls;
            self.frame_state.delta = self.frame_timer.delta();
        }
    
        Ok(())
    }
}

fn setup() -> Result<Registry, Box<dyn std::error::Error>> {
    let mut registry = Registry::init()?;

    register_components(&mut registry);
    register_resources(&mut registry);

    Ok(registry)
}

pub fn set_frame_debug_info(app_state: &mut FrameState) {
    app_state.debug_info.fps = (1.0 / app_state.delta) as u32;
    app_state.debug_info.frame_ms = (app_state.delta * 1000.0) as u32;
}
