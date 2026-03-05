use std::sync::Arc;
use std::time::Instant;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::robot_face::RobotFace;
use crate::state::State;

// Custom event sent back from the async WASM init task
#[allow(dead_code)] // variant is used only on wasm32
pub enum AppEvent {
    StateReady(State),
}

// ---- Active app state (once window + GPU are ready) ----
struct ActiveApp {
    window: Arc<Window>,
    state:  State,
    face:   RobotFace,
    last_frame: Instant,
    elapsed: f32,
}

// ---- Top-level handler ----
pub struct App {
    #[allow(dead_code)] // read on wasm32 via spawn_local closure
    proxy:  EventLoopProxy<AppEvent>,
    inner:  Option<ActiveApp>,
    // WASM: window is created in resumed() but State init is async
    #[cfg(target_arch = "wasm32")]
    pending_window: Option<Arc<Window>>,
}

impl App {
    pub fn new(proxy: EventLoopProxy<AppEvent>) -> Self {
        Self {
            proxy,
            inner: None,
            #[cfg(target_arch = "wasm32")]
            pending_window: None,
        }
    }
}

impl ApplicationHandler<AppEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.inner.is_some() {
            return; // already initialized
        }

        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Robot Face — WGPU")
                        .with_inner_size(winit::dpi::LogicalSize::new(800u32, 600u32)),
                )
                .expect("Failed to create window"),
        );

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            let canvas = window.canvas().expect("Couldn't get canvas");
            canvas.set_id("wgpu-canvas");
            web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.body())
                .map(|body| body.append_child(&canvas).ok())
                .expect("Couldn't append canvas to body");

            // Async init: spawn a future that creates State and sends it back
            let window_clone = Arc::clone(&window);
            let proxy = self.proxy.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let state = State::new(Arc::clone(&window_clone)).await;
                let _ = proxy.send_event(AppEvent::StateReady(state));
            });

            self.pending_window = Some(window);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Native: block on the async init
            let state = pollster::block_on(State::new(Arc::clone(&window)));
            self.inner = Some(ActiveApp {
                window,
                state,
                face: RobotFace::new(),
                last_frame: Instant::now(),
                elapsed: 0.0,
            });
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: AppEvent) {
        match event {
            AppEvent::StateReady(state) => {
                #[cfg(target_arch = "wasm32")]
                {
                    if let Some(window) = self.pending_window.take() {
                        self.inner = Some(ActiveApp {
                            window,
                            state,
                            face: RobotFace::new(),
                            last_frame: Instant::now(),
                            elapsed: 0.0,
                        });
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    // On native this path isn't used, but avoid dead_code warning
                    let _ = state;
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(app) = self.inner.as_mut() else { return };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::Resized(new_size) => {
                app.state.resize(new_size);
            }

            WindowEvent::KeyboardInput {
                event: KeyEvent { physical_key: PhysicalKey::Code(code), state: ElementState::Pressed, .. },
                ..
            } => match code {
                KeyCode::KeyH => app.face.set_happiness(1.0),
                KeyCode::KeyN => app.face.set_happiness(0.5),
                KeyCode::KeyS => app.face.set_happiness(0.0),
                KeyCode::KeyB => app.face.trigger_blink(),
                KeyCode::Escape => event_loop.exit(),
                _ => {}
            },

            WindowEvent::RedrawRequested => {
                let now     = Instant::now();
                let delta   = now.duration_since(app.last_frame).as_secs_f32();
                app.last_frame = now;
                app.elapsed += delta;

                app.face.update(delta);

                let size    = app.state.size;
                let aspect  = size.width as f32 / size.height as f32;
                let uniforms = app.face.uniforms(app.elapsed, aspect);
                app.state.render(uniforms);
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(app) = self.inner.as_ref() {
            app.window.request_redraw();
        }
    }
}

pub fn run() {
    let event_loop = EventLoop::<AppEvent>::with_user_event()
        .build()
        .expect("Failed to build event loop");

    let proxy = event_loop.create_proxy();
    let mut app = App::new(proxy);
    event_loop.run_app(&mut app).expect("Event loop error");
}
