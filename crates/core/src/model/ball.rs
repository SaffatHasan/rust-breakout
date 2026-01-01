use eframe::epaint::{Color32, Pos2, Vec2};
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
            center: Pos2::new(0.0, 0.0),
            radius: 50.0,
            velocity: Vec2::angled(rand::rng().random_range(0.0..std::f32::consts::TAU)) * 200.0,
            fill: Color32::WHITE,
            fill_colors: vec![
                Color32::CYAN,
                Color32::GOLD,
                Color32::KHAKI,
                Color32::LIGHT_BLUE,
                Color32::LIGHT_GRAY,
                Color32::LIGHT_GREEN,
                Color32::LIGHT_RED,
                Color32::LIGHT_YELLOW,
                Color32::MAGENTA,
                Color32::ORANGE,
                Color32::PURPLE,
                Color32::RED,
                Color32::WHITE,
                Color32::YELLOW,
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
