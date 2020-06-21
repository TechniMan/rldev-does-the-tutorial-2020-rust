use std::cmp::{max, min};
use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::Component;

/// COMPONENTS ///
/// ////////// ///
#[derive(Component)]
struct TransformData {
    x: i32,
    y: i32
}

#[derive(Component)]
struct RenderData {
    glyph: rltk::FontCharType,
    foreground: RGB,
    background: RGB
}

#[derive(Component)]
struct LeftMover {}

#[derive(Component)]
struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    let mut transforms = world.write_storage::<TransformData>();
    let mut players = world.write_storage::<Player>();

    for (player, trans) in (&mut players, &mut transforms).join() {
        trans.x = min(79, max(0, trans.x + delta_x));
        trans.y = min(49, max(0, trans.y + delta_y));
    }
}

fn player_input(state: &mut State, context: &mut Rltk) {
    match context.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut state.world),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut state.world),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut state.world),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut state.world),
            _ => {}
        }
    }
}

/// SYSTEMS ///
/// /////// ///
struct LeftMoverSystem {}
impl<'a> System<'a> for LeftMoverSystem {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, TransformData>);

    fn run(&mut self, (_lefty, mut _trans) : Self::SystemData) {
        for (lefty, trans) in (&_lefty, &mut _trans).join() {
            // move this unit left
            trans.x -= 1;
            // reset if invalid
            if trans.x < 0 {
                trans.x = 79;
            }
        }
    }
}

/// STATE ///
/// ///// ///
struct State {
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
        let mut left_mover_system = LeftMoverSystem {};
        left_mover_system.run_now(&self.world);
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
    game_state.world.register::<LeftMover>();

    // add entities
    game_state.world.create_entity()
        .with(TransformData { x: 40, y: 25 })
        .with(RenderData {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::WHITE),
            background: RGB::named(rltk::BLACK)
        })
        .with(Player {})
        .build();

    for i in 0..10 {
        game_state.world.create_entity()
            .with(TransformData { x: i * 7, y: 20 })
            .with(RenderData {
                glyph: rltk::to_cp437('é'),
                foreground: RGB::named(rltk::RED),
                background: RGB::named(rltk::BLACK)
            })
            .with(LeftMover {})
            .build();
    }

    // fire off main loop
    return rltk::main_loop(context, game_state);
}
