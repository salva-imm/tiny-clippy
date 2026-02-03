use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::{Duration, Instant};
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, WindowLevel}, // <--- Added WindowLevel
};
use image::GenericImageView;

// CONFIG
const FRAME_W: u32 = 124;
const FRAME_H: u32 = 93;
const TOTAL_FRAMES: u32 = 27;

struct AppState {
    frame: u32,
    row: u32,
    last_update: Instant,
    is_animating: bool,
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // 1. Load Sprite
    let img_path = "clippy_map.png";
    let dynamic_img = image::open(img_path).expect("Failed to open clippy_map.png");
    let img_width = dynamic_img.width();
    let img_buffer = dynamic_img.to_rgba8();

    // 2. Setup Window
    let window = Rc::new(
        WindowBuilder::new()
            .with_title("Tiny Clippy")
            .with_decorations(false)
            .with_transparent(true)
            // FIX 1: New API for "Always on Top"
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_inner_size(winit::dpi::LogicalSize::new(FRAME_W as f64, FRAME_H as f64))
            .build(&event_loop)
            .unwrap()
    );

    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    let mut state = AppState {
        frame: 0,
        row: 0,
        last_update: Instant::now(),
        is_animating: false,
    };

    event_loop.run(move |event, elwt| {
        let frame_duration = Duration::from_millis(66); // ~15 FPS
        elwt.set_control_flow(ControlFlow::WaitUntil(Instant::now() + frame_duration));

        match event {
            // A. DRAGGING
            Event::WindowEvent { event: WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, window_id }
            if window_id == window.id() => {
                let _ = window.drag_window();
            }

            // B. REDRAW
            Event::WindowEvent { window_id, event: WindowEvent::RedrawRequested }
            if window_id == window.id() => {
                let now = Instant::now();

                // Logic
                if now.duration_since(state.last_update) >= frame_duration {
                    if !state.is_animating {
                        if rand::random::<f32>() < 0.05 {
                            state.is_animating = true;
                            state.row = (rand::random::<u32>() % 33) + 1;
                            state.frame = 0;
                        }
                    } else {
                        state.frame += 1;
                        if state.frame >= TOTAL_FRAMES {
                            state.is_animating = false;
                            state.frame = 0;
                            state.row = 0;
                        }
                    }
                    state.last_update = now;
                }

                // Drawing
                let (width, height) = (FRAME_W, FRAME_H);

                // Check for minimized window to prevent crash
                if width > 0 && height > 0 {
                    surface.resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    ).unwrap();

                    let mut buffer = surface.buffer_mut().unwrap();
                    buffer.fill(0); // Nuke ghosts

                    let src_start_x = state.frame * FRAME_W;
                    let src_start_y = state.row * FRAME_H;

                    for y in 0..height {
                        for x in 0..width {
                            let src_x = src_start_x + x;
                            let src_y = src_start_y + y;

                            if src_x < img_width {
                                let pixel = img_buffer.get_pixel(src_x, src_y);
                                if pixel[3] > 10 { // Alpha Threshold
                                    let r = pixel[0] as u32;
                                    let g = pixel[1] as u32;
                                    let b = pixel[2] as u32;
                                    let color = (r << 16) | (g << 8) | b;

                                    let dest_idx = (y * width) + x;
                                    if dest_idx < buffer.len() as u32 {
                                        buffer[dest_idx as usize] = color;
                                    }
                                }
                            }
                        }
                    }
                    buffer.present().unwrap();
                }
            }

            // FIX 2: "MainEventsCleared" renamed to "AboutToWait" in 0.29
            Event::AboutToWait => {
                window.request_redraw();
            }

            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                elwt.exit();
            }
            _ => {}
        }
    }).unwrap();
}