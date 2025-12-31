use eframe::{
    egui::{Rect, Stroke, StrokeKind},
    epaint::{CircleShape, Color32, Pos2, RectShape, Vec2},
};

pub struct Game {
    pub ball: Ball,
    pub game_bounds: RectShape,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            ball: Ball::default(),
            game_bounds: RectShape::new(
                Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(0.0, 0.0)),
                0.0,
                Color32::GREEN,
                Stroke::default(),
                StrokeKind::Inside,
            ),
        }
    }
}

impl Game {
    pub fn step(&mut self, delta_time: f32) {
        // Check if ball will collide with game bounds and reflect if necessary
        self.ball.center.x += self.ball.velocity.x * delta_time;
        self.ball.center.y += self.ball.velocity.y * delta_time;

        // check left bounds collision
        if self.ball.center.x - self.ball.radius < self.game_bounds.rect.min.x {
            self.ball
                .update_velocity(-self.ball.velocity.x, self.ball.velocity.y);

            // calculate how far out of bounds we went
            let over = self.game_bounds.rect.min.x - (self.ball.center.x - self.ball.radius);
            self.ball.center.x += over * 2.0;
        }
        // check right bounds collision
        else if self.ball.center.x + self.ball.radius > self.game_bounds.rect.max.x {
            self.ball
                .update_velocity(-self.ball.velocity.x, self.ball.velocity.y);

            // calculate how far out of bounds we went
            let over = self.game_bounds.rect.max.x - (self.ball.center.x + self.ball.radius);
            self.ball.center.x += over * 2.0;
        }

        // check top bounds collision
        if self.ball.center.y - self.ball.radius < self.game_bounds.rect.min.y {
            self.ball
                .update_velocity(self.ball.velocity.x, -self.ball.velocity.y);

            // calculate how far out of bounds we went
            let over = self.game_bounds.rect.min.y - (self.ball.center.y - self.ball.radius);
            self.ball.center.y += over * 2.0;
        }
        // check bottom bounds collision
        else if self.ball.center.y + self.ball.radius > self.game_bounds.rect.max.y {
            self.ball
                .update_velocity(self.ball.velocity.x, -self.ball.velocity.y);
            let over = self.game_bounds.rect.max.y - (self.ball.center.y + self.ball.radius);
            self.ball.center.y += over * 2.0;
        }
    }
}

pub struct Ball {
    pub center: Pos2,
    pub radius: f32,
    pub velocity: Vec2,
    pub fill: Color32,
}

// Default for ball
impl Default for Ball {
    fn default() -> Self {
        Ball::new(Pos2::new(0.0, 0.0), 1.0, Color32::RED, Vec2::new(0.0, 0.0))
    }
}

impl Ball {
    pub fn new(center: Pos2, radius: f32, fill: Color32, velocity: Vec2) -> Self {
        Self {
            center,
            radius,
            velocity,
            fill,
        }
    }

    pub fn update_velocity(&mut self, dx: f32, dy: f32) {
        self.velocity = Vec2::new(dx, dy);
    }

    pub fn get_shape(&self) -> CircleShape {
        CircleShape::filled(self.center, self.radius, self.fill)
    }
}
