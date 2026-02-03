use eframe::egui;
use std::time::{Duration, Instant};
const FRAME_W: u32 = 124;
const FRAME_H: u32 = 93;
const TOTAL_FRAMES: u32 = 27;

fn main() -> eframe::Result {
    // 1. SETUP WINDOW
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

struct ClippyApp {
    frame: u32,
    row: u32,
    last_update: Instant,
    is_animating: bool,
    texture: Option<egui::TextureHandle>,
    img_buffer: image::RgbaImage,
}

impl ClippyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let img_path = "clippy_map.png";
        let dynamic_img = image::open(img_path).expect("Failed to open clippy_map.png");
        let img_buffer = dynamic_img.to_rgba8();

        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = egui::Color32::TRANSPARENT;
        visuals.panel_fill = egui::Color32::TRANSPARENT;
        cc.egui_ctx.set_visuals(visuals);

        Self {
            frame: 0,
            row: 0,
            last_update: Instant::now(),
            is_animating: false,
            texture: None,
            img_buffer,
        }
    }
}

impl eframe::App for ClippyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let frame_duration = Duration::from_millis(66);

        if now.duration_since(self.last_update) >= frame_duration {
            if !self.is_animating {
                if rand::random::<f32>() < 0.05 {
                    self.is_animating = true;
                    self.row = (rand::random::<u32>() % 33) + 1;
                    self.frame = 0;
                }
            } else {
                self.frame += 1;
                if self.frame >= TOTAL_FRAMES {
                    self.is_animating = false;
                    self.frame = 0;
                    self.row = 0;
                }
            }
            self.last_update = now;
        }

        let src_x = self.frame * FRAME_W;
        let src_y = self.row * FRAME_H;

        let sub_img = image::imageops::crop_imm(
            &self.img_buffer,
            src_x, src_y,
            FRAME_W, FRAME_H
        ).to_image();

        let size = [sub_img.width() as usize, sub_img.height() as usize];
        let pixels = sub_img.as_flat_samples();
        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
        );

        self.texture = Some(ctx.load_texture(
            "clippy-frame",
            color_image,
            egui::TextureOptions::NEAREST,
        ));

        egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| {
            if let Some(texture) = &self.texture {
                let img_widget = egui::Image::new(texture)
                    .sense(egui::Sense::drag());

                let response = ui.add(img_widget);

                if response.dragged() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
            }
        });

        ctx.request_repaint_after(Duration::from_millis(30));
    }
}