use specs::prelude::*;
use rltk::{ Point, console };
use super::{ Viewshed, Enemy, Name };

pub struct EnemyAI {}

impl<'a> System<'a> for EnemyAI {
    type SystemData = ( ReadExpect<'a, Point>,
                        ReadStorage<'a, Viewshed>,
                        ReadStorage<'a, Enemy>,
                        ReadStorage<'a, Name>);

    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, viewsheds, enemies, names) = data;

        for (viewshed, _, name) in (&viewsheds, &enemies, &names).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(format!("{} shouts insults", name.name));
            }
        }
    }
}
