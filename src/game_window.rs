use sdl2::render::{Texture, WindowCanvas};

pub struct GameWindow {
    pub canvas: WindowCanvas,
}

impl GameWindow {
    pub fn new(canvas: WindowCanvas) -> GameWindow {
        GameWindow { canvas }
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn render_texture(&mut self, texture: &Texture) {
        self.canvas.copy(texture, None, None).unwrap();
    }
}
