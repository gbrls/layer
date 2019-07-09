pub struct Video {
    video_sys: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::WindowCanvas,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
}

impl Video {
    pub fn new(
        sdl_context: &sdl2::Sdl,
        title: &String,
        width: u32,
        height: u32,
    ) -> Result<Video, String> {
        let vs = sdl_context.video()?;

        let w = vs.window(title.as_str(), width, height).build().unwrap();
        let mut c = w.into_canvas().build().unwrap();

        let tc = c.texture_creator();

        use sdl2::pixels::Color;
        c.set_draw_color(Color::RGB(30, 30, 30));

        Ok(Video {
            video_sys: vs,
            canvas: c,
            texture_creator: tc,
        })
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    ///TODO: better font support;
    pub fn draw_text_solid(
        &mut self,
        x: i32,
        y: i32,
        scale: f64,
        color: sdl2::pixels::Color,
        text: &str,
        //font: &sdl2::ttf::Font,
        font: &sdl2::ttf::Font,
    ) {
        //let font = ttf_context
        //.load_font(
        //std::path::Path::new("/usr/share/fonts/TTF/LiberationMono-Regular.ttf"),
        //20,
        //)
        //.unwrap();

        let surface = font.render(text).solid(color).unwrap();

        let text_w = (surface.width() as f64 * scale) as u32;
        let text_h = (surface.height() as f64 * scale) as u32;

        let texture = self
            .texture_creator
            .create_texture_from_surface(surface)
            .unwrap();
        use sdl2::rect::Rect;

        self.canvas
            .copy(&texture, None, Some(Rect::new(x, y, text_w, text_h)))
            .unwrap();
    }
}
