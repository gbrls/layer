use crate::run::{App, Context};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyAction {
    Accept,
    Back,
    Up,
    Down,
    Left,
    Right,
    MouseMotion(i32, i32),
    MouseDragged(i32, i32),
    MouseButton(sdl2::mouse::MouseButton),
    MouseWheel(i32),

    DebugKey(sdl2::keyboard::Keycode),
    DebugToggle,
    DebugKey1,
    DebugKey2,
    DebugKey3,
    DebugPlus,
    DebugMinus,

    Key(sdl2::keyboard::Keycode),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyActionType {
    KeyUp,
    KeyDown,
    KeyJustPressed,
    AxisMotion,

    BtnJustPressed,
    BtnUp,
}

pub struct Input {
    event_pump: sdl2::EventPump,
    controller_sys: sdl2::GameControllerSubsystem,
}

impl Input {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Input, String> {
        let ep = sdl_context.event_pump()?;

        let cs = sdl_context.game_controller()?;

        Ok(Input {
            event_pump: ep,
            controller_sys: cs,
        })
    }

    fn map_mouse_button(button: sdl2::mouse::MouseButton) -> sdl2::keyboard::Keycode {
        use sdl2::keyboard::Keycode;
        use sdl2::mouse::MouseButton;

        match button {
            MouseButton::Left => Keycode::Www,
            _ => Keycode::Mail,
        }
    }

    fn map_to_key_action(sdl_keycode: sdl2::keyboard::Keycode) -> KeyAction {
        use sdl2::keyboard::Keycode;
        match sdl_keycode {
            Keycode::F1 => KeyAction::DebugToggle,
            Keycode::Space => KeyAction::Accept,
            Keycode::D => KeyAction::Right,
            Keycode::A => KeyAction::Left,
            Keycode::W => KeyAction::Up,
            Keycode::S => KeyAction::Down,
            _ => KeyAction::Key(sdl_keycode),
        }
    }

    fn map_to_debug_key(sdl_keycode: sdl2::keyboard::Keycode) -> KeyAction {
        use sdl2::keyboard::Keycode;
        match sdl_keycode {
            Keycode::Num1 => KeyAction::DebugKey1,
            Keycode::Num2 => KeyAction::DebugKey2,
            Keycode::Num3 => KeyAction::DebugKey3,
            Keycode::Equals => KeyAction::DebugPlus,
            Keycode::Minus => KeyAction::DebugMinus,
            _ => KeyAction::DebugKey(sdl_keycode),
        }
    }

    /// Return true if recieved close event.
    pub fn update<T: App>(&mut self, app: &mut T, ctx: &mut Context) {
        let x = self.event_pump.mouse_state().x();
        let y = self.event_pump.mouse_state().y();

        ctx.mouse_state = (x, y);

        for event in self.event_pump.poll_iter() {
            use sdl2::{event::Event, keyboard::Keycode};
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => ctx.close(),

                Event::KeyDown {
                    keycode: Some(k),
                    repeat: r,
                    ..
                } => {
                    ctx.keys_state.insert(k);

                    let action = if r {
                        KeyActionType::KeyDown
                    } else {
                        KeyActionType::KeyJustPressed
                    };

                    if ctx.get_key(&Keycode::LCtrl) {
                        app.handle_input(ctx, Input::map_to_debug_key(k), action);
                    }

                    if !r {
                        app.handle_input(ctx, Input::map_to_key_action(k), action);
                    }
                }

                Event::KeyUp {
                    keycode: Some(k), ..
                } => {
                    app.handle_input(ctx, Input::map_to_key_action(k), KeyActionType::KeyUp);
                    ctx.keys_state.remove(&k);
                }

                Event::MouseMotion { x, y, .. } => {
                    if ctx.get_key(&Input::map_mouse_button(sdl2::mouse::MouseButton::Left)) {
                        app.handle_input(
                            ctx,
                            KeyAction::MouseDragged(x, y),
                            KeyActionType::AxisMotion,
                        );
                    } else {
                        app.handle_input(
                            ctx,
                            KeyAction::MouseMotion(x, y),
                            KeyActionType::AxisMotion,
                        );
                    }
                }

                Event::MouseButtonDown { mouse_btn: btn, .. } => {
                    ctx.keys_state.insert(Input::map_mouse_button(btn));

                    app.handle_input(
                        ctx,
                        KeyAction::MouseButton(btn),
                        KeyActionType::BtnJustPressed,
                    );
                }

                Event::MouseButtonUp { mouse_btn: btn, .. } => {
                    ctx.keys_state.remove(&Input::map_mouse_button(btn));

                    app.handle_input(ctx, KeyAction::MouseButton(btn), KeyActionType::BtnUp);
                }

                Event::MouseWheel { y, .. } => {
                    app.handle_input(ctx, KeyAction::MouseWheel(y), KeyActionType::AxisMotion);
                }

                _ => {}
            }
        }
    }
}
