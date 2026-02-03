use leptos::{ev, prelude::*, html};
use std::time::Duration;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"], js_name = getCurrentWindow)]
    fn get_current_window() -> Window;
    type Window;
    #[wasm_bindgen(method, js_name = startDragging)]
    fn start_dragging(this: &Window);
}

#[component]
pub fn App() -> impl IntoView {
    // 1. Create Refs for the Canvas and the hidden Source Image
    let canvas_ref = NodeRef::<html::Canvas>::new();
    let img_ref = NodeRef::<html::Img>::new();

    // Config
    let frame_w = 124.0;
    let frame_h = 93.0;
    let total_frames = 27; // Total columns in your map

    // State
    let (is_animating, set_animating) = signal(false);
    let (current_frame, set_current_frame) = signal(0);
    let (current_row, set_current_row) = signal(0);

    Effect::new(move |_| {
        // --- LOOP 1: THE BRAIN (Decides when to act) ---
        set_interval(move || {
            let r = js_sys::Math::random();
            // If idle, 10% chance to start animating
            if !is_animating.get() && r < 0.1 {
                let r_row = js_sys::Math::random();
                let random_row = (r_row * 33.0) as i32 + 1; // Random row 1-33

                set_current_row.set(random_row);
                set_current_frame.set(0);
                set_animating.set(true);
            }
        }, Duration::from_secs(1));

        // --- LOOP 2: THE PAINTER (Draws to Canvas) ---
        // Runs at ~15 FPS (60ms) to look "Retro"
        set_interval(move || {
            if let (Some(canvas), Some(img)) = (canvas_ref.get(), img_ref.get()) {
                // Get Context
                let ctx = canvas.get_context("2d")
                    .unwrap()
                    .unwrap()
                    .unchecked_into::<CanvasRenderingContext2d>();

                // CRITICAL FIX FOR LINUX:
                // Manually wipe the entire canvas area transparent.
                // This deletes the "ghosts" from the previous frame.
                ctx.clear_rect(0.0, 0.0, frame_w, frame_h);

                // Determine which sprite to show
                let row = if is_animating.get() { current_row.get() } else { 0 };
                let col = if is_animating.get() { current_frame.get() } else { 0 };

                // Draw the specific slice
                let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &img,
                    col as f64 * frame_w,      // Source X
                    row as f64 * frame_h,      // Source Y
                    frame_w, frame_h,          // Source W/H
                    0.0, 0.0, frame_w, frame_h // Dest X/Y/W/H
                );

                // Advance Frame
                if is_animating.get() {
                    if col < total_frames {
                        set_current_frame.update(|f| *f += 1);
                    } else {
                        // Animation Finished -> Go to sleep
                        set_animating.set(false);
                        set_current_row.set(0);
                    }
                }
            }
        }, Duration::from_millis(60));
    });

    view! {
        <main
            class="app-container"
            on:mousedown=move |_| {
                let window = get_current_window();
                window.start_dragging();
            }
        >
            <img
                src="public/clippy_map.png"
                node_ref=img_ref
                style="display: none;"
            />

            <canvas
                node_ref=canvas_ref
                width="124"
                height="93"
                // Disable pointer events so clicks pass through to the container
                style="pointer-events: none;"
            />
        </main>
    }
}