use rltk::{ GameState, Rltk, RGB, Point };
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
// systems
mod visibility_system;
pub use visibility_system::*;
mod enemy_ai_system;
pub use enemy_ai_system::*;

/// STATE ///
/// ///// ///
#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running
}

pub struct State {
    world: World,
    run_state: RunState
}
impl GameState for State {
    fn tick(&mut self, context : &mut Rltk) {
        // 
        if self.run_state == RunState::Running {
            // update systems
            self.update_systems();
            self.run_state = RunState::Paused;
        } else {
            let received_input = player_input(self, context);
            if received_input {
                self.run_state = RunState::Running;
            }
        }

        // clear screen
        context.cls();

        // render map
        draw_map(&self.world, context);

        // render entities
        let positions = self.world.read_storage::<Position>();
        let render_datas = self.world.read_storage::<RenderData>();
        let map = self.world.fetch::<Map>();
        for (pos, render) in (&positions, &render_datas).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                context.set(pos.x, pos.y, render.foreground, render.background, render.glyph);
            }
        }
    }
}
impl State {
    fn update_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.world);

        let mut ai = EnemyAI {};
        ai.run_now(&self.world);

        self.world.maintain();
    }
}

/// MAIN ///
/// //// ///
fn main() -> rltk::BError {
    // init
    use rltk::RltkBuilder;
    let mut game_state = State {
        world: World::new(),
        run_state: RunState::Running
    };
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .with_fps_cap(60f32)
        .build()?;

    // register components
    register_components(&mut game_state.world);

    // create map
    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].centre();
    
    // add entities
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.centre();

        let glyph: rltk::FontCharType;
        let col: RGB;
        let name: String;

        let roll = rng.roll_dice(1, 2);
        match roll {
            // goblin
            1 => { glyph = rltk::to_cp437('g'); col = COLOURS[11]; name = "Goblin".to_string(); }
            // orc
            _ => { glyph = rltk::to_cp437('o'); col = COLOURS[3]; name = "Orc".to_string(); }
        }

        game_state.world.create_entity()
            .with(Position{ x, y })
            .with(RenderData {
                glyph: glyph,
                foreground: col,
                background: COLOURS[0]
            })
            .with(Viewshed::new(8))
            .with(Enemy {})
            .with(Name { name: format!("{} #{}", &name, i) })
            .build();
    }

    // add player
    game_state.world.create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(RenderData {
            glyph: rltk::to_cp437('@'),
            foreground: COLOURS[7],
            background: COLOURS[0]
        })
        .with(Player {})
        .with(Viewshed::new(8))
        .with(Name { name: "Player".to_string() })
        .build();

    // insert resources
    game_state.world.insert(map);
    game_state.world.insert(Point::new(player_x, player_y));

    // fire off main loop
    rltk::main_loop(context, game_state)
}
