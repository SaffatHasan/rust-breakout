// Contains the rendering logic for a Game

use crate::model::Game;
use eframe::egui;
use eframe::epaint::{Color32, Pos2, Vec2};
use rand;
use rand::Rng;

pub struct BreakoutApp {
    game: Game,
    last_update: std::time::Instant,
}

impl BreakoutApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut rng = rand::rng();
        let mut game = Game::default();
        // update game to have same dimensions as frame via context
        // game.game_bounds = RectShape::new();
        game.game_bounds.fill = Color32::PURPLE;
        game.game_bounds.rect.min = Pos2::new(0.0, 0.0);
        game.game_bounds.rect.max = Pos2::new(400.0, 400.0);
        // Set velocity to a random direction
        let speed = 100.0;
        let angle: f32 = rng.random_range(0.0..std::f32::consts::TAU); // 0 to 2π
        game.ball.velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
        game.ball.radius = 50.0;
        game.ball.center = Pos2::new(200.0, 200.0);

        Self {
            game,
            last_update: std::time::Instant::now(),
        }
    }
}

impl eframe::App for BreakoutApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust game");
            let painter = ui.painter();
            let delta_time = self.last_update.elapsed().as_secs_f32();
            self.game.step(delta_time);

            render_game(painter, &self.game);

            self.last_update = std::time::Instant::now();
            ctx.request_repaint();
        });
    }
}

pub fn render_game(painter: &egui::Painter, game: &Game) {
    // Render background
    painter.add(game.game_bounds.clone());

    // Render ball
    painter.add(game.ball.get_shape());

    println!("{:?}, {:?}", game.ball.center, game.ball.velocity);
}
