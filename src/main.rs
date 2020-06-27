use rltk::{ GameState, Rltk };
use specs::prelude::*;

mod colours;
pub use colours::*;
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
    world: World
}
impl GameState for State {
    fn tick(&mut self, context : &mut Rltk) {
        // update systems
        player_input(self, context);
        self.update_systems();

        // clear screen
        context.cls();

        // render map
        let map = self.world.fetch::<Map>();
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
    let mut game_state = State {
        world: World::new()
    };
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .with_fps_cap(60f32)
        .build()?;

    // register components
    game_state.world.register::<TransformData>();
    game_state.world.register::<RenderData>();
    game_state.world.register::<Player>();

    // insert resources
    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].centre();
    game_state.world.insert(map);

    // add entities
    game_state.world.create_entity()
        .with(TransformData { x: player_x, y: player_y })
        .with(RenderData {
            glyph: rltk::to_cp437('@'),
            foreground: COLOURS[10],
            background: COLOURS[0]
        })
        .with(Player {})
        .build();

    // fire off main loop
    rltk::main_loop(context, game_state)
}
