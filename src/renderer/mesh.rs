calc_use!(alloc::vec::Vec);

use nalgebra::{Vector2, Vector3};

use crate::{
    constants::{world::CHUNK_SIZE, BlockType},
    world::{chunk::Chunk, chunk_manager::ChunksManager},
};

const CHUNK_SIZE_I: isize = CHUNK_SIZE as isize;

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum QuadDir {
    Front = 1,
    Back = 2,
    Top = 3,
    Bottom = 4,
    Right = 5,
    Left = 6,
}

impl QuadDir {
    pub const fn from_id(id: u8) -> Self {
        match id {
            1 => QuadDir::Front,
            2 => QuadDir::Back,
            3 => QuadDir::Top,
            4 => QuadDir::Bottom,
            5 => QuadDir::Right,
            6 => QuadDir::Left,
            _ => panic!("Unknown quad direction."),
        }
    }

    pub const fn get_normal_vector(&self) -> Vector3<isize> {
        match *self {
            QuadDir::Front => Vector3::new(0, 0, -1),
            QuadDir::Back => Vector3::new(0, 0, 1),
            QuadDir::Bottom => Vector3::new(0, -1, 0),
            QuadDir::Top => Vector3::new(0, 1, 0),
            QuadDir::Left => Vector3::new(1, 0, 0),
            QuadDir::Right => Vector3::new(-1, 0, 0),
        }
    }
}

pub struct Quad {
    data: u16,
    texture_id: u8,
}

impl Quad {
    pub fn new(pos: Vector3<u16>, dir: QuadDir, texture_id: u8, light: u16) -> Self {
        // xxx yyy zzz ddd llll
        // tttttttt
        let x = pos.x;
        let y = pos.y;
        let z = pos.z;
        let dir = dir as u16;
        let data = x << 13 | y << 10 | z << 7 | dir << 4 | light;
        Quad { data, texture_id }
    }

    pub fn get_pos(&self) -> nalgebra::Vector3<u16> {
        let x = self.data >> 13; // Equivalent to (self.data & 0b1110000000000000) >> 13
        let y = (self.data & 0b0001110000000000) >> 10;
        let z = (self.data & 0b0000001110000000) >> 7;
        nalgebra::Vector3::new(x, y, z)
    }
    pub fn get_light_level(&self) -> u16 {
        self.data & 0b0000000000001111
    }

    pub fn get_dir(&self) -> QuadDir {
        let dir = (self.data & 0b0000000001110000) >> 4;
        QuadDir::from_id(dir as u8)
    }
}

impl Quad {
    pub fn get_triangles(&self, chunk_block_pos: Vector3<isize>) -> (Triangle, Triangle) {
        let pos = self.get_pos().map(|x| x as isize) + chunk_block_pos;

        let pos_x = (pos.x) as f32;
        let pos_x_plus_one = pos_x + 1.0;
        let pos_y = (pos.y) as f32;
        let pos_y_plus_one = pos_y + 1.0;
        let pos_z = (pos.z) as f32;
        let pos_z_plus_one = pos_z + 1.0;

        let light = self.get_light_level() as u8;
        match self.get_dir() {
            QuadDir::Front => (
                Triangle {
                    p1: Vector3::new(pos_x_plus_one, pos_y_plus_one, pos_z),
                    p2: Vector3::new(pos_x_plus_one, pos_y, pos_z),
                    p3: Vector3::new(pos_x, pos_y, pos_z),
                    texture_id: self.texture_id,
                    light,
                },
                Triangle {
                    p1: Vector3::new(pos_x, pos_y, pos_z),
                    p2: Vector3::new(pos_x, pos_y_plus_one, pos_z),
                    p3: Vector3::new(pos_x_plus_one, pos_y_plus_one, pos_z),
                    texture_id: self.texture_id,
                    light,
                },
            ),
            QuadDir::Back => (
                Triangle {
                    p1: Vector3::new(pos_x, pos_y, pos_z_plus_one),
                    p2: Vector3::new(pos_x_plus_one, pos_y, pos_z_plus_one),
                    p3: Vector3::new(pos_x_plus_one, pos_y_plus_one, pos_z_plus_one),
                    texture_id: self.texture_id,
                    light,
                },
                Triangle {
                    p1: Vector3::new(pos_x_plus_one, pos_y_plus_one, pos_z_plus_one),
                    p2: Vector3::new(pos_x, pos_y_plus_one, pos_z_plus_one),
                    p3: Vector3::new(pos_x, pos_y, pos_z_plus_one),
                    texture_id: self.texture_id,
                    light,
                },
            ),
            QuadDir::Bottom => (
                Triangle {
                    p1: Vector3::new(pos_x_plus_one, pos_y, pos_z),
                    p2: Vector3::new(pos_x_plus_one, pos_y, pos_z_plus_one),
                    p3: Vector3::new(pos_x, pos_y, pos_z_plus_one),
                    texture_id: self.texture_id,
                    light,
                },
                Triangle {
                    p1: Vector3::new(pos_x, pos_y, pos_z_plus_one),
                    p2: Vector3::new(pos_x, pos_y, pos_z),
                    p3: Vector3::new(pos_x_plus_one, pos_y, pos_z),
                    texture_id: self.texture_id,
                    light,
                },
            ),
            QuadDir::Top => (
                Triangle {
                    p1: Vector3::new(pos_x, pos_y_plus_one, pos_z_plus_one),
                    p2: Vector3::new(pos_x_plus_one, pos_y_plus_one, pos_z_plus_one),
                    p3: Vector3::new(pos_x_plus_one, pos_y_plus_one, pos_z),
                    texture_id: self.texture_id,
                    light,
                },
                Triangle {
                    p1: Vector3::new(pos_x_plus_one, pos_y_plus_one, pos_z),
                    p2: Vector3::new(pos_x, pos_y_plus_one, pos_z),
                    p3: Vector3::new(pos_x, pos_y_plus_one, pos_z_plus_one),
                    texture_id: self.texture_id,
                    light,
                },
            ),
            QuadDir::Left => (
                Triangle {
                    p3: Vector3::new(pos_x_plus_one, pos_y_plus_one, pos_z_plus_one),
                    p2: Vector3::new(pos_x_plus_one, pos_y_plus_one, pos_z),
                    p1: Vector3::new(pos_x_plus_one, pos_y, pos_z),
                    texture_id: self.texture_id,
                    light,
                },
                Triangle {
                    p3: Vector3::new(pos_x_plus_one, pos_y, pos_z),
                    p2: Vector3::new(pos_x_plus_one, pos_y, pos_z_plus_one),
                    p1: Vector3::new(pos_x_plus_one, pos_y_plus_one, pos_z_plus_one),
                    texture_id: self.texture_id,
                    light,
                },
            ),
            QuadDir::Right => (
                Triangle {
                    p1: Vector3::new(pos_x, pos_y_plus_one, pos_z_plus_one),
                    p2: Vector3::new(pos_x, pos_y_plus_one, pos_z),
                    p3: Vector3::new(pos_x, pos_y, pos_z),
                    texture_id: self.texture_id,
                    light,
                },
                Triangle {
                    p1: Vector3::new(pos_x, pos_y, pos_z),
                    p2: Vector3::new(pos_x, pos_y, pos_z_plus_one),
                    p3: Vector3::new(pos_x, pos_y_plus_one, pos_z_plus_one),
                    texture_id: self.texture_id,
                    light,
                },
            ),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub p1: Vector3<f32>,
    pub p2: Vector3<f32>,
    pub p3: Vector3<f32>,
    pub texture_id: u8,
    pub light: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle2D {
    pub p1: Vector2<i16>,
    pub p2: Vector2<i16>,
    pub p3: Vector2<i16>,
    pub texture_id: u8,
    pub light: u8,
}

impl Triangle2D {
    pub fn to_small(&self) -> SmallTriangle2D {
        let value: u64 = ((self.p1.x as u64) << 45)
            | ((self.p1.y as u64) << 36)
            | ((self.p2.x as u64) << 27)
            | ((self.p2.y as u64) << 18)
            | ((self.p3.x as u64) << 9)
            | (self.p3.y as u64);

        let pos = (
            ((value >> 48) & 0xFF) as u8,
            ((value >> 40) & 0xFF) as u8,
            ((value >> 32) & 0xFF) as u8,
            ((value >> 24) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            (value & 0xFF) as u8,
        );

        SmallTriangle2D {
            pos,
            texture_id: self.texture_id,
            light: self.light,
        }
    }
}

pub struct SmallTriangle2D {
    pub pos: (u8, u8, u8, u8, u8, u8, u8),
    pub texture_id: u8,
    pub light: u8,
}

impl SmallTriangle2D {
    pub fn to_tri_2d(&self) -> Triangle2D {
        // Recompose le u64 à partir des 7 u8
        let value: u64 = ((self.pos.0 as u64) << 48)
            | ((self.pos.1 as u64) << 40)
            | ((self.pos.2 as u64) << 32)
            | ((self.pos.3 as u64) << 24)
            | ((self.pos.4 as u64) << 16)
            | ((self.pos.5 as u64) << 8)
            | (self.pos.6 as u64);

        // Extrait chaque coordonnée sur 9 bits
        let p1x = ((value >> 45) & 0x1FF) as i16;
        let p1y = ((value >> 36) & 0x1FF) as i16;
        let p2x = ((value >> 27) & 0x1FF) as i16;
        let p2y = ((value >> 18) & 0x1FF) as i16;
        let p3x = ((value >> 9) & 0x1FF) as i16;
        let p3y = (value & 0x1FF) as i16;

        Triangle2D {
            p1: Vector2::new(p1x, p1y),
            p2: Vector2::new(p2x, p2y),
            p3: Vector2::new(p3x, p3y),
            texture_id: self.texture_id,
            light: self.light,
        }
    }
}

impl Triangle {
    pub fn get_normal(&self) -> Vector3<f32> {
        let a = self.p2 - self.p1;
        let b = self.p3 - self.p1;
        a.cross(&b).normalize()
    }
}

fn get_block_in_chunk_or_world(
    pos: Vector3<isize>,
    chunks_manager: &ChunksManager,
    chunk: &Chunk,
) -> Option<BlockType> {
    if pos.x < 0
        || pos.x >= CHUNK_SIZE_I
        || pos.y < 0
        || pos.y >= CHUNK_SIZE_I
        || pos.z < 0
        || pos.z >= CHUNK_SIZE_I
    {
        chunks_manager.get_block_in_world(pos + *chunk.get_pos() * CHUNK_SIZE_I)
    } else {
        Some(chunk.get_at_unchecked(pos))
    }
}

/// Cheap, deterministic position hash (no floats, no_std friendly) used to fake a
/// per-block texture pattern without adding any triangle or touching the rasterizer.
/// Odd multipliers are used so the pattern doesn't visibly line up with the block grid.
fn hash3(x: isize, y: isize, z: isize) -> u32 {
    let mut h = (x as i32)
        .wrapping_mul(374_761_393)
        ^ (y as i32).wrapping_mul(668_265_263)
        ^ (z as i32).wrapping_mul(-2_147_483_647i32);
    h ^= h >> 13;
    h = h.wrapping_mul(1_274_126_177);
    (h ^ (h >> 16)) as u32
}

/// Small brightness reduction (kept well under the base light levels in
/// `Mesh::get_light_level_from_dir`, so it can never underflow) used to simulate a
/// texture-like pattern per block type using only flat per-face shading. Colored
/// blocks (and anything not listed) intentionally fall back to the original
/// alternating checkerboard so they stay exactly as before.
fn get_block_pattern_shade(block_type: BlockType, x: isize, y: isize, z: isize) -> u16 {
    match block_type {
        // Sparse darker specks
        BlockType::Stone => {
            if hash3(x, y, z) % 7 == 0 {
                2
            } else {
                0
            }
        }
        // Coarser, patchier noise (rounded stone chunks)
        BlockType::Cobblestone => match hash3(x, y, z) % 5 {
            0 => 3,
            1 => 1,
            _ => 0,
        },
        // Grass reuses the dirt texture on its sides, so it shares dirt's pattern
        BlockType::Dirt | BlockType::Grass => match hash3(x, y, z) % 6 {
            0 => 2,
            1 | 2 => 1,
            _ => 0,
        },
        // Fine, subtle grain
        BlockType::Sand => {
            if (x + y + z).rem_euclid(2) == 0 {
                1
            } else {
                0
            }
        }
        // Repeating diagonal bands to suggest bark/wood grain
        BlockType::Log => match (x.rem_euclid(3) + z.rem_euclid(3)).rem_euclid(3) {
            0 => 2,
            1 => 0,
            _ => 1,
        },
        // Sparse irregular gaps (mottled foliage look)
        BlockType::Leaves => match hash3(x, y, z) % 3 {
            0 => 3,
            1 => 1,
            _ => 0,
        },
        // Horizontal board lines (planks, stairs and slab share wood texture)
        BlockType::Planks
        | BlockType::StairsSouth
        | BlockType::StairsNorth
        | BlockType::StairsEast
        | BlockType::StairsWest
        | BlockType::Slab => match y.rem_euclid(4) {
            0 => 1,
            2 => 2,
            _ => 0,
        },
        // Strong, frequent variation so a wall of Tnt doesn't read as flat color
        BlockType::Tnt => match hash3(x, y, z) % 3 {
            0 => 3,
            1 => 1,
            _ => 0,
        },
        // Colored blocks, Border, Air: unchanged, original checkerboard behavior
        _ => {
            if (x + y + z) % 2 == 0 {
                2
            } else {
                0
            }
        }
    }
}

pub struct Mesh {
    pub quads: Vec<Quad>,
    /// Triangles with arbitrary float positions (used for non-cubic block shapes
    /// like stairs). Rendered after quads in the same frame.
    pub direct_triangles: Vec<Triangle>,
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            quads: Vec::new(),
            direct_triangles: Vec::new(),
        }
    }

    pub fn get_reference_vec(&mut self) -> &mut Vec<Quad> {
        &mut self.quads
    }

    const fn get_light_level_from_dir(dir: QuadDir) -> u16 {
        // Please not bellow 2 to avoid negative light. What is neagative light ?
        match dir {
            QuadDir::Front => 13,
            QuadDir::Back => 10,
            QuadDir::Top => 15,
            QuadDir::Bottom => 6,
            QuadDir::Right => 11,
            QuadDir::Left => 10,
        }
    }

    pub fn generate_chunk(chunks_manager: &ChunksManager, chunk: &Chunk) -> Self {
        let mut quads = Vec::new();
        let mut direct_triangles: Vec<Triangle> = Vec::new();

        let chunk_world_pos = chunk.get_pos().map(|v| v * CHUNK_SIZE_I);

        for x in 0..CHUNK_SIZE as isize {
            for y in 0..CHUNK_SIZE as isize {
                for z in 0..CHUNK_SIZE as isize {
                    let block_type = chunk.get_at_unchecked(Vector3::new(x, y, z));
                    if block_type == BlockType::Air {
                        continue;
                    }

                    // Stairs: emit custom float-position triangles, skip quad path
                    if matches!(block_type,
                        BlockType::StairsSouth | BlockType::StairsNorth |
                        BlockType::StairsEast  | BlockType::StairsWest)
                    {
                        let wx = x + chunk_world_pos.x;
                        let wy = y + chunk_world_pos.y;
                        let wz = z + chunk_world_pos.z;
                        let light = Mesh::get_light_level_from_dir(QuadDir::Top) as u8;
                        let tid = block_type.get_texture_id(QuadDir::Top);
                        Mesh::emit_stair_triangles(wx, wy, wz, block_type, light, tid, &mut direct_triangles);
                        continue;
                    }

                    // Slab: flat half-block, same principle
                    if block_type == BlockType::Slab {
                        let wx = x + chunk_world_pos.x;
                        let wy = y + chunk_world_pos.y;
                        let wz = z + chunk_world_pos.z;
                        let light = Mesh::get_light_level_from_dir(QuadDir::Top) as u8;
                        let tid = block_type.get_texture_id(QuadDir::Top);
                        Mesh::emit_slab_triangles(wx, wy, wz, light, tid, &mut direct_triangles);
                        continue;
                    }

                    let bloc_pos = Vector3::new(x as u16, y as u16, z as u16);
                    let grid_additional_light = get_block_pattern_shade(block_type, x, y, z);

                    if get_block_in_chunk_or_world(Vector3::new(x, y, z - 1), chunks_manager, chunk)
                        .is_some_and(|block| block.is_air())
                    {
                        quads.push(Quad::new(
                            bloc_pos,
                            QuadDir::Front,
                            block_type.get_texture_id(QuadDir::Front),
                            Mesh::get_light_level_from_dir(QuadDir::Front) - grid_additional_light,
                        ));
                    }
                    if get_block_in_chunk_or_world(Vector3::new(x, y, z + 1), chunks_manager, chunk)
                        .is_some_and(|block| block.is_air())
                    {
                        quads.push(Quad::new(
                            bloc_pos,
                            QuadDir::Back,
                            block_type.get_texture_id(QuadDir::Back),
                            Mesh::get_light_level_from_dir(QuadDir::Back) - grid_additional_light,
                        ));
                    }
                    if get_block_in_chunk_or_world(Vector3::new(x - 1, y, z), chunks_manager, chunk)
                        .is_some_and(|block| block.is_air())
                    {
                        quads.push(Quad::new(
                            bloc_pos,
                            QuadDir::Right,
                            block_type.get_texture_id(QuadDir::Right),
                            Mesh::get_light_level_from_dir(QuadDir::Right) - grid_additional_light,
                        ));
                    }
                    if get_block_in_chunk_or_world(Vector3::new(x + 1, y, z), chunks_manager, chunk)
                        .is_some_and(|block| block.is_air())
                    {
                        quads.push(Quad::new(
                            bloc_pos,
                            QuadDir::Left,
                            block_type.get_texture_id(QuadDir::Left),
                            Mesh::get_light_level_from_dir(QuadDir::Left) - grid_additional_light,
                        ));
                    }
                    if get_block_in_chunk_or_world(Vector3::new(x, y + 1, z), chunks_manager, chunk)
                        .is_some_and(|block| block.is_air())
                    {
                        quads.push(Quad::new(
                            bloc_pos,
                            QuadDir::Top,
                            block_type.get_texture_id(QuadDir::Top),
                            Mesh::get_light_level_from_dir(QuadDir::Top) - grid_additional_light,
                        ));
                    }
                    if get_block_in_chunk_or_world(Vector3::new(x, y - 1, z), chunks_manager, chunk)
                        .is_some_and(|block| block.is_air())
                    {
                        quads.push(Quad::new(
                            bloc_pos,
                            QuadDir::Bottom,
                            block_type.get_texture_id(QuadDir::Bottom),
                            Mesh::get_light_level_from_dir(QuadDir::Bottom) - grid_additional_light,
                        ));
                    }
                }
            }
        }

        Mesh { quads, direct_triangles }
    }

    /// Emit stair geometry as raw float-position Triangles, in one of 4 orientations.
    ///
    /// The "step" (top half) always faces the direction the player was looking when
    /// they placed the block:
    ///   StairsSouth → step face toward +Z (climb from +Z side)
    ///   StairsNorth → step face toward -Z (climb from -Z side)
    ///   StairsEast  → step face toward +X (climb from +X side)
    ///   StairsWest  → step face toward -X (climb from -X side)
    /// Emit stair geometry with correct winding order (outward normals via cross product).
    /// Winding convention derived from existing Quad::get_triangles():
    ///   top(+Y):    p=(x0,y,z1)→(x1,y,z1)→(x1,y,z0)  and  (x1,y,z0)→(x0,y,z0)→(x0,y,z1)
    ///   bottom(-Y): p=(x1,y,z0)→(x1,y,z1)→(x0,y,z1)  and  (x0,y,z1)→(x0,y,z0)→(x1,y,z0)
    ///   front(-Z):  p=(x1,y1,z)→(x1,y0,z)→(x0,y0,z)  and  (x0,y0,z)→(x0,y1,z)→(x1,y1,z)
    ///   back(+Z):   p=(x0,y0,z)→(x1,y0,z)→(x1,y1,z)  and  (x1,y1,z)→(x0,y1,z)→(x0,y0,z)
    ///   left(x0,+X):p=(x0,y1,z1)→(x0,y1,z0)→(x0,y0,z0) and (x0,y0,z0)→(x0,y0,z1)→(x0,y1,z1)
    ///   right(x1,-X):p=(x1,y0,z0)→(x1,y1,z0)→(x1,y1,z1) and (x1,y1,z1)→(x1,y0,z1)→(x1,y0,z0)
    fn emit_stair_triangles(
        wx: isize, wy: isize, wz: isize,
        variant: BlockType,
        _light: u8, tid: u8,
        out: &mut Vec<Triangle>,
    ) {
        let x0 = wx as f32; let x1 = x0 + 1.0;
        let y0 = wy as f32; let y_mid = y0 + 0.5; let y1 = y0 + 1.0;
        let z0 = wz as f32; let z_mid = z0 + 0.5; let z1 = z0 + 1.0;
        let x_mid = x0 + 0.5;

        let lf = 13u8; let lb = 10u8; let lt = 15u8;
        let lbo = 6u8; let lr = 11u8; let ll = 10u8;

        #[inline(always)]
        fn tri(out: &mut Vec<Triangle>, a: [f32;3], b: [f32;3], c: [f32;3], tid: u8, l: u8) {
            out.push(Triangle {
                p1: nalgebra::Vector3::new(a[0],a[1],a[2]),
                p2: nalgebra::Vector3::new(b[0],b[1],b[2]),
                p3: nalgebra::Vector3::new(c[0],c[1],c[2]),
                texture_id: tid, light: l,
            });
        }
        // top face (+Y normal)
        macro_rules! top { ($x0:expr,$z0:expr,$x1:expr,$z1:expr,$y:expr,$l:expr) => {{
            tri(out,[$x0,$y,$z1],[$x1,$y,$z1],[$x1,$y,$z0],tid,$l);
            tri(out,[$x1,$y,$z0],[$x0,$y,$z0],[$x0,$y,$z1],tid,$l);
        }}}
        // bottom face (-Y normal)
        macro_rules! bot { ($x0:expr,$z0:expr,$x1:expr,$z1:expr,$y:expr,$l:expr) => {{
            tri(out,[$x1,$y,$z0],[$x1,$y,$z1],[$x0,$y,$z1],tid,$l);
            tri(out,[$x0,$y,$z1],[$x0,$y,$z0],[$x1,$y,$z0],tid,$l);
        }}}
        // front face (-Z normal, at z=ZV)
        macro_rules! front { ($x0:expr,$y0:expr,$x1:expr,$y1:expr,$z:expr,$l:expr) => {{
            tri(out,[$x1,$y1,$z],[$x1,$y0,$z],[$x0,$y0,$z],tid,$l);
            tri(out,[$x0,$y0,$z],[$x0,$y1,$z],[$x1,$y1,$z],tid,$l);
        }}}
        // back face (+Z normal, at z=ZV)
        macro_rules! back { ($x0:expr,$y0:expr,$x1:expr,$y1:expr,$z:expr,$l:expr) => {{
            tri(out,[$x0,$y0,$z],[$x1,$y0,$z],[$x1,$y1,$z],tid,$l);
            tri(out,[$x1,$y1,$z],[$x0,$y1,$z],[$x0,$y0,$z],tid,$l);
        }}}
        // left face (+X outward, at x=XV)
        macro_rules! lface { ($z0:expr,$y0:expr,$z1:expr,$y1:expr,$x:expr,$l:expr) => {{
            tri(out,[$x,$y1,$z1],[$x,$y1,$z0],[$x,$y0,$z0],tid,$l);
            tri(out,[$x,$y0,$z0],[$x,$y0,$z1],[$x,$y1,$z1],tid,$l);
        }}}
        // right face (-X outward, at x=XV)
        macro_rules! rface { ($z0:expr,$y0:expr,$z1:expr,$y1:expr,$x:expr,$l:expr) => {{
            tri(out,[$x,$y0,$z0],[$x,$y1,$z0],[$x,$y1,$z1],tid,$l);
            tri(out,[$x,$y1,$z1],[$x,$y0,$z1],[$x,$y0,$z0],tid,$l);
        }}}

        match variant {
            BlockType::StairsSouth => {
                // slab bas (pleine largeur)
                bot!(x0,z0,x1,z1,y0,lbo);
                top!(x0,z0,x1,z_mid,y_mid,lt);  // top exposé moitié avant
                front!(x0,y0,x1,y_mid,z0,lf);
                back!(x0,y0,x1,y_mid,z1,lb);
                lface!(z0,y0,z1,y_mid,x0,ll);
                rface!(z0,y0,z1,y_mid,x1,lr);
                // marche haute (moitié avant)
                top!(x0,z0,x1,z_mid,y1,lt);
                front!(x0,y_mid,x1,y1,z0,lf);
                back!(x0,y_mid,x1,y1,z_mid,lb);  // contremarche
                lface!(z0,y_mid,z_mid,y1,x0,ll);
                rface!(z0,y_mid,z_mid,y1,x1,lr);
            }
            BlockType::StairsNorth => {
                bot!(x0,z0,x1,z1,y0,lbo);
                top!(x0,z_mid,x1,z1,y_mid,lt);  // top exposé moitié arrière
                front!(x0,y0,x1,y_mid,z0,lf);
                back!(x0,y0,x1,y_mid,z1,lb);
                lface!(z0,y0,z1,y_mid,x0,ll);
                rface!(z0,y0,z1,y_mid,x1,lr);
                // marche haute (moitié arrière)
                top!(x0,z_mid,x1,z1,y1,lt);
                front!(x0,y_mid,x1,y1,z_mid,lf); // contremarche
                back!(x0,y_mid,x1,y1,z1,lb);
                lface!(z_mid,y_mid,z1,y1,x0,ll);
                rface!(z_mid,y_mid,z1,y1,x1,lr);
            }
            BlockType::StairsEast => {
                bot!(x0,z0,x1,z1,y0,lbo);
                top!(x0,z0,x_mid,z1,y_mid,lt);  // top exposé moitié gauche
                front!(x0,y0,x1,y_mid,z0,lf);
                back!(x0,y0,x1,y_mid,z1,lb);
                lface!(z0,y0,z1,y_mid,x0,ll);
                rface!(z0,y0,z1,y_mid,x1,lr);
                // marche haute (moitié gauche)
                top!(x0,z0,x_mid,z1,y1,lt);
                front!(x0,y_mid,x_mid,y1,z0,lf);
                back!(x0,y_mid,x_mid,y1,z1,lb);
                lface!(z0,y_mid,z1,y1,x0,ll);
                rface!(z0,y_mid,z1,y1,x_mid,lr); // contremarche
            }
            BlockType::StairsWest => {
                bot!(x0,z0,x1,z1,y0,lbo);
                top!(x_mid,z0,x1,z1,y_mid,lt);  // top exposé moitié droite
                front!(x0,y0,x1,y_mid,z0,lf);
                back!(x0,y0,x1,y_mid,z1,lb);
                lface!(z0,y0,z1,y_mid,x0,ll);
                rface!(z0,y0,z1,y_mid,x1,lr);
                // marche haute (moitié droite)
                top!(x_mid,z0,x1,z1,y1,lt);
                front!(x_mid,y_mid,x1,y1,z0,lf);
                back!(x_mid,y_mid,x1,y1,z1,lb);
                lface!(z0,y_mid,z1,y1,x_mid,ll); // contremarche
                rface!(z0,y_mid,z1,y1,x1,lr);
            }
            _ => {}
        }
    }

    /// Emit a bottom slab (y0 → y0+0.5) as raw float-position Triangles.
    /// The slab sits on the bottom half of the block space — 5 visible faces
    /// (floor, top, front, back, left, right).
    fn emit_slab_triangles(
        wx: isize, wy: isize, wz: isize,
        _light: u8, tid: u8,
        out: &mut Vec<Triangle>,
    ) {
        let x0 = wx as f32; let x1 = x0 + 1.0;
        let y0 = wy as f32; let y1 = y0 + 0.5;
        let z0 = wz as f32; let z1 = z0 + 1.0;

        let lt  = 15u8; // top
        let lbo =  6u8; // bottom
        let lf  = 13u8; // front
        let lb  = 10u8; // back
        let lr  = 11u8; // right
        let ll  = 10u8; // left

        macro_rules! tri2 {
            ($a:expr,$b:expr,$c:expr,$d:expr,$l:expr) => {
                out.push(Triangle { p1:$a, p2:$b, p3:$c, texture_id:tid, light:$l });
                out.push(Triangle { p1:$c, p2:$d, p3:$a, texture_id:tid, light:$l });
            };
        }

        // Bottom face
        tri2!(Vector3::new(x0,y0,z0), Vector3::new(x1,y0,z0),
              Vector3::new(x1,y0,z1), Vector3::new(x0,y0,z1), lbo);
        // Top face (at y0+0.5)
        tri2!(Vector3::new(x0,y1,z0), Vector3::new(x1,y1,z0),
              Vector3::new(x1,y1,z1), Vector3::new(x0,y1,z1), lt);
        // Front face (z0 side)
        tri2!(Vector3::new(x1,y1,z0), Vector3::new(x1,y0,z0),
              Vector3::new(x0,y0,z0), Vector3::new(x0,y1,z0), lf);
        // Back face (z1 side)
        tri2!(Vector3::new(x0,y0,z1), Vector3::new(x1,y0,z1),
              Vector3::new(x1,y1,z1), Vector3::new(x0,y1,z1), lb);
        // Right face (x1 side)
        tri2!(Vector3::new(x1,y0,z0), Vector3::new(x1,y0,z1),
              Vector3::new(x1,y1,z1), Vector3::new(x1,y1,z0), lr);
        // Left face (x0 side)
        tri2!(Vector3::new(x0,y1,z0), Vector3::new(x0,y1,z1),
              Vector3::new(x0,y0,z1), Vector3::new(x0,y0,z0), ll);
    }
}
