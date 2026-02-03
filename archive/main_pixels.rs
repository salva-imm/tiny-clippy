use std::rc::Rc;
use std::time::{Duration, Instant};
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, WindowLevel}, // <--- Import WindowLevel
    dpi::LogicalSize,
};
use pixels::{Pixels, SurfaceTexture};
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
    let event_loop = EventLoop::new();

    // 1. Load Sprite
    let img_path = "clippy_map.png";
    let dynamic_img = image::open(img_path).expect("Failed to open clippy_map.png");
    let img_width = dynamic_img.width();
    let img_buffer = dynamic_img.to_rgba8();

    // 2. Setup Window
    let size = LogicalSize::new(FRAME_W as f64, FRAME_H as f64);
    let window = Rc::new(
        WindowBuilder::new()
            .with_title("Tiny Clippy")
            .with_decorations(false)
            .with_transparent(true)
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    );

    // FIX: Use the 0.28 API for Always On Top
    window.set_window_level(WindowLevel::AlwaysOnTop);

    // 3. Setup GPU Pixels
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &*window);

    // Safety: If this unwrap fails, it means the window is invisible or 0 size
    let mut pixels = Pixels::new(FRAME_W, FRAME_H, surface_texture).unwrap();
    pixels.clear_color(pixels::wgpu::Color::TRANSPARENT);

    let mut state = AppState {
        frame: 0,
        row: 0,
        last_update: Instant::now(),
        is_animating: false,
    };

    // 4. Run Loop
    event_loop.run(move |event, _, control_flow| {
        let frame_duration = Duration::from_millis(66);
        *control_flow = ControlFlow::WaitUntil(Instant::now() + frame_duration);

        match event {
            // A. Dragging
            Event::WindowEvent { event: WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. }, window_id }
            if window_id == window.id() => {
                let _ = window.drag_window();
            }

            // B. Redraw (Top Level in 0.28)
            Event::RedrawRequested(window_id)
            if window_id == window.id() => {
                let now = Instant::now();

                // Animation Logic
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

                // Drawing (GPU)
                let frame = pixels.frame_mut();

                // 1. Clear to Transparent
                for pixel in frame.chunks_exact_mut(4) {
                    pixel.copy_from_slice(&[0, 0, 0, 0]);
                }

                // 2. Draw Sprite
                let src_start_x = state.frame * FRAME_W;
                let src_start_y = state.row * FRAME_H;

                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let x = (i as u32) % FRAME_W;
                    let y = (i as u32) / FRAME_W;

                    let src_x = src_start_x + x;
                    let src_y = src_start_y + y;

                    if src_x < img_width {
                        let sprite_pixel = img_buffer.get_pixel(src_x, src_y);
                        if sprite_pixel[3] > 10 {
                            pixel.copy_from_slice(&sprite_pixel.0);
                        }
                    }
                }

                // 3. Render
                if let Err(_) = pixels.render() {
                    *control_flow = ControlFlow::Exit;
                }
            }

            // Resize
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                if let Err(_) = pixels.resize_surface(size.width, size.height) {
                    *control_flow = ControlFlow::Exit;
                }
            }

            // Trigger Redraw
            Event::MainEventsCleared => {
                window.request_redraw();
            }

            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}