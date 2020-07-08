use specs::prelude::*;
use super::{ BlocksTile, Map, Position };

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, BlocksTile>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, positions, blockers) = data;

        // block out wall tiles
        map.populate_blocked();

        // for each entity that blocks movement, add to mao's blocked_tiles
        for (position, _) in (&positions, &blockers).join() {
            let idx = map.xy_idx(position.x, position.y);
            map.blocked_tiles[idx] = true;
        }
    }
}
