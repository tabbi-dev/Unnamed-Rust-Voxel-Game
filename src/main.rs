struct Pos {    //
    x: f64,
    y: f64,
    z: f64,
    rx: f32,
    ry: f32
}

struct BlockPos {   // Unsigned int bc there is no negative chunk coordinate value, chunks are built from position 0 -> 32
    x: u8,
    y: u8,
    z: u8
}

struct ChunkPos {   // Chunk position relative to the world's Chunk Grid - position values can be negative
    x: i16,
    y: i16,
    z: i16
}
struct VoxelPos {  // Each block will contain 16^2 voxels
    x: u8,
    y: u8,
    z: u8
}

struct Colour {
    RGB: (u8,u8,u8), // Basic RGB which will be assigned to each voxel face based on its material
    Opacity: u8 // constrained to a max of 100
}; // yes this struct is spelled properly, get over yourself you yank bastard

// I've made this bottle of Bacardi my bitch over the last few weeks
// Surprised its lasted me this long tbh
// OMG Lemon Lime & Bitters soda w bacardi is PEAK
// I am transcending humanity, time to lock the FUCK IN

struct Player {
    name: String,
    uuid: String,
    mode: u8,
    age: u32,
    position: Pos
    /*
    height:  block */
}

struct Voxel {
    id: u16,        // Allows for 65,536 block types (incl. null (id=0))
    blockstate: u8, // 255 possible variations / blockstates
    colors: [Colour;6],  // voxel color for each side - will probably be moved to block type allocation
    pos: VoxelPos
}

struct Block {
    pos: BlockPos,
    voxel_array: [[Voxel; 24]; 24]
}

struct Chunk {
    pos: ChunkPos,
    block_array: [[Block; 32]; 32] // 65536 (256^2) blocks per Chunk
}

/*  I was thinking of grouping chunks into clusters, but nah I dont feel like it
struct Cluster {

}
*/




fn main() {
    println!(
        "u8:{}, u16:{}, u32:{}, u64:{}\ni8:{}, i16:{}, i32:{}, i64:{}",
        u8::MAX, u16::MAX, u32:: MAX, u64::MAX, i8::MAX,i16::MAX,i32::MAX,i64::MAX
    )
}
