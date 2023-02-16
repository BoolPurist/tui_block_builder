//! Grid system to composite blocks

use core::fmt::{Debug, Display};
use ndarray::Array2;

mod grid_block_builder;
pub use grid_block_builder::BlockGridBuilder;

/// Grid which is made up in blocks . Every block has the same size. A block has size * size
/// elements.
/// Every grid has a number of blocks in x and y direction.
/// The whole API is exposed for immutable access only.
#[derive(Debug)]
pub struct GridBlock<T> {
    grid: Array2<T>,
    width: usize,
    height: usize,
}

impl<T> GridBlock<T> {
    fn new(grid: Array2<T>, width: usize, height: usize) -> Self {
        Self {
            grid,
            width,
            height,
        }
    }
}

impl<T> GridBlock<T> {
    /// Number of elements in x direction
    pub fn width(&self) -> usize {
        self.width
    }

    /// Number of elements in y direction
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.grid.get((y, x))
    }

    /// Get all elements along a y axis
    pub fn get_row_at(&self, y: usize) -> Option<&[T]> {
        self.grid.row(y).to_slice()
    }

    /// Yields all elements of blocks given in param `grid` along on given y axis `y`
    /// This operation can be used to traverse by rows as if the given block are one.
    pub fn iter_along_x_axis(grid: &[GridBlock<T>], y: usize) -> impl Iterator<Item = &T> {
        grid.iter()
            .filter_map(move |next_grid| next_grid.get_row_at(y))
            .flat_map(|row| row.iter())
    }

    pub fn iter(&self) -> impl Iterator<Item = &[T]>
    where
        T: Clone,
    {
        (0..self.height()).map(|index| self.get_row_at(index).unwrap())
    }
    /// Yields elements of given `grid` for traversing from top left to bottom right corner by
    /// rows. It stops at row `max_len`.
    pub fn iter_top_left_bottom_right(
        grid: &[GridBlock<T>],
        max_len: usize,
    ) -> impl Iterator<Item = Vec<&T>> {
        (0..max_len).map(|index| Self::iter_along_x_axis(grid, index).collect())
    }
}

impl<T> Display for GridBlock<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use crate::ascii_art_lib;
    #[test]
    fn should_get_row_at() {
        const UNEXPECTED_MSG: &str = "No zeroth row for given block in test";
        let block = BlockGridBuilder::with_default(' ')
            .block_size(2)
            .blocks_in_x(3)
            .blocks_in_y(3)
            .set_block_sector(0, 0, '*')
            .set_block_sector(2, 0, '^')
            .set_block_sector(0, 1, 'x')
            .build();

        let row = block.get_row_at(0).expect(UNEXPECTED_MSG);

        let third_row = block.get_row_at(2).expect(UNEXPECTED_MSG);
        insta::assert_yaml_snapshot!("1 and 5 row", &[row, third_row]);
    }

    #[test]
    fn should_traverse_from_top_left_to_bottom_right() {
        let one = ascii_art_lib::build_1(" ", "*").build();
        let double_point = ascii_art_lib::build_double_point(" ", "x").build();
        let four = ascii_art_lib::build_4(" ", "*").build();

        let actual: Vec<String> =
            GridBlock::iter_top_left_bottom_right(&vec![one, double_point, four], 5)
                .map(|line| {
                    line.into_iter()
                        .map(|symbol| symbol.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                })
                .collect();

        insta::assert_yaml_snapshot!(actual);
    }
}
