use specs::prelude::*;
use rltk::{ field_of_view };
use super::{ Viewshed, Position, Player, Map };

pub struct VisibilitySystem {}
impl<'a> System<'a> for VisibilitySystem {
    type SystemData = ( WriteExpect<'a, Map>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, Player>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;
        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.out_of_date {
                viewshed.out_of_date = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = field_of_view(pos.to_point(), viewshed.range, &*map);
                viewshed.visible_tiles.retain(
                    |p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height
                );

                // if this is the player
                let p: Option<&Player> = player.get(ent);
                if let Some(_) = p {
                    for t in map.visible_tiles.iter_mut() { *t = false; }
                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.explored_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}
