#![cfg(target_arch = "wasm32")]
use crate::render::{camera::OrbitCamera, state::State};
use wasm_bindgen::{prelude::*, JsCast};
use js_sys::{Array, Reflect, Float32Array};

#[wasm_bindgen]
extern "C" {
    /// Log a string value to the console.
    #[allow(unused)]
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Renders to a canvas using the `wgpu` **WebGL2** backend.
#[wasm_bindgen]
pub struct WebGLRenderer {
    state: State,
}

#[wasm_bindgen]
impl WebGLRenderer {
    /// Create a new [WebGLRenderer] instance for the given canvas id.
    #[wasm_bindgen(constructor)]
    pub fn create(canvas_id: &str, width: u32, height: u32) -> Self {
        // Log rust panics to browser console. Debug only!
        console_error_panic_hook::set_once();

        // Get the browser window.
        let window = web_sys::window().expect("Can not get browser window.");

        // Get the dom document.
        let document = window.document().expect("Can not get html document.");

        // Get the canvas with the given id.
        let canvas = document
            .get_element_by_id(canvas_id)
            .expect("The given canvas id was not found in the document.");

        // Canvas need to be of type HtmlCanvasElement.
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("Failed to convert element to canvas.");

        // The numeric id for the `raw-window_handle` is retrieved from the id of
        // the canvas minus the first 4 chars.
        let mut id = canvas.id();
        id.replace_range(0..4, "");

        // The numeric id in the `raw_window_handle::web::WebHandle` has to be set
        // inside the `data-raw-handle` attribute of the canvas.
        canvas
            .set_attribute("data-raw-handle", id.as_str())
            .expect("Failed to assign numeric raw-window-handle id to canvas.");

        let id: u32 = id
            .parse()
            .expect("Failed to get numeric sequence from canvas id.");

        let mut window_handle = raw_window_handle::WebWindowHandle::empty();
        window_handle.id = id;

        let display_handle = raw_window_handle::WebDisplayHandle::empty();

        let wgpu_canvas = WgpuCanvas {
            display_handle,
            window_handle
        };

        let mut camera = OrbitCamera::new(
            2.0,
            1.5,
            1.25,
            glam::Vec3::new(0.0, 0.0, 0.0),
            width as f32 / height as f32,
        );
        camera.bounds.min_distance = Some(1.1);

        // State::new uses async code, so we're going to wait for it to finish
        let state = pollster::block_on(State::new(&wgpu_canvas, width, height, camera));

        Self { state }
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        self.state.update();
    }

    #[wasm_bindgen]
    pub fn update_box_size(&mut self, width: f32, height: f32, depth: f32) {
        let vec = glam::Vec3::new(width, height, depth);
        self.state.update_box_size(vec);
    }

    #[wasm_bindgen]
    pub fn render(&mut self) {
        self.state.render().expect("Failed to render.");
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        log(format!("Resized to: {} x {}", new_width, new_height).as_str());
        self.state.resize(new_width, new_height);
    }

    #[wasm_bindgen]
    pub fn add_distance(&mut self, delta: f32) {
        self.state.camera.add_distance(delta);
    }

    #[wasm_bindgen]
    pub fn add_pitch(&mut self, delta: f32) {
        self.state.camera.add_pitch(delta);
    }

    #[wasm_bindgen]
    pub fn add_yaw(&mut self, delta: f32) {
        self.state.camera.add_yaw(delta);
    }
}

/// Implement [raw_window_handle::HasRawWindowHandle] and
/// [raw_window_handle::HasRawDisplayHandle] for [web_sys::HtmlCanvasElement].
struct WgpuCanvas {
    display_handle: raw_window_handle::WebDisplayHandle,

    window_handle: raw_window_handle::WebWindowHandle,
}

unsafe impl raw_window_handle::HasRawWindowHandle for WgpuCanvas {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        raw_window_handle::RawWindowHandle::Web(self.window_handle)
    }
}

unsafe impl raw_window_handle::HasRawDisplayHandle for WgpuCanvas {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        raw_window_handle::RawDisplayHandle::Web(self.display_handle)
    }
}
