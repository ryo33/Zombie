use std::collections::HashMap;
use super::CHUNK_SIZE;

#[derive(Clone)]
pub struct Chunk {
    tiles: [[i32; CHUNK_SIZE]; CHUNK_SIZE],
    // tiles_info: HashMap<(i32, i32), TileInfo>,
    // entities: Vec<Entity>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            tiles: [
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
        }
    }

    pub fn iter(&self) -> ChunkIterator {
        ChunkIterator {
            chunk: self.clone(),
            x: 0,
            y: 0,
        }
    }
}

struct ChunkIterator{
    chunk: Chunk,
    x: i32,
    y: i32,
}

impl Iterator for ChunkIterator {
    type Item = (i32, i32, i32);
    fn next(&mut self) -> Option<(i32, i32, i32)> {
        let item;
        if self.y == CHUNK_SIZE as i32 {
            item = None
        } else {
            item = Some((self.x, self.y, self.chunk.tiles[self.y as usize][self.x as usize]))
        }
        if self.x == CHUNK_SIZE as i32 - 1 {
            self.x = 0;
            self.y = self.y + 1;
        } else {
            self.x = self.x + 1;
        }
        item
    }
}
