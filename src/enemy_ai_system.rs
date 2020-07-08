use specs::prelude::*;
use rltk::{ Point, console };
use super::{ Enemy, Map, Name, Position, Viewshed };

pub struct EnemyAI {}

impl<'a> System<'a> for EnemyAI {
    #[allow(clippy::type_complexity)]
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadExpect<'a, Point>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, Enemy>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, mut viewsheds, enemies, names, mut positions) = data;

        for (mut viewshed, _, name, mut pos) in (&mut viewsheds, &enemies, &names, &mut positions).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
                if distance < 1.5 {
                    // attack goes here later
                    console::log(format!("{} shouts insults", name.name));
                    return;
                }
                let path = rltk::a_star_search(
                    map.xy_idx(pos.x, pos.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map
                );
                if path.success && path.steps.len() > 1 {
                    pos.x = (path.steps[1] as i32) % map.width;
                    pos.y = (path.steps[1] as i32) / map.width;
                    viewshed.out_of_date = true;
                }
            }
        }
    }
}
