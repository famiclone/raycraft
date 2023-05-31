const VERSION: &str = "0.1.1";

pub struct Renderer {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
    clear_color: sdl2::pixels::Color,
}

impl Renderer {
    pub fn new(ctx: sdl2::Sdl, width: u32, height: u32) -> Self {
        let video_subsystem = ctx.video().unwrap();

        let window = video_subsystem
            .window(
                format!("raycraft {}", VERSION).as_str(),
                width,
                height,
            )
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let event_pump = ctx.event_pump().unwrap();

        Self { canvas, event_pump, clear_color: sdl2::pixels::Color::RGB(0, 0, 0) }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(self.clear_color);
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
