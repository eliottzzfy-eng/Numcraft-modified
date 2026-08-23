use core::any::Any;

use crate::constants::world::{
    CHUNK_SIZE, ITEM_MAGNET_FORCE, MAX_ITEM_MERGING_DISTANCE, MAX_PLAYER_ITEM_MAGNET_DISTANCE,
    MAX_PIGS, PIG_DIRECTION_CHANGE_INTERVAL, PIG_SPAWN_RADIUS,
    PIG_SPAWN_TICK_INTERVAL, PIG_WALK_SPEED,
};
use crate::constants::{BlockType, EntityType, ItemType};
use crate::entity::Entity;
use crate::entity::item::ItemEntityCustomData;
use crate::entity::pig::PigEntityData;
use crate::inventory::{Inventory, ItemStack};
use crate::world::chunk_manager::ChunksManager;
use crate::world::world_generator::WorldGenerator;

calc_use!(alloc::boxed::Box);
calc_use!(alloc::vec::Vec);
calc_use!(alloc::vec);

use nalgebra::Vector3;

pub mod chunk;
pub mod chunk_manager;
mod structures;
pub mod world_generator;

const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

pub struct World {
    pub chunks_manager: ChunksManager,
    registered_inventories: Vec<Inventory>,
    loaded_entities: Vec<Entity>,
    next_available_entity_id: usize,
    world_generator: WorldGenerator,
    /// Counts game ticks; drives natural pig spawning
    pub pig_spawn_tick: u32,
}

pub struct RegisteredInventory {
    inventory: Inventory,
    block_pos: Option<Vector3<usize>>,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents the current world. Contains all the chunks
impl World {
    pub fn new() -> Self {
        let mut world = World {
            chunks_manager: ChunksManager::new(),
            registered_inventories: Vec::new(),
            loaded_entities: vec![Entity::new(0, EntityType::Player, None)],
            next_available_entity_id: 1,
            world_generator: WorldGenerator::new(),
            pig_spawn_tick: 0,
        };

        world
    }

    pub fn get_player_entity_mut(&mut self) -> &mut Entity {
        &mut self.loaded_entities[0]
    }

    pub fn get_player_entity(&self) -> &Entity {
        &self.loaded_entities[0]
    }

    pub fn load_area(
        &mut self,
        x_start: isize,
        x_stop: isize,
        y_start: isize,
        y_stop: isize,
        z_start: isize,
        z_stop: isize,
    ) {
        for x in x_start..x_stop {
            for y in y_start..y_stop {
                for z in z_start..z_stop {
                    self.chunks_manager.add_chunk(Vector3::new(x, y, z));
                }
            }
        }

        for x in x_start..x_stop {
            for y in y_start..y_stop {
                for z in z_start..z_stop {
                    self.world_generator
                        .generate_chunk(&mut self.chunks_manager, Vector3::new(x, y, z));
                }
            }
        }
    }

    pub fn update_entities(&mut self, delta_time: f32) {
        // Check for item merging and player magnet
        'first_loop: for i in 0..self.loaded_entities.len() {
            if self.loaded_entities[i].get_type() == EntityType::Item {
                // Ignore items with custom data = None, they will be removed after
                if self.loaded_entities[i].custom_data.is_none() {
                    continue;
                }

                // Get the item_data from the first item
                let first_item_data = ItemEntityCustomData::get_item_data(&self.loaded_entities[i])
                    .expect("Item Entity must have ItemData as custom data.");
                let first_item_stack = first_item_data.item_stack.clone();

                let max_stack = first_item_stack.get_item_type().get_max_stack_amount();

                for j in 0..self.loaded_entities.len() {
                    if i != j
                        && self.loaded_entities[j].custom_data.is_some() // Ignore items with custom data = None, they will be removed after
                        && self.loaded_entities[j].get_type() == EntityType::Item
                        && self.loaded_entities[i]
                            .pos
                            .metric_distance(&self.loaded_entities[j].pos)
                            <= MAX_ITEM_MERGING_DISTANCE
                    {
                        // Check if the items can merge
                        let second_item_data =
                            ItemEntityCustomData::get_item_data(&self.loaded_entities[j])
                                .expect("Item Entity must have ItemData as custom data.");
                        let second_item_stack = second_item_data.item_stack.clone();

                        if second_item_stack.get_item_type() != first_item_stack.get_item_type() {
                            continue;
                        }

                        if let Some(first_bbox) = self.loaded_entities[i].get_bbox()
                            && let Some(second_bbox) = self.loaded_entities[j].get_bbox()
                            && first_bbox.is_coliding(&second_bbox)
                        {
                            if first_item_stack.get_amount() == max_stack
                                || second_item_stack.get_amount() == max_stack
                            {
                                continue;
                            }

                            let total =
                                first_item_stack.get_amount() + second_item_stack.get_amount();
                            if total <= max_stack {
                                // Merge the two items together and request the deletion of the second one
                                self.loaded_entities[i].custom_data =
                                    Some(Box::new(ItemEntityCustomData {
                                        item_stack: ItemStack::new(
                                            first_item_stack.get_item_type(),
                                            total,
                                            false,
                                        ),
                                    }));
                                self.loaded_entities[j].custom_data = None; // Yes, this should be illegal but it can also be a feature.
                                self.loaded_entities[i].velocity = Vector3::zeros();
                                continue 'first_loop;
                            } else {
                                self.loaded_entities[i].custom_data =
                                    Some(Box::new(ItemEntityCustomData {
                                        item_stack: ItemStack::new(
                                            first_item_stack.get_item_type(),
                                            max_stack,
                                            false,
                                        ),
                                    }));
                                self.loaded_entities[j].custom_data =
                                    Some(Box::new(ItemEntityCustomData {
                                        item_stack: ItemStack::new(
                                            first_item_stack.get_item_type(),
                                            total - max_stack,
                                            false,
                                        ),
                                    }));
                                self.loaded_entities[i].velocity = Vector3::zeros();
                                self.loaded_entities[j].velocity = Vector3::zeros();
                            }
                            continue;
                        }

                        // Calculate the direction to the other item
                        let direction =
                            (self.loaded_entities[j].pos - self.loaded_entities[i].pos).normalize();

                        self.loaded_entities[i].velocity +=
                            direction * ITEM_MAGNET_FORCE * delta_time;

                        // Limit the magnet speed
                        /*if self.loaded_entities[i].velocity.norm() > ITEM_MAGNET_SPEED {
                            self.loaded_entities[i].velocity =
                                self.loaded_entities[i].velocity.normalize()
                                    * ITEM_MAGNET_SPEED
                                    * delta_time;
                        }*/
                    }
                }
            }
        }

        // Player item magnet
        for i in 0..self.loaded_entities.len() {
            let distance = self.loaded_entities[i]
                .pos
                .metric_distance(&self.get_player_entity().pos);
            if self.loaded_entities[i].get_type() == EntityType::Item
                && self.loaded_entities[i].custom_data.is_some()
                && distance < MAX_PLAYER_ITEM_MAGNET_DISTANCE
            {
                let direction =
                    (self.get_player_entity().pos - self.loaded_entities[i].pos).normalize();

                self.loaded_entities[i].velocity += direction * ITEM_MAGNET_FORCE * delta_time;
            }
        }

        // Remove illegal items
        self.loaded_entities.retain(|entity| {
            entity.get_type() != EntityType::Item
                || (entity.get_type() == EntityType::Item && !entity.custom_data.is_none())
        });

        // Update pig AI
        self.update_pig_ai(delta_time);
    }

    /// Set the world generation seed
    pub fn set_seed(&mut self, seed: i32) {
        self.world_generator.set_seed(seed);
    }

    fn register_inventory(&mut self, inventory: Inventory) {
        self.registered_inventories.push(inventory);
    }

    pub fn get_all_entities_mut(&mut self) -> &mut Vec<Entity> {
        &mut self.loaded_entities
    }
    pub fn get_all_entities(&self) -> &Vec<Entity> {
        &self.loaded_entities
    }

    pub fn get_entity_by_id_mut(&mut self, id: usize) -> Option<&mut Entity> {
        self.loaded_entities
            .iter_mut()
            .find(|entity| entity.get_id() == id)
    }

    pub fn get_entity_by_id(&self, id: usize) -> Option<&Entity> {
        self.loaded_entities
            .iter()
            .find(|entity| entity.get_id() == id)
    }

    pub fn spawn_entity(&mut self, mut entity: Entity, pos: Vector3<f32>) {
        entity.pos = pos;
        self.loaded_entities.push(entity);
    }

    pub fn get_new_entity_id(&mut self) -> usize {
        let id = self.next_available_entity_id;
        self.next_available_entity_id += 1;
        id
    }

    pub fn clear_entities(&mut self) {
        if self.loaded_entities.len() > 1 {
            for _ in 1..self.loaded_entities.len() {
                self.loaded_entities.remove(1);
            }
        }
    }

    pub fn spawn_entity_auto(
        &mut self,
        entity_type: EntityType,
        pos: Vector3<f32>,
        custom_data: Option<Box<dyn Any>>,
    ) {
        let id = self.get_new_entity_id();
        self.spawn_entity(Entity::new(id, entity_type, custom_data), pos);
    }

    pub fn spawn_item_entity(&mut self, pos: Vector3<f32>, item_stack: ItemStack) {
        self.spawn_entity_auto(
            EntityType::Item,
            pos,
            Some(Box::new(ItemEntityCustomData { item_stack })),
        );
    }

    pub fn replace_block_and_drop_item(&mut self, pos: Vector3<isize>, block_type: BlockType) {
        if let Some(current_block) = self.chunks_manager.get_block_in_world(pos) {
            let drop_type = current_block.get_dropped_item_type();
            if drop_type != ItemType::Air {
                self.chunks_manager.set_block_in_world(pos, block_type);
                self.spawn_item_entity(
                    pos.map(|v| v as f32 + 0.5),
                    ItemStack::new(drop_type, 1, false),
                );
            }
        }
    }

    /// Destroys every breakable block within `radius` blocks of `center` (a sphere,
    /// like a TNT explosion). Border (the indestructible world edge, hardness < 0.)
    /// is skipped. Pure destruction: no items are dropped, unlike normal mining.
    pub fn explode(&mut self, center: Vector3<isize>, radius: isize) {
        let radius_f = radius as f32;
        for x in -radius..=radius {
            for y in -radius..=radius {
                for z in -radius..=radius {
                    let offset = Vector3::new(x, y, z);
                    if offset.map(|v| v as f32).norm() > radius_f {
                        continue;
                    }
                    let pos = center + offset;
                    if let Some(block) = self.chunks_manager.get_block_in_world(pos)
                        && !block.is_air()
                        && block.get_hardness() >= 0.
                    {
                        self.chunks_manager.set_block_in_world(pos, BlockType::Air);
                    }
                }
            }
        }
    }

    pub fn remove_entity(&mut self, id: usize) -> bool {
        for i in 0..self.loaded_entities.len() {
            if self.loaded_entities[i].get_id() == id {
                self.loaded_entities.remove(i);
                return true;
            }
        }
        false
    }

    pub fn get_highest_block(&self, x: isize, z: isize) -> isize {
        let mut highest_chunk_y = self.chunks_manager.chunks[0].get_pos().y;
        for chunk in self.chunks_manager.chunks.iter() {
            if chunk.get_pos().y > highest_chunk_y {
                highest_chunk_y = chunk.get_pos().y;
            }
        }
        let max_block = (highest_chunk_y + 1) * CHUNK_SIZE_I;
        let mut highest_block = max_block-1;

        for y in (0..max_block).rev() {
            if !self
                .chunks_manager
                .get_block_in_world(Vector3::new(x, y, z))
                .is_some_and(|b| b.is_air())
            {
                highest_block = y + 1;
                break;
            }
        }
        highest_block
    }

    /// Spawn a pig at the given world position (one block above the ground)
    pub fn spawn_pig(&mut self, pos: Vector3<f32>) {
        let id = self.next_available_entity_id;
        self.next_available_entity_id += 1;
        let mut entity = Entity::new(
            id,
            EntityType::Pig,
            Some(Box::new(PigEntityData::new())),
        );
        entity.pos = pos;
        entity.gravity = true;
        self.loaded_entities.push(entity);
    }

    /// Count how many pigs are currently alive
    pub fn count_pigs(&self) -> usize {
        self.loaded_entities
            .iter()
            .filter(|e| e.get_type() == EntityType::Pig)
            .count()
    }

    /// Update pig AI: random walk with periodic direction changes.
    /// Called from update_entities every tick.
    pub fn update_pig_ai(&mut self, delta_time: f32) {
        // Collect pig ids first to avoid borrow issues
        let pig_ids: Vec<usize> = self
            .loaded_entities
            .iter()
            .filter(|e| e.get_type() == EntityType::Pig)
            .map(|e| e.get_id())
            .collect();

        for pig_id in pig_ids {
            // Get a deterministic-ish direction based on the pig's id and current time
            let (new_timer, new_vx, new_vz) = {
                let entity = self
                    .loaded_entities
                    .iter()
                    .find(|e| e.get_id() == pig_id)
                    .unwrap();
                let pig_data = PigEntityData::get_pig_data(entity).unwrap();
                let timer = pig_data.direction_timer - delta_time;
                if timer <= 0.0 {
                    // Pick a new direction using a simple hash of id + position
                    let hash = (pig_id as f32 * 1234.5
                        + entity.pos.x * 7.3
                        + entity.pos.z * 3.7) as i32;
                    let angle_idx = hash.unsigned_abs() % 8;
                    let (vx, vz) = match angle_idx {
                        0 => (1.0_f32, 0.0_f32),
                        1 => (0.7, 0.7),
                        2 => (0.0, 1.0),
                        3 => (-0.7, 0.7),
                        4 => (-1.0, 0.0),
                        5 => (-0.7, -0.7),
                        6 => (0.0, -1.0),
                        _ => (0.7, -0.7),
                    };
                    (PIG_DIRECTION_CHANGE_INTERVAL, vx * PIG_WALK_SPEED, vz * PIG_WALK_SPEED)
                } else {
                    (timer, entity.velocity.x, entity.velocity.z)
                }
            };

            let entity = self
                .loaded_entities
                .iter_mut()
                .find(|e| e.get_id() == pig_id)
                .unwrap();
            entity.velocity.x = new_vx;
            entity.velocity.z = new_vz;
            if let Some(data) = PigEntityData::get_pig_data_mut(entity) {
                data.direction_timer = new_timer;
            }
        }
    }

    /// Try to spawn a pig naturally near the player. Call once per game tick.
    /// Spawns only if fewer than MAX_PIGS pigs exist and the random tick fires.
    pub fn try_natural_pig_spawn(&mut self, player_pos: Vector3<f32>) {
        self.pig_spawn_tick = self.pig_spawn_tick.wrapping_add(1);
        if self.pig_spawn_tick % PIG_SPAWN_TICK_INTERVAL != 0 {
            return;
        }
        if self.count_pigs() >= MAX_PIGS {
            return;
        }

        // Pick a candidate position using a cheap hash of the tick counter
        let tick = self.pig_spawn_tick as isize;
        let dx = (tick * 7 % (PIG_SPAWN_RADIUS * 2)) - PIG_SPAWN_RADIUS;
        let dz = (tick * 13 % (PIG_SPAWN_RADIUS * 2)) - PIG_SPAWN_RADIUS;
        let spawn_x = player_pos.x as isize + dx;
        let spawn_z = player_pos.z as isize + dz;
        let spawn_y = self.get_highest_block(spawn_x, spawn_z);

        // Only spawn on a solid block with air above it
        let ground = self
            .chunks_manager
            .get_block_in_world(Vector3::new(spawn_x, spawn_y - 1, spawn_z));
        let above = self
            .chunks_manager
            .get_block_in_world(Vector3::new(spawn_x, spawn_y, spawn_z));

        if ground.is_some_and(|b| !b.is_air()) && above.is_some_and(|b| b.is_air()) {
            self.spawn_pig(Vector3::new(
                spawn_x as f32 + 0.5,
                spawn_y as f32 + 0.5,
                spawn_z as f32 + 0.5,
            ));
        }
    }

    /// Clear all the chunks and entities
    pub fn clear(&mut self) {
        self.chunks_manager.clear();
        self.clear_entities();
    }
}
