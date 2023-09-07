use std::collections::{HashSet, VecDeque};

use crate::cell::FieldCell;

#[derive(Debug)]
pub struct CellCounter {
    pub visited_cells: HashSet<FieldCell>,
    max_coord_sum: i64,
    new_cells: VecDeque<FieldCell>,
}

impl CellCounter {
    pub fn new(max_coord_sum: i64, init_cell: FieldCell) -> Self {
        Self {
            max_coord_sum,
            visited_cells: HashSet::new(),
            new_cells: VecDeque::from([init_cell]),
        }
    }

    pub fn available_cells_count(&mut self) -> Option<u64> {
        while !self.new_cells.is_empty() {
            let current = self.new_cells.pop_front()?;

            if !self.is_sum_fit(&current) || self.visited_cells.contains(&current) {
                continue;
            }

            self.new_cells.push_back(current.get_top());
            self.new_cells.push_back(current.get_bot());
            self.new_cells.push_back(current.get_left());
            self.new_cells.push_back(current.get_right());

            self.visited_cells.insert(current);
        }

        Some(self.visited_cells.len() as u64)
    }

    fn is_sum_fit(&self, cell: &FieldCell) -> bool {
        cell.coords_digits_sum() <= self.max_coord_sum
    }
}
