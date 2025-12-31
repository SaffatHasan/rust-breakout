use eframe::{
    egui::{Rect, Stroke, StrokeKind},
    epaint::{Color32, Pos2, RectShape, Vec2},
};
use rand::Rng;

pub struct Ball {
    pub center: Pos2,
    pub radius: f32,
    pub velocity: Vec2,
    pub fill: Color32,
    // List of Colors to cycle through
    pub fill_colors: Vec<Color32>,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            center: Pos2::new(400.0, 300.0),
            radius: 50.0,
            velocity: Vec2::angled(rand::rng().random_range(0.0..std::f32::consts::TAU)) * 200.0,
            fill: Color32::WHITE,
            fill_colors: vec![
                Color32::WHITE,
                Color32::RED,
                Color32::GREEN,
                Color32::BLUE,
                Color32::YELLOW,
                Color32::PURPLE,
                Color32::ORANGE,
            ],
        }
    }
}

impl Ball {
    pub fn next_color(&mut self) {
        let current = self.fill;
        let mut rng = rand::rng();
        let index = rng.random_range(0..self.fill_colors.len());
        self.fill = self.fill_colors[index];

        if self.fill == current {
            self.next_color();
        }
    }
}

pub struct Game {
    pub ball: Ball,
    pub game_bounds: RectShape,
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
    pub fn step(&mut self, delta_time: f32) {
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

        if has_collision {
            self.ball.next_color();
        }
    }
}


    }
}
