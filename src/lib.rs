mod cell;
mod cell_counter;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{cell::FieldCell, cell_counter::CellCounter};

    #[test]
    fn answer_as_a_test() {
        let init_cell = FieldCell { x: 1000, y: 1000 };

        let mut cell_counter = CellCounter::new(25, init_cell);
        let count = cell_counter.available_cells_count().unwrap();
        println!("Answer: {:}", count);
    }

    #[test]
    fn small_max_coord_test() {
        let init_cell = FieldCell { x: 1000, y: 1000 };

        let mut cell_counter = CellCounter::new(1, init_cell);
        assert_eq!(cell_counter.available_cells_count().unwrap(), 0)
    }

    #[test]
    fn one_cell_test() {
        let init_cell = FieldCell { x: 1000, y: 1000 };

        let mut cell_counter = CellCounter::new(2, init_cell);
        assert_eq!(cell_counter.available_cells_count().unwrap(), 1)
    }

    #[test]
    fn max_sum_4_test() {
        //    999,1002  [1000,1002]   1001,1002    1002,1002
        //
        //    999,1001  [1000,1001]  [1001,1001]   1002,1001
        //
        //    999,1000  [1000,1000]  [1001,1000]  [1002,1000]
        //
        //    999,999    1000,999     1001,999     1002,999

        let init_cell = FieldCell { x: 1000, y: 1000 };

        let mut cell_counter = CellCounter::new(4, init_cell);
        assert_eq!(cell_counter.available_cells_count().unwrap(), 6);

        let reference = HashSet::from([
            FieldCell { x: 1000, y: 1002 },
            FieldCell { x: 1000, y: 1002 },
            FieldCell { x: 1000, y: 1001 },
            FieldCell { x: 1000, y: 1000 },
            FieldCell { x: 1001, y: 1000 },
            FieldCell { x: 1001, y: 1001 },
            FieldCell { x: 1002, y: 1000 },
        ]);
        assert_eq!(cell_counter.visited_cells, reference);
    }
}
