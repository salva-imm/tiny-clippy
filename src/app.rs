use leptos::prelude::*;
use std::time::Duration;
use wasm_bindgen::prelude::*;

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
    let (row, set_row) = signal(0);
    let total_rows = 34;
    let total_cols = 27;

    Effect::new(move |_| {
        let _ = set_interval(
            move || {
                if row.get() == 0 {
                    let r = js_sys::Math::random();

                    if r < 0.1 {
                        let r_row = js_sys::Math::random();
                        let random_row = (r_row * (total_rows - 1) as f64) as i32 + 1;

                        set_row.set(random_row);

                        let _ = set_timeout(
                            move || set_row.set(0),
                            Duration::from_secs(4)
                        );
                    }
                }
            },
            Duration::from_secs(1),
        );
    });

    view! {
        <main
            class="app-container"
            on:mousedown=move |_| {
                let window = get_current_window();
                window.start_dragging();
            }
        >
            <div
                class="clippy"
                style:--row=move || row.get().to_string()
                style:--frames=total_cols.to_string()
                style:animation-play-state=move || if row.get() == 0 { "paused" } else { "running" }
            ></div>
        </main>
    }
}