mod map;
mod map_builder;
mod player;

mod prelude {
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
}

pub use prelude::*;

struct State {
    map: Map,
    player: Player,
}
impl State {
    fn new() -> Self {
        Self {
            map: Map::default(),
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
        self.player.render(ctx);
        self.player.update(ctx, &self.map);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .build()?;
    main_loop(context, State::new())
}
