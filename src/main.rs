use rltk::{ GameState, Rltk, RGB };
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
mod rect;
pub use rect::Rect;

/// STATE ///
/// ///// ///
pub struct State {
    world: World,
    game_seconds: u32,
    frame_time: u8,
    cap: u8
}
impl GameState for State {
    fn tick(&mut self, context : &mut Rltk) {
        // update systems
        player_input(self, context);
        self.update_systems();

        // render time
        context.cls();
        context.print(1, 1, "Time: ");
        context.print(7, 1, self.game_seconds);
        context.print(10, 1, self.frame_time);
        self.frame_time += 1;
        if self.frame_time > self.cap {
            self.frame_time = 0;
            self.game_seconds += 1
        }

        // render map
        let map = self.world.fetch::<Vec<TileType>>();
        draw_map(&map, context);

        // render entities
        let transform_datas = self.world.read_storage::<TransformData>();
        let render_datas = self.world.read_storage::<RenderData>();
        for (pos, render) in (&transform_datas, &render_datas).join() {
            context.set(pos.x, pos.y, render.foreground, render.background, render.glyph);
        }
    }
}
impl State {
    fn update_systems(&mut self) {
        self.world.maintain();
    }
}

/// MAIN ///
/// //// ///
fn main() -> rltk::BError {
    // init
    use rltk::RltkBuilder;
    let fps_cap = 30;
    let mut game_state = State {
        world: World::new(),
        game_seconds: 0,
        frame_time: 0,
        cap: fps_cap
    };
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .with_fps_cap(fps_cap.into())
        .build()?;

    // register components
    game_state.world.register::<TransformData>();
    game_state.world.register::<RenderData>();
    game_state.world.register::<Player>();

    // insert resources
    game_state.world.insert(new_map_test());

    // add entities
    game_state.world.create_entity()
        .with(TransformData { x: 40, y: 25 })
        .with(RenderData {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK)
        })
        .with(Player {})
        .build();

    // fire off main loop
    rltk::main_loop(context, game_state)
}
