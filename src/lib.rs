mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<u128>,
}

impl Universe {
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;

                if self.get_value(neighbor_col, neighbor_row) {
                    count += 1u8;
                }
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let cell = self.get_value(col, row);

                let live_neighbors = self.live_neighbor_count(row, col);

                if (cell && live_neighbors < 2)
                    || (cell && live_neighbors > 3)
                    || (!cell && live_neighbors == 3)
                {
                    next[row as usize] ^= 1u128 << col;
                }
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let mut cells = vec![1; 128];

        for index in 0..cells.len() {
            cells[index] = (index * 2 - 453) as u128;
        }

        Universe {
            width: 128u32,
            height: 128u32,
            cells,
        }
    }

    pub fn get_value(&self, col: u32, row: u32) -> bool {
        self.cells[row as usize] & (1u128 << col) > 0
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
