use ggez::{
    self,
    conf::{self},
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{self, Color},
    Context, ContextBuilder, GameError, GameResult,
};

mod input;
mod map;
mod physics;
mod player;

struct Game {
    map: map::Map,
}

impl Game {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut id = 0;
        // set the tile size
        let tile_size = 10.;

        // load the map
        let mut map = map::Map::new(tile_size, &mut id);
        map.load_new_map("game_jam_map_test_1".to_string(), ctx);
        Ok(Game { map: map })
    }
}
impl EventHandler<GameError> for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgba(100, 100, 100, 255));

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
