use rltk::{Rltk, GameState};

struct State { time: u16, cap: u16 }
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust world!");
        ctx.print(1, 2, "Time: ");
        ctx.print(7, 2, self.time);
        self.time += 1;
        if self.time > self.cap {
            self.time = 0;
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let fps_cap = 30;
    let gs = State{ time: 0, cap: fps_cap };
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .with_fps_cap(fps_cap.into())
        .build()?;
    return rltk::main_loop(context, gs);
}
