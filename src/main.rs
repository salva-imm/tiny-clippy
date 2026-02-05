use eframe::egui;
use rand::Rng;
use std::time::{Duration, Instant};

// Constants
const FRAME_W: u32 = 124;
const FRAME_H: u32 = 93;
const TOTAL_FRAMES: u32 = 27;
const TOTAL_ROWS: u32 = 34;
const FRAME_DURATION_MS: u64 = 75;
const IDLE_CHECK_MS: u64 = 100;
const MIN_DELAY_BETWEEN_ANIMATIONS_SECS: u64 = 10; // Static delay between animations
const ANIMATION_TRIGGER_CHANCE: f32 = 1.0;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_inner_size([FRAME_W as f32, FRAME_H as f32])
            .with_resizable(false)
            .with_mouse_passthrough(false),
        ..Default::default()
    };

    eframe::run_native(
        "Tiny Clippy",
        options,
        Box::new(|cc| Ok(Box::new(ClippyApp::new(cc)))),
    )
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AnimationState {
    Idle,
    Playing { row: u32 },
    Cooldown,
}

struct Animation {
    state: AnimationState,
    current_frame: u32,
    last_frame_time: Instant,
    last_idle_check: Instant,
    last_animation_end: Instant,
}

impl Animation {
    fn new() -> Self {
        Self {
            state: AnimationState::Idle,
            current_frame: 0,
            last_frame_time: Instant::now(),
            last_idle_check: Instant::now(),
            last_animation_end: Instant::now(),
        }
    }

    fn update(&mut self) -> (u32, u32) {
        let now = Instant::now();

        if now.duration_since(self.last_frame_time) >= Duration::from_millis(FRAME_DURATION_MS) {
            self.advance_frame();
            self.last_frame_time = now;
        }

        if matches!(self.state, AnimationState::Idle | AnimationState::Cooldown) {
            if now.duration_since(self.last_idle_check) >= Duration::from_millis(IDLE_CHECK_MS) {
                self.maybe_start_animation(now);
                self.last_idle_check = now;
            }
        }

        self.get_sprite_coordinates()
    }

    fn advance_frame(&mut self) {
        match self.state {
            AnimationState::Idle | AnimationState::Cooldown => {
            }
            AnimationState::Playing { row } => {
                self.current_frame += 1;
                if self.current_frame >= TOTAL_FRAMES {
                    // Animation complete, enter cooldown
                    self.state = AnimationState::Cooldown;
                    self.current_frame = 0;
                    self.last_animation_end = Instant::now();
                }
            }
        }
    }

    fn maybe_start_animation(&mut self, now: Instant) {
        // Check if we're in cooldown
        if matches!(self.state, AnimationState::Cooldown) {
            let elapsed = now.duration_since(self.last_animation_end);
            if elapsed < Duration::from_secs(MIN_DELAY_BETWEEN_ANIMATIONS_SECS) {
                return;
            }
            self.state = AnimationState::Idle;
        }

        if matches!(self.state, AnimationState::Idle) {
            let mut rng = rand::thread_rng();

            if rng.gen::<f32>() < ANIMATION_TRIGGER_CHANCE {
                let row = rng.gen_range(1..TOTAL_ROWS);
                self.state = AnimationState::Playing { row };
                self.current_frame = 0;
            }
        }
    }

    fn get_sprite_coordinates(&self) -> (u32, u32) {
        let row = match self.state {
            AnimationState::Idle | AnimationState::Cooldown => 0,
            AnimationState::Playing { row } => row,
        };

        (self.current_frame, row)
    }

    fn is_playing(&self) -> bool {
        matches!(self.state, AnimationState::Playing { .. })
    }

    fn time_until_next_animation(&self) -> Option<Duration> {
        if matches!(self.state, AnimationState::Cooldown) {
            let elapsed = Instant::now().duration_since(self.last_animation_end);
            let required = Duration::from_secs(MIN_DELAY_BETWEEN_ANIMATIONS_SECS);
            if elapsed < required {
                return Some(required - elapsed);
            }
        }
        None
    }
}

struct ClippyApp {
    animation: Animation,
    texture: Option<egui::TextureHandle>,
    sprite_sheet: image::RgbaImage,
}

impl ClippyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load sprite sheet
        let img_bytes = include_bytes!("../clippy_map.png");
        let sprite_sheet = image::load_from_memory(img_bytes)
            .expect("Failed to load clippy sprite sheet")
            .to_rgba8();

        Self::setup_transparent_ui(&cc.egui_ctx);

        Self {
            animation: Animation::new(),
            texture: None,
            sprite_sheet,
        }
    }

    fn setup_transparent_ui(ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = egui::Color32::TRANSPARENT;
        visuals.panel_fill = egui::Color32::TRANSPARENT;
        ctx.set_visuals(visuals);
    }

    fn extract_frame(&self, frame_x: u32, frame_y: u32) -> egui::ColorImage {
        let src_x = frame_x * FRAME_W;
        let src_y = frame_y * FRAME_H;

        let sub_img = image::imageops::crop_imm(
            &self.sprite_sheet,
            src_x,
            src_y,
            FRAME_W,
            FRAME_H,
        )
            .to_image();

        let size = [sub_img.width() as usize, sub_img.height() as usize];
        let pixels = sub_img.as_flat_samples();

        egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice())
    }

    fn update_texture(&mut self, ctx: &egui::Context, frame_x: u32, frame_y: u32) {
        let color_image = self.extract_frame(frame_x, frame_y);

        self.texture = Some(ctx.load_texture(
            "clippy-frame",
            color_image,
            egui::TextureOptions::NEAREST,
        ));
    }
}

impl eframe::App for ClippyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (frame_x, frame_y) = self.animation.update();

        self.update_texture(ctx, frame_x, frame_y);

        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                if let Some(texture) = &self.texture {
                    let img_widget = egui::Image::new(texture)
                        .sense(egui::Sense::drag());

                    let response = ui.add(img_widget);

                    if response.drag_started() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                    }

                    response.context_menu(|ui| {
                        if let Some(time_left) = self.animation.time_until_next_animation() {
                            ui.label(format!("Next animation in: {:.1}s", time_left.as_secs_f32()));
                        } else {
                            ui.label("Ready for animation");
                        }

                        ui.separator();

                        if ui.button("Close Clippy").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                }
            });

        // Request repaint
        let repaint_delay = if self.animation.is_playing() {
            Duration::from_millis(16)
        } else {
            Duration::from_millis(50)
        };

        ctx.request_repaint_after(repaint_delay);
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}