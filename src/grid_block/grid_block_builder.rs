use super::GridBlock;
use ndarray::Array2;

/// Builder for constructing a immutable `GridBlock`
/// # Example
/// ```
/// use block_builder::grid_block::BlockGridBuilder;
/// // Configure the grid to build
///  let block_grid =
///      // Every not set element has white space as value
///      BlockGridBuilder::with_default(' ')
///      // Every block contains be 2*2 elements
///      .block_size(2)
///      // Grid is 3 blocks wide
///      .blocks_in_x(3)
///      // Grid is 3 blocks high.
///      .blocks_in_y(3)
///      // elements of top left corner block have * as value
///      .set_block_sector(0, 0, '*')
///      // elements of bottom right corner block have * as value
///      .set_block_sector(2, 2, '*')
///      // Build grid according to current configuration
///      .build();
///
/// // Access every element by rows from top left to bottom right.
/// let mut rows: Vec<Vec<char>> = Vec::new();
/// for row in block_grid.iter() {
///     let mut to_push: Vec<char> = Vec::new();
///     for &column in row {
///         // do something with element
///         to_push.push(column);
///     }
///     // Do something the elements in a row
///     rows.push(to_push);
/// }
///
/// assert_eq!(
///     rows,
///     vec![
///         ['*', '*', ' ', ' ', ' ', ' ',],
///         ['*', '*', ' ', ' ', ' ', ' ',],
///         [' ', ' ', ' ', ' ',' ', ' ',],
///         [' ', ' ', ' ', ' ', ' ', ' ',],
///         [' ', ' ', ' ', ' ', '*', '*',],
///         [' ', ' ', ' ', ' ', '*', '*',],
///     ]
/// )
/// ```
pub struct BlockGridBuilder<T> {
    block_size: usize,
    blocks_in_x: usize,
    blocks_in_y: usize,
    default_value: T,
    setting_blocks: Vec<(usize, usize, T)>,
}

impl<T> Default for BlockGridBuilder<T>
where
    T: Default + Clone,
{
    fn default() -> Self {
        Self::init(Default::default())
    }
}

impl<T> BlockGridBuilder<T>
where
    T: Clone,
{
    /// Every not set element with have the value `default_value`
    pub fn with_default(default_value: T) -> Self {
        Self::init(default_value)
    }
    /// How big a block in the grid is.
    /// Grid will be [`block_size`] * [`blocks_in_y`] * [`blocks_in_x`] elements large.
    ///
    /// # Panic
    ///
    /// If `new_block_size` is zero
    pub fn block_size(&mut self, new_block_size: usize) -> &mut Self {
        if new_block_size < 1 {
            panic!("{} must be not be zero", stringify!(block_size));
        }

        self.block_size = new_block_size;

        self
    }

    pub fn reset_sectors(&mut self) -> &mut Self {
        self.setting_blocks.clear();

        self
    }

    /// All values in the block will be `value`.
    /// Example: given (1, 1) with * as value => value at (2, 2), (3, 2), (2, 3) (3, 3) will be * in
    /// constructed grid with `block_size` of 2.
    pub fn set_bulk_sectors(&mut self, value: T, sectors: &[(usize, usize)]) -> &mut Self {
        for (x, y) in sectors {
            self.setting_blocks.push((*x, *y, value.clone()));
        }

        self
    }

    /// # Panic
    ///
    /// If [`blocks_in_x`] and [`blocks_in_y`] are both zero after calling this method.
    pub fn blocks_in_x(&mut self, new_blocks_in_x: usize) -> &mut Self {
        self.validate_x_y();

        self.blocks_in_x = new_blocks_in_x;

        self
    }

    /// # Panic
    ///
    /// If [`blocks_in_x`] and [`blocks_in_y`] are both zero after calling this method.
    pub fn blocks_in_y(&mut self, new_blocks_in_y: usize) -> &mut Self {
        self.validate_x_y();

        self.blocks_in_y = new_blocks_in_y;

        self
    }

    pub fn set_block_sector(&mut self, x: usize, y: usize, value: T) -> &mut Self {
        self.setting_blocks.push((x, y, value));
        self
    }

    pub fn build(&self) -> GridBlock<T> {
        let block_size = self.block_size;
        let width = block_size * self.blocks_in_x;
        let height = block_size * self.blocks_in_y;
        let mut build: Array2<T> = Array2::from_elem((height, width), self.default_value.clone());

        for (x, y, to_insert) in self.setting_blocks.iter() {
            let to_add = block_size - 1;

            let (scaled_x, scaled_y) = (*x * block_size, *y * block_size);

            let max_x = scaled_x + to_add;
            let max_y = scaled_y + to_add;

            for next_y in scaled_y..=max_y {
                for next_x in scaled_x..=max_x {
                    let insert_into = build
                        .get_mut((next_y, next_x))
                        .expect("Unexpected: during insertion of value out of bound indexing");
                    *insert_into = to_insert.clone();
                }
            }
        }

        GridBlock::new(build, width, height)
    }

    fn init(value: T) -> Self {
        Self {
            block_size: 1,
            blocks_in_x: 1,
            blocks_in_y: 1,
            setting_blocks: Default::default(),
            default_value: value,
        }
    }

    fn validate_x_y(&self) {
        if self.blocks_in_x < 1 && self.blocks_in_y < 1 {
            panic!(
                "{} and {} must be not be zero",
                stringify!(blocks_in_x),
                stringify!(blocks_in_y)
            );
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    #[test]
    fn should_set_sectors() {
        let block = BlockGridBuilder::with_default(' ')
            .blocks_in_y(3)
            .blocks_in_x(4)
            .block_size(2)
            .set_block_sector(0, 0, '*')
            .set_block_sector(1, 0, '*')
            .set_block_sector(2, 0, '*')
            .set_block_sector(2, 1, '*')
            .set_block_sector(2, 2, '*')
            .set_block_sector(3, 0, '*')
            .set_block_sector(0, 1, '*')
            .build();

        insta::assert_display_snapshot!(block);
    }

    #[test]
    #[rustfmt::skip]
    fn doc_test() {
        
   }

    #[test]
    fn should_should_produce_nine() {
        let block = create_base_nine().build();

        insta::assert_display_snapshot!(block);
    }
    #[test]
    fn should_should_produce_nine_x3() {
        let block = create_base_nine().block_size(3).build();

        insta::assert_display_snapshot!(block);
    }

    fn create_base_nine() -> BlockGridBuilder<char> {
        let mut block = BlockGridBuilder::with_default(' ');
        block
            .blocks_in_y(5)
            .blocks_in_x(3)
            .block_size(1)
            // ***
            // * *
            // ***
            //   *
            // ***
            //
            // ***
            .set_block_sector(0, 0, '*')
            .set_block_sector(1, 0, '*')
            .set_block_sector(2, 0, '*')
            // * *
            .set_block_sector(0, 1, '*')
            .set_block_sector(2, 1, '*')
            // ***
            .set_block_sector(0, 2, '*')
            .set_block_sector(1, 2, '*')
            .set_block_sector(2, 2, '*')
            //   *
            .set_block_sector(2, 3, '*')
            // ***
            .set_block_sector(0, 4, '*')
            .set_block_sector(1, 4, '*')
            .set_block_sector(2, 4, '*');
        block
    }
}
