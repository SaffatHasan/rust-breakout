use crate::model;
use eframe::epaint::{Color32, FontId};
use eframe::{egui, App};

pub struct BreakoutApp {
    game: model::game::Game,
    last_update: instant::Instant,
    initialized: bool,
}

impl BreakoutApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let app = Self {
            game: model::game::Game::default(),
            last_update: instant::Instant::now(),
            initialized: false,
        };
        app
    }
}

impl App for BreakoutApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Define the Bottom Panel for controls
        egui::TopBottomPanel::bottom("controls_panel").show(ctx, |ui| {
            ui.style_mut()
                .text_styles
                .insert(egui::TextStyle::Body, FontId::proportional(20.0));
            ui.style_mut()
                .text_styles
                .insert(egui::TextStyle::Button, FontId::proportional(20.0));
            ui.style_mut().spacing.button_padding = egui::vec2(10.0, 8.0);

            ui.centered_and_justified(|ui| {
                ui.horizontal(|ui| {
                    ui.label("BG:");
                    if ui.button("Black").clicked() {
                        self.game.game_bounds.fill = Color32::BLACK;
                    }
                    if ui.button("Blue").clicked() {
                        self.game.game_bounds.fill = Color32::from_rgb(0, 0, 100);
                    }
                    if ui.button("Green").clicked() {
                        self.game.game_bounds.fill = Color32::from_rgb(0, 50, 0);
                    }

                    ui.separator();

                    if ui.button("🔄 Reset").clicked() {
                        self.game = model::game::Game::default();
                        self.initialized = false;
                    }
                });
            });
        });

        // 2. Define the Game Area in the Central Panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // This ensures the painter draws relative to this panel's location
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::hover());

            // Update the internal game bounds to match the actual allocated screen rect
            self.game.game_bounds.rect = response.rect;

            if !self.initialized {
                self.initialized = true;
                self.game.ball.center = response.rect.center();
            }

            // Scale ball size and speed based on the available area
            let min_dim = response.rect.width().min(response.rect.height());
            let scale = min_dim / 600.0;
            self.game.ball.radius = 50.0 * scale;
            if self.game.ball.velocity.length_sq() > 0.0 {
                self.game.ball.velocity = self.game.ball.velocity.normalized() * (200.0 * scale);
            }

            let delta_time = self.last_update.elapsed().as_secs_f32();
            self.game.step(delta_time);

            // --- RENDERING ---
            // Render Background
            painter.add(self.game.game_bounds.clone());

            // Render Ball
            painter.circle_filled(
                self.game.ball.center,
                self.game.ball.radius,
                self.game.ball.fill,
            );

            // Render DVD Text
            painter.text(
                self.game.ball.center,
                egui::Align2::CENTER_CENTER,
                "DVD",
                FontId::proportional(self.game.ball.radius * 0.6),
                self.game.game_bounds.fill,
            );

            self.last_update = instant::Instant::now();
            ctx.request_repaint();
        });
    }
}
