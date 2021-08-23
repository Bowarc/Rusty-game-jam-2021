use ggez::{
    self,
    conf::{self},
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{self, Color},
    Context, ContextBuilder, GameError, GameResult,
};

mod map;

struct Game {}

impl Game {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Game {})
    }
}
impl EventHandler<GameError> for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgba(100, 100, 100, 1));

        graphics::present(ctx);
        Ok(())
    }
}

fn main() -> GameResult {
    println!("Hello, world!");

    let cb = ContextBuilder::new("test game 1", "ggez")
        .window_setup(conf::WindowSetup::default().title("The game title"))
        .window_mode(conf::WindowMode::default().dimensions(640., 480.));
    // maybe resource dir

    let (mut ctx, events_loop) = cb.build()?;

    let game = Game::new(&mut ctx)?;
    event::run(ctx, events_loop, game)
}
