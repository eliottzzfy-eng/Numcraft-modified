use fastnoise_lite::FastNoiseLite;
use libm::roundf;
use nalgebra::Vector3;
use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::{
    constants::world::*,
    world::{
        chunk_manager::ChunksManager,
        structures::{Structure, HOUSE1, TREE1, WELL},
    },
};

const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

pub struct WorldGenerator {
    noise: FastNoiseLite,
    /// Whether the village has already been placed in this world
    village_placed: bool,
}

impl WorldGenerator {
    pub fn new() -> Self {
        let mut noise = FastNoiseLite::new();
        noise.set_noise_type(Some(fastnoise_lite::NoiseType::OpenSimplex2));
        WorldGenerator {
            noise,
            village_placed: false,
        }
    }

    pub fn set_seed(&mut self, seed: i32) {
        self.noise.set_seed(Some(seed));
    }

    pub fn generate_chunk(
        &mut self,
        chunks_manager: &mut ChunksManager,
        chunk_pos: Vector3<isize>,
    ) {
        let chunk = chunks_manager.get_chunk_at_pos_mut(chunk_pos).unwrap();

        if chunk.generated {
            return;
        }
        chunk.generated = true;

        let chunk_block_pos = chunk_pos * CHUNK_SIZE_I;

        let mut height_map = [0isize; CHUNK_SIZE * CHUNK_SIZE];

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let negative_1_to_1 = self.noise.get_noise_2d(
                    (x as isize + chunk_block_pos.x) as f32,
                    (z as isize + chunk_block_pos.z) as f32,
                );
                let height = roundf((negative_1_to_1 + 1.) / 2. * 14.0 + 8.0) as isize;
                height_map[x + z * CHUNK_SIZE] = height;
            }
        }

        for x in 0..CHUNK_SIZE_I {
            for z in 0..CHUNK_SIZE_I {
                let height = height_map[x as usize + z as usize * CHUNK_SIZE];
                for y in 0..CHUNK_SIZE_I {
                    let block_y = chunk_block_pos.y + y;
                    if block_y == height {
                        chunk.set_at(
                            Vector3::new(x as usize, y as usize, z as usize),
                            crate::constants::BlockType::Grass,
                        );
                    }
                    if block_y < height && block_y >= height - 3 {
                        chunk.set_at(
                            Vector3::new(x as usize, y as usize, z as usize),
                            crate::constants::BlockType::Dirt,
                        );
                    }
                    if block_y < height - 3 {
                        chunk.set_at(
                            Vector3::new(x as usize, y as usize, z as usize),
                            crate::constants::BlockType::Stone,
                        );
                    }
                }
            }
        }

        // Generate a pseudo random seed
        let seed = ((chunk_pos.x as i64 + 2147483648) * 1000
            + (chunk_pos.y as i64 + 2147483648) * 1000000
            + (chunk_pos.z as i64 + 2147483648) * 1000000000
            + (self.noise.seed as i64 + 2147483648)) as u64;
        let mut rng = XorShiftRng::seed_from_u64(seed);

        for x in 0..CHUNK_SIZE_I {
            for z in 0..CHUNK_SIZE_I {
                let world_pos = Vector3::new(
                    x + chunk_block_pos.x,
                    height_map[x as usize + z as usize * CHUNK_SIZE] + 1,
                    z + chunk_block_pos.z,
                );
                if rng.next_u32() < u32::MAX / 64 {
                    self.place_struct_check_space(
                        chunks_manager,
                        &TREE1,
                        world_pos - Vector3::new(1, 0, 1),
                        Vector3::new(1, 0, 1),
                    );
                }
            }
        }

        // Place the village once, in the spawn chunk (0, y, 0)
        if !self.village_placed && chunk_pos.x == 0 && chunk_pos.z == 0 {
            self.village_placed = true;
            // Ground height at center of spawn chunk (block 4,_,4)
            let ground_y = height_map[4 + 4 * CHUNK_SIZE];
            let center = Vector3::new(
                chunk_block_pos.x + 4,
                ground_y + 1,
                chunk_block_pos.z + 4,
            );
            self.place_village(chunks_manager, center);
        }
    }

    /// Place a small village centered at `center` (one block above ground).
    /// Layout: puits au centre, 4 maisons aux 4 coins, chemins en planches.
    fn place_village(&self, chunks_manager: &mut ChunksManager, center: Vector3<isize>) {
        // ── Puits au centre ─────────────────────────────────────────────────
        let well_pos = center + Vector3::new(-1, -1, -1); // centré sur 3 blocs
        self.place_struct(chunks_manager, &WELL, well_pos);

        // ── 4 maisons (7×6 empreinte) aux 4 coins ───────────────────────────
        // Offset: assez loin du puits (rayon ~10 blocs) pour qu'il y ait de la place
        let house_offsets: [(isize, isize); 4] = [
            (-12, -10),  // nord-ouest
            (  6, -10),  // nord-est
            (-12,   5),  // sud-ouest
            (  6,   5),  // sud-est
        ];

        for (dx, dz) in house_offsets {
            let house_pos = Vector3::new(center.x + dx, center.y - 1, center.z + dz);
            self.place_struct(chunks_manager, &HOUSE1, house_pos);
        }

        // ── Chemins en planches reliant les maisons au puits ────────────────
        // On trace 4 segments droits : du centre vers chaque maison
        let path_targets: [(isize, isize, isize, isize); 4] = [
            (-8, -5, -2, -1),   // vers nord-ouest
            ( 3, -5,  2, -1),   // vers nord-est
            (-8,  3, -2,  2),   // vers sud-ouest
            ( 3,  3,  2,  2),   // vers sud-est
        ];
        for (x1, z1, x2, z2) in path_targets {
            self.place_path(
                chunks_manager,
                center + Vector3::new(x1, -1, z1),
                center + Vector3::new(x2, -1, z2),
            );
        }
    }

    /// Draw a straight plank path between two world positions (same Y).
    fn place_path(
        &self,
        chunks_manager: &mut ChunksManager,
        from: Vector3<isize>,
        to: Vector3<isize>,
    ) {
        let dx = (to.x - from.x).signum();
        let dz = (to.z - from.z).signum();
        let mut pos = from;
        let y = from.y;

        loop {
            // Place planks on the surface (replace only grass/air/dirt on top)
            let surface = Vector3::new(pos.x, y, pos.z);
            chunks_manager.set_block_in_world(surface, crate::constants::BlockType::Planks);

            if pos.x == to.x && pos.z == to.z {
                break;
            }
            // Bresenham-ish: advance on the longer axis first
            let rem_x = (to.x - pos.x).abs();
            let rem_z = (to.z - pos.z).abs();
            if rem_x >= rem_z {
                pos.x += dx;
            } else {
                pos.z += dz;
            }
        }
    }

    /// Place a structure only if there is enough space
    pub fn place_struct_check_space(
        &self,
        chunks_manager: &mut ChunksManager,
        structure: &'static Structure,
        pos: Vector3<isize>,
        margins: Vector3<isize>,
    ) {
        for y in (-margins.y)..structure.size.y as isize + margins.y {
            for x in (-margins.x)..structure.size.x as isize + margins.x {
                for z in (-margins.z)..structure.size.z as isize + margins.z {
                    if !chunks_manager
                        .get_block_in_world(pos + Vector3::new(x, y, z))
                        .is_none_or(|b| b.is_air())
                    {
                        return;
                    }
                }
            }
        }
        self.place_struct(chunks_manager, structure, pos);
    }

    pub fn place_struct(
        &self,
        chunks_manager: &mut ChunksManager,
        structure: &'static Structure,
        pos: Vector3<isize>,
    ) {
        for y in 0..structure.size.y {
            for x in 0..structure.size.x {
                for z in 0..structure.size.z {
                    if let Some(block) = structure.get_block_at(Vector3::new(x, y, z)) {
                        let dest_pos = pos + Vector3::new(x as isize, y as isize, z as isize);
                        chunks_manager.set_block_in_world(dest_pos, block);
                    }
                }
            }
        }
    }
}
