use minifb::{Window, WindowOptions, Scale, MouseMode, MouseButton};
use std::time::{Duration, Instant};
use image::GenericImageView;

// CONFIG
const FRAME_W: usize = 124;
const FRAME_H: usize = 93;
const TOTAL_FRAMES: u32 = 27;

struct AppState {
    frame: u32,
    row: u32,
    last_update: Instant,
    is_animating: bool,
    // Dragging State
    is_dragging: bool,
    drag_start_x: f32,
    drag_start_y: f32,
}

fn main() {
    // 1. Load Sprite
    let img_path = "clippy_map.png";
    let dynamic_img = image::open(img_path).expect("Failed to open clippy_map.png");
    let img_width = dynamic_img.width();
    let img_buffer = dynamic_img.to_rgba8();

    // 2. Setup Window
    let mut window = Window::new(
        "Tiny Clippy",
        FRAME_W,
        FRAME_H,
        WindowOptions {
            borderless: true,
            transparency: true,    // <--- FIXED: It is 'transparency', not 'transparent'
            topmost: true,
            resize: false,
            scale: Scale::X1,
            ..WindowOptions::default()
        },
    )
        .expect("Unable to create window");

    // Start position
    window.set_position(100, 100);

    let mut buffer: Vec<u32> = vec![0; FRAME_W * FRAME_H];

    let mut state = AppState {
        frame: 0,
        row: 0,
        last_update: Instant::now(),
        is_animating: false,
        is_dragging: false,
        drag_start_x: 0.0,
        drag_start_y: 0.0,
    };

    let frame_duration = Duration::from_millis(66);

    // 3. Loop
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        let now = Instant::now();

        // --- DRAGGING LOGIC ---
        // Get mouse position relative to the window window
        if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Pass) {
            let left_down = window.get_mouse_down(MouseButton::Left);

            if left_down {
                if !state.is_dragging {
                    // Start Drag: Record where we clicked inside the window
                    state.is_dragging = true;
                    state.drag_start_x = mx;
                    state.drag_start_y = my;
                } else {
                    // Continue Drag: Move window by the difference
                    // If mouse moved right, we move window right to keep mouse under cursor
                    let (wx, wy) = window.get_position();
                    let dx = mx - state.drag_start_x;
                    let dy = my - state.drag_start_y;

                    window.set_position(
                        (wx as f32 + dx) as isize,
                        (wy as f32 + dy) as isize
                    );
                }
            } else {
                state.is_dragging = false;
            }
        }

        // --- ANIMATION ---
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

            // --- DRAWING ---
            // Clear buffer to 0 (Transparent)
            for pixel in buffer.iter_mut() { *pixel = 0; }

            let src_start_x = state.frame * FRAME_W as u32;
            let src_start_y = state.row * FRAME_H as u32;

            for y in 0..FRAME_H {
                for x in 0..FRAME_W {
                    let src_x = src_start_x + x as u32;
                    let src_y = src_start_y + y as u32;

                    if src_x < img_width {
                        let pixel = img_buffer.get_pixel(src_x, src_y);

                        // Check Alpha (pixel[3])
                        if pixel[3] > 10 {
                            let r = pixel[0] as u32;
                            let g = pixel[1] as u32;
                            let b = pixel[2] as u32;
                            // Pack 0x00RRGGBB
                            buffer[y * FRAME_W + x] = (r << 16) | (g << 8) | b;
                        }
                    }
                }
            }

            // Push to window
            window.update_with_buffer(&buffer, FRAME_W, FRAME_H).unwrap();
        } else {
            // Update even if not drawing new frame (handles input/drag smoothness)
            window.update_with_buffer(&buffer, FRAME_W, FRAME_H).unwrap();
        }

        // Short sleep to prevent 100% CPU usage
        std::thread::sleep(Duration::from_millis(5));
    }
}