use leptos::prelude::*;
use std::time::Duration;
use leptos::html;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

// --- TAURI DRAG BINDING ---
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
    // 1. Setup Refs
    let canvas_ref = NodeRef::<html::Canvas>::new();
    let img_ref = NodeRef::<html::Img>::new();

    // 2. Config (Map Size)
    let frame_w = 124.0;
    let frame_h = 93.0;
    let total_frames = 27; // Total columns

    // 3. State
    let (is_animating, set_animating) = signal(false);
    let (current_frame, set_current_frame) = signal(0);
    let (current_row, set_current_row) = signal(0);

    Effect::new(move |_| {
        // --- HEARTBEAT LOOP (The Brain) ---
        // Runs every 1 second to decide "Should I act?"
        set_interval(move || {
            let r = js_sys::Math::random();
            // If idle, 10% chance to start animating
            if !is_animating.get() && r < 0.1 {
                let r_row = js_sys::Math::random();
                let random_row = (r_row * 33.0) as i32 + 1; // Rows 1-33

                set_current_row.set(random_row);
                set_current_frame.set(0);
                set_animating.set(true);
            }
        }, Duration::from_secs(1));

        // --- RENDER LOOP (The Painter) ---
        // Runs at ~15 FPS (60ms)
        set_interval(move || {
            // A. THE LINUX "MICRO-FLICKER" FIX
            // We imperceptibly change the background opacity to force the OS
            // to repaint the entire transparent window, wiping any ghosts.
            if let Some(window) = web_sys::window() {
                if let Some(doc) = window.document() {
                    if let Some(body) = doc.body() {
                        let current_bg = body.style().get_property_value("background-color").unwrap_or_default();
                        // Toggle between 1% and 1.1% opacity
                        if current_bg.contains("0.01)") {
                            let _ = body.style().set_property("background-color", "rgba(0, 0, 0, 0.011)");
                        } else {
                            let _ = body.style().set_property("background-color", "rgba(0, 0, 0, 0.01)");
                        }
                    }
                }
            }

            // B. THE CANVAS ATOMIC DRAW
            if let (Some(canvas), Some(img)) = (canvas_ref.get(), img_ref.get()) {
                let ctx = canvas.get_context("2d")
                    .unwrap()
                    .unwrap()
                    .unchecked_into::<CanvasRenderingContext2d>();

                // 1. ATOMIC COPY (Nukes old pixels instantly)
                let _ = ctx.set_global_composite_operation("copy");

                let row = if is_animating.get() { current_row.get() } else { 0 };
                let col = if is_animating.get() { current_frame.get() } else { 0 };

                // 2. Draw Slice
                let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &img,
                    col as f64 * frame_w,      // Source X
                    row as f64 * frame_h,      // Source Y
                    frame_w, frame_h,          // Source Size
                    0.0, 0.0, frame_w, frame_h // Dest Size
                );

                // 3. Reset Composite (Safety)
                let _ = ctx.set_global_composite_operation("source-over");

                // 4. Advance Frame
                if is_animating.get() {
                    if col < total_frames {
                        set_current_frame.update(|f| *f += 1);
                    } else {
                        // Done
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
                style="pointer-events: none;"
            />
        </main>
    }
}