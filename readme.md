# Layer
This was made for me, to simplify making games.

# Usage

```rust
use layer::internals::{KeyAction, KeyActionType};
use layer::run;
use layer::run::{App, Context};

struct game;

impl game {
    fn new() -> game {
        game
    }
}

impl App for game {
    fn start(&mut self, ctx: &mut Context) {}
    fn fixed_update(&mut self, ctx: &mut Context) {}
    fn update(&mut self, ctx: &mut Context, dt: f64) {}
    fn render(&mut self, ctx: &mut Context) {}
    fn handle_input(&mut self, ctx: &mut Context, key: KeyAction, action: KeyActionType) {}
}

fn main() -> Result<(), String> {
    run::run(game::new(), String::from("Example"), 640, 480)
}
```

# Contrib and feedback
Are welcome, there's a lot of _not so good_ code in here.
