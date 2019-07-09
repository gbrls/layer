/*! This module will be the entry point of the "engine"
 mod::run::Run(T) where T implements a Game trait
 The game trait will implement something like fn update(dt), fn start() fn fixed_update()...
*/

use crate::internals;
use std::collections::HashSet;

pub struct Context<'a, 'b> {
    pub video_sys: internals::Video,
    closed: bool, //pub input_sys: internals::Input

    pub font: Option<sdl2::ttf::Font<'a, 'b>>,

    pub started_time: std::time::Instant,

    // Insert returns false if that value already exists.
    pub keys_state: HashSet<sdl2::keyboard::Keycode>,

    pub mouse_state: (i32, i32),
}

impl Context<'_, '_> {
    ///Placeholder
    fn new(sdl: &sdl2::Sdl, title: String, width: u32, height: u32) -> Context {
        Context {
            video_sys: internals::Video::new(sdl, &title, width, height).unwrap(),
            closed: false,
            font: None,
            started_time: std::time::Instant::now(),

            keys_state: HashSet::new(),
            mouse_state: (0, 0),
            //input_sys: internals::Input::new(sdl).unwrap(),
        }
    }

    pub fn close(&mut self) {
        self.closed = true;
    }

    pub fn get_key(&mut self, code: &sdl2::keyboard::Keycode) -> bool {
        self.keys_state.contains(code)
    }

    pub fn draw_text_solid(
        &mut self,
        x: i32,
        y: i32,
        scale: f64,
        color: sdl2::pixels::Color,
        text: &str,
    ) {
        if let Some(_) = self.font {
            self.video_sys
                .draw_text_solid(x, y, scale, color, text, self.font.as_ref().unwrap())
        }
    }

    pub fn window_bounds(&self) -> (u32, u32) {
        self.video_sys.canvas.output_size().unwrap()
    }
}

pub trait App {
    fn start(&mut self, ctx: &mut Context) {}
    /// Return true if should close.

    fn fixed_update(&mut self, ctx: &mut Context) {}
    fn update(&mut self, ctx: &mut Context, dt: f64) {}
    fn render(&mut self, ctx: &mut Context) {}

    fn handle_input(
        &mut self,
        ctx: &mut Context,
        key: internals::KeyAction,
        action: internals::KeyActionType,
    ) {
    }
}

pub fn run<T: App>(mut game: T, title: String, width: u32, height: u32) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut input_sys = internals::Input::new(&sdl_context)?;

    let mut ctx = Context::new(&sdl_context, title, width, height);

    ctx.font = Some(
        ttf_context
            .load_font(
                std::path::Path::new("/usr/share/fonts/TTF/LiberationMono-Regular.ttf"),
                24,
            )
            .unwrap(),
    );

    game.start(&mut ctx);

    let mut reference_time = std::time::Instant::now();
    let mut debug_text = String::from("Debug");

    let mut fixed_update_reference = std::time::Instant::now();

    loop {
        if ctx.closed {
            break;
        }

        ctx.video_sys
            .canvas
            .set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));

        ctx.video_sys.clear();

        input_sys.update(&mut game, &mut ctx);

        //TODO: update_elapsed is error prone; game.update isn't frame rate independent.
        let update_elapsed = fixed_update_reference.elapsed();
        if update_elapsed > std::time::Duration::from_millis(100) {
            game.fixed_update(&mut ctx);
            fixed_update_reference = std::time::Instant::now();
        }

        game.update(&mut ctx, update_elapsed.as_secs_f64());
        game.render(&mut ctx);

        let fps_color = if ctx.get_key(&sdl2::keyboard::Keycode::LCtrl) {
            sdl2::pixels::Color::RGBA(255, 0, 0, 150)
        } else {
            sdl2::pixels::Color::RGBA(0, 255, 0, 50)
        };

        let (x, y) = ctx.window_bounds();

        ctx.draw_text_solid(5, y as i32 - 35, 1.5, fps_color, debug_text.as_str());

        ctx.video_sys.present();

        use std::thread::sleep;
        use std::time::{Duration, Instant};

        let target = Duration::from_micros(16666);
        let elapsed = reference_time.elapsed();

        if elapsed < target {
            sleep(target - elapsed);
        }

        debug_text = format!("FPS: {:.2}", 1.0 / reference_time.elapsed().as_secs_f64());

        reference_time = Instant::now();
    }

    Ok(())
}
