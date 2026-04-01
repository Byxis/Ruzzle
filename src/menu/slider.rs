use raylib::prelude::*;

pub struct Slider {
    pub rect: Rectangle,
    pub value: f32,
    is_dragging: bool,
}

impl Slider {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            rect: Rectangle::new(x, y, width, height),
            value: 0.5,
            is_dragging: false,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        let mouse_pos = rl.get_mouse_position();
        let cursor_x = self.rect.x + (self.rect.width - 10.0) * self.value;
        let cursor_rect = Rectangle::new(cursor_x, self.rect.y - 5.0, 10.0, self.rect.height + 10.0);

        // Début du drag
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && cursor_rect.check_collision_point_rec(mouse_pos) {
            self.is_dragging = true;
        }

        // Fin du drag
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            self.is_dragging = false;
        }

        // Mise à jour de la valeur
        if self.is_dragging {
            let relative_x = (mouse_pos.x - self.rect.x).clamp(0.0, self.rect.width);
            self.value = (relative_x / self.rect.width).clamp(0.0, 1.0);
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        // Barre de fond
        d.draw_rectangle_rec(self.rect, Color::DARKGRAY);

        // Barre remplie
        let filled_rect = Rectangle::new(
            self.rect.x,
            self.rect.y,
            self.rect.width * self.value,
            self.rect.height,
        );
        d.draw_rectangle_rec(filled_rect, Color::GREEN);

        // Curseur
        let cursor_x = self.rect.x + (self.rect.width - 10.0) * self.value;
        d.draw_rectangle(cursor_x as i32, (self.rect.y - 5.0) as i32, 10, (self.rect.height as i32) + 10, Color::WHITE);
    }
}