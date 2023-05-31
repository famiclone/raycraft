pub struct Debugger<'a> {
    font: sdl2::ttf::Font<'a, 'static>,
}

impl<'a> Debugger<'a> {
    pub fn new(ttf_context: &'a sdl2::ttf::Sdl2TtfContext) -> Self {
        let font_path = std::path::Path::new("assets/fonts/font.ttf");
        let font_size = 16;
        let font = ttf_context.load_font(font_path, font_size).unwrap();

        Self { font }
    }

    pub fn render_text(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        text: &str,
        position: sdl2::rect::Point,
        color: sdl2::pixels::Color,
    ) {
        canvas.set_draw_color(color);
        let surface = self.font.render(text).blended(color).unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let texture_query = texture.query();
        let texture_rect = sdl2::rect::Rect::new(
            position.x,
            position.y,
            texture_query.width,
            texture_query.height,
        );

        canvas.copy(&texture, None, Some(texture_rect)).unwrap();
    }
}
