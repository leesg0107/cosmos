use eframe::egui;
use rand::random;

pub struct Particle {
    pos: egui::Pos2,
    velocity: egui::Vec2,
    life: f32,
    color: egui::Color32,
    size: f32,
    trail: Vec<egui::Pos2>,
}

impl Particle {
    pub fn new(pos: egui::Pos2) -> Self {
        let angle = random::<f32>() * std::f32::consts::TAU;
        let speed = random::<f32>() * 10.0 + 5.0;
        Self {
            pos,
            velocity: egui::vec2(angle.cos() * speed, angle.sin() * speed),
            life: 1.0,
            color: Self::random_bright_color(),
            size: random::<f32>() * 5.0 + 2.0,
            trail: vec![pos],
        }
    }

    fn random_bright_color() -> egui::Color32 {
        let hue = random::<f32>() * 360.0;
        let saturation = 0.8 + random::<f32>() * 0.2;
        let value = 0.8 + random::<f32>() * 0.2;

        let h = hue / 60.0;
        let i = h.floor() as i32;
        let f = h - i as f32;
        let p = value * (1.0 - saturation);
        let q = value * (1.0 - saturation * f);
        let t = value * (1.0 - saturation * (1.0 - f));

        let (r, g, b) = match i % 6 {
            0 => (value, t, p),
            1 => (q, value, p),
            2 => (p, value, t),
            3 => (p, q, value),
            4 => (t, p, value),
            _ => (value, p, q),
        };

        egui::Color32::from_rgb(
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
        )
    }

    pub fn update(&mut self) {
        self.velocity *= 1.02;
        self.pos += self.velocity;
        self.life -= 0.01;
        
        self.trail.push(self.pos);
        if self.trail.len() > 10 {
            self.trail.remove(0);
        }
    }

    pub fn is_alive(&self) -> bool {
        self.life > 0.0
    }

    pub fn draw(&self, painter: &egui::Painter) {
        for i in 1..self.trail.len() {
            let alpha = (i as f32 / self.trail.len() as f32) * self.life;
            let trail_color = self.color.linear_multiply(alpha * 0.5);
            painter.line_segment(
                [self.trail[i-1], self.trail[i]],
                egui::Stroke::new(self.size * alpha, trail_color),
            );
        }

        painter.circle_filled(
            self.pos,
            self.size,
            self.color.linear_multiply(self.life),
        );
    }
} 