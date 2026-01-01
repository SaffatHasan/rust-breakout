use crate::model::ball::Ball;

use eframe::{
    egui::{Rect, Stroke, StrokeKind},
    epaint::{Color32, Pos2, RectShape},
};

pub struct Game {
    pub ball: Ball,
    pub game_bounds: RectShape,
}

pub struct GameEvent {
    pub has_collision: bool,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            ball: Ball::default(),
            game_bounds: RectShape::new(
                Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(800.0, 600.0)),
                0.0,
                Color32::BLACK,
                Stroke::default(),
                StrokeKind::Inside,
            ),
        }
    }
}

impl Game {
    pub fn step(&mut self, delta_time: f32) -> GameEvent {
        self.ball.center.x += self.ball.velocity.x * delta_time;
        self.ball.center.y += self.ball.velocity.y * delta_time;

        let bounds = self.game_bounds.rect;
        let radius = self.ball.radius;

        let resolve_collision = |pos: &mut f32, vel: &mut f32, min: f32, max: f32| -> bool {
            if *pos - radius < min {
                *vel = -*vel;
                let over = min - (*pos - radius);
                *pos += over * 2.0;
                return true;
            } else if *pos + radius > max {
                *vel = -*vel;
                let over = max - (*pos + radius);
                *pos += over * 2.0;
                return true;
            }
            false
        };

        let has_collision = resolve_collision(
            &mut self.ball.center.x,
            &mut self.ball.velocity.x,
            bounds.min.x,
            bounds.max.x,
        ) || resolve_collision(
            &mut self.ball.center.y,
            &mut self.ball.velocity.y,
            bounds.min.y,
            bounds.max.y,
        );

        GameEvent { has_collision }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::epaint::Vec2;

    fn get_test_game() -> Game {
        let mut game = Game::default();
        // Set predictable bounds and ball size for testing
        // Bounds: 0,0 to 800,600
        game.ball.radius = 10.0;
        game
    }

    #[test]
    fn test_step_left_collision() {
        let mut game = get_test_game();
        // Position ball near left edge (x=0)
        // Radius 10. Center at 15. Left edge at 5.
        game.ball.center = Pos2::new(15.0, 300.0);
        // Moving left at 100 units/sec
        game.ball.velocity = Vec2::new(-100.0, 0.0);

        // Step 0.1s -> move -10 units.
        // New center: 5.0. Left edge: -5.0.
        // Overlap: 0.0 - (-5.0) = 5.0.
        // Correction: center += 5.0 * 2.0 = 10.0. New center = 15.0.
        let has_collision = game.step(0.1).has_collision;
        assert!(has_collision);

        assert_eq!(game.ball.center.x, 15.0);
        assert_eq!(game.ball.velocity.x, 100.0); // Velocity flipped
    }

    #[test]
    fn test_step_right_collision() {
        let mut game = get_test_game();
        // Position ball near right edge (x=800)
        // Radius 10. Center at 785. Right edge at 795.
        game.ball.center = Pos2::new(785.0, 300.0);
        // Moving right at 100 units/sec
        game.ball.velocity = Vec2::new(100.0, 0.0);

        // Step 0.1s -> move +10 units.
        // New center: 795.0. Right edge: 805.0.
        // Overlap: 800.0 - 805.0 = -5.0.
        // Correction: center += -5.0 * 2.0 = -10.0. New center = 785.0.
        let has_collision = game.step(0.1).has_collision;
        assert!(has_collision);

        assert_eq!(game.ball.center.x, 785.0);
        assert_eq!(game.ball.velocity.x, -100.0); // Velocity flipped
    }

    #[test]
    fn test_step_top_collision() {
        let mut game = get_test_game();
        // Position ball near top edge (y=0)
        // Radius 10. Center at 15. Top edge at 5.
        game.ball.center = Pos2::new(400.0, 15.0);
        // Moving up (negative Y) at 100 units/sec
        game.ball.velocity = Vec2::new(0.0, -100.0);

        // Step 0.1s -> move -10 units.
        // New center: 5.0. Top edge: -5.0.
        // Overlap: 0.0 - (-5.0) = 5.0.
        // Correction: center += 5.0 * 2.0 = 10.0. New center = 15.0.
        let has_collision = game.step(0.1).has_collision;
        assert!(has_collision);

        assert_eq!(game.ball.center.y, 15.0);
        assert_eq!(game.ball.velocity.y, 100.0); // Velocity flipped
    }

    #[test]
    fn test_step_bottom_collision() {
        let mut game = get_test_game();
        // Position ball near bottom edge (y=600)
        // Radius 10. Center at 585. Bottom edge at 595.
        game.ball.center = Pos2::new(400.0, 585.0);
        // Moving down (positive Y) at 100 units/sec
        game.ball.velocity = Vec2::new(0.0, 100.0);

        // Step 0.1s -> move +10 units.
        // New center: 595.0. Bottom edge: 605.0.
        // Overlap: 600.0 - 605.0 = -5.0.
        // Correction: center += -5.0 * 2.0 = -10.0. New center = 585.0.
        let has_collision = game.step(0.1).has_collision;
        assert!(has_collision);

        assert_eq!(game.ball.center.y, 585.0);
        assert_eq!(game.ball.velocity.y, -100.0); // Velocity flipped
    }
}
