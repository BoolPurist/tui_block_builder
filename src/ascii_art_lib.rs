//! Provides helper functions to build block like numbers, : and empty white space seperator

use crate::grid_block::BlockGridBuilder;

const TOP_LEFT: (usize, usize) = (0, 0);
const TOP_CENTER: (usize, usize) = (1, 0);
const TOP_RIGHT: (usize, usize) = (2, 0);

const UPPER_LEFT: (usize, usize) = (0, 1);
const UPPER_RIGHT: (usize, usize) = (2, 1);

const MIDDLE_LEFT: (usize, usize) = (0, 2);
const MIDDLE_CENTER: (usize, usize) = (1, 2);
const MIDDLE_RIGHT: (usize, usize) = (2, 2);

const LOWER_LEFT: (usize, usize) = (0, 3);
const LOWER_RIGHT: (usize, usize) = (2, 3);

const BOTTOM_LEFT: (usize, usize) = (0, 4);
const BOTTOM_CENTER: (usize, usize) = (1, 4);
const BOTTOM_RIGHT: (usize, usize) = (2, 4);

pub fn build_1<T>(default_v: T, taken_v: T) -> BlockGridBuilder<T>
where
    T: Clone,
{
    let mut builder = build_thin_number_base(default_v);
    builder.reset_sectors().set_bulk_sectors(
        taken_v,
        &[
            TOP_RIGHT,
            UPPER_RIGHT,
            MIDDLE_RIGHT,
            LOWER_RIGHT,
            BOTTOM_RIGHT,
        ],
    );
    builder
}

#[rustfmt::skip]
pub fn build_2<T>(default_v: T, taken_v: T) -> BlockGridBuilder<T>
where
    T: Clone,
{
    let mut builder = build_thin_number_base(default_v);
    builder.set_bulk_sectors(
        taken_v,
        &[
            TOP_LEFT,    TOP_CENTER,    TOP_RIGHT,
                                        UPPER_RIGHT,
            MIDDLE_LEFT, MIDDLE_CENTER, MIDDLE_RIGHT,
            LOWER_LEFT,
            BOTTOM_LEFT, BOTTOM_CENTER, BOTTOM_RIGHT,
        ],
    );
    builder
}

#[rustfmt::skip]
pub fn build_3<T>(default_v: T, taken_v: T) -> BlockGridBuilder<T>
where
    T: Clone,
{
    let mut builder = build_thin_number_base(default_v);
    builder.set_bulk_sectors(
        taken_v,
        &[
            TOP_LEFT,    TOP_CENTER,    TOP_RIGHT,
                                        UPPER_RIGHT,
            MIDDLE_LEFT, MIDDLE_CENTER, MIDDLE_RIGHT,
                                        LOWER_RIGHT,
            BOTTOM_LEFT, BOTTOM_CENTER, BOTTOM_RIGHT,
        ],
    );
    builder
}

#[rustfmt::skip]
pub fn build_4<T>(default_v: T, taken_v: T) -> BlockGridBuilder<T>
where
    T: Clone,
{
    let mut builder = build_thin_number_base(default_v);
    builder.set_bulk_sectors(
        taken_v,
        &[
            TOP_LEFT,                   TOP_RIGHT,
            UPPER_LEFT,                 UPPER_RIGHT,
            MIDDLE_LEFT, MIDDLE_CENTER, MIDDLE_RIGHT,
                                        LOWER_RIGHT,
                                        BOTTOM_RIGHT,
        ],
    );
    builder
}
#[rustfmt::skip]
pub fn build_5<T>(default_v: T, taken_v: T) -> BlockGridBuilder<T>
where
    T: Clone,
{
    let mut builder = build_thin_number_base(default_v);
    builder.set_bulk_sectors(
        taken_v,
        &[
            TOP_LEFT,    TOP_CENTER,    TOP_RIGHT,
            UPPER_LEFT,                 
            MIDDLE_LEFT, MIDDLE_CENTER, MIDDLE_RIGHT,
                                        LOWER_RIGHT,
            BOTTOM_LEFT, BOTTOM_CENTER, BOTTOM_RIGHT,
        ],
    );
    builder
}
#[rustfmt::skip]
pub fn build_6<T>(default_v: T, taken_v: T) -> BlockGridBuilder<T>
where
    T: Clone,
{
    let mut builder = build_thin_number_base(default_v);
    builder.set_bulk_sectors(
        taken_v,
        &[
            TOP_LEFT,    TOP_CENTER,    TOP_RIGHT,
            UPPER_LEFT,                 
            MIDDLE_LEFT, MIDDLE_CENTER, MIDDLE_RIGHT,
            LOWER_LEFT,                 LOWER_RIGHT,
            BOTTOM_LEFT, BOTTOM_CENTER, BOTTOM_RIGHT,
        ],
    );
    builder
}
#[rustfmt::skip]
pub fn build_7<T>(default_v: T, taken_v: T) -> BlockGridBuilder<T>
where
    T: Clone,
{
    let mut builder = build_thin_number_base(default_v);
    builder.set_bulk_sectors(
        taken_v,
        &[
            TOP_LEFT,    TOP_CENTER,    TOP_RIGHT,
                                        UPPER_RIGHT, 
                                        MIDDLE_RIGHT,
                                        LOWER_RIGHT,
                                        BOTTOM_RIGHT,
        ],
    );
    builder
}
#[rustfmt::skip]
pub fn build_8<T>(default_v: T, taken_v: T) -> BlockGridBuilder<T>
where
    T: Clone,
{
    let mut builder = build_thin_number_base(default_v);
    builder.set_bulk_sectors(
        taken_v,
        &[
            TOP_LEFT,    TOP_CENTER,    TOP_RIGHT,
            UPPER_LEFT,                 UPPER_RIGHT, 
            MIDDLE_LEFT, MIDDLE_CENTER, MIDDLE_RIGHT,
            LOWER_LEFT,                 LOWER_RIGHT,
            BOTTOM_LEFT, BOTTOM_CENTER, BOTTOM_RIGHT,
        ],
    );
    builder
}
#[rustfmt::skip]
pub fn build_9<T>(default_v: T, taken_v: T) -> BlockGridBuilder<T>
where
    T: Clone,
{
    let mut builder = build_thin_number_base(default_v);
    builder.set_bulk_sectors(
        taken_v,
        &[
            TOP_LEFT,    TOP_CENTER,    TOP_RIGHT,
            UPPER_LEFT,                 UPPER_RIGHT, 
            MIDDLE_LEFT, MIDDLE_CENTER, MIDDLE_RIGHT,
                                        LOWER_RIGHT,
            BOTTOM_LEFT, BOTTOM_CENTER, BOTTOM_RIGHT,
        ],
    );
    builder
}
#[rustfmt::skip]
pub fn build_0<T>(default_v: T, taken_v: T) -> BlockGridBuilder<T>
where
    T: Clone,
{
    let mut builder = build_thin_number_base(default_v);
    builder.set_bulk_sectors(
        taken_v,
        &[
            TOP_LEFT,    TOP_CENTER,    TOP_RIGHT,
            UPPER_LEFT,                 UPPER_RIGHT, 
            MIDDLE_LEFT,                MIDDLE_RIGHT,
            LOWER_LEFT,                 LOWER_RIGHT,
            BOTTOM_LEFT, BOTTOM_CENTER, BOTTOM_RIGHT,
        ],
    );
    builder
}
pub fn build_double_point<T>(default_v: T, taken_v: T) -> BlockGridBuilder<T>
where
    T: Clone,
{
    let mut builder = build_thin_number_base(default_v);
    builder.set_bulk_sectors(taken_v, &[(1, 1), (1, 3)]);
    builder
}
pub fn build_space<T>(default_v: T) -> BlockGridBuilder<T>
where
    T: Clone,
{
    let mut builder = BlockGridBuilder::with_default(default_v);
    builder.block_size(1).blocks_in_x(1).blocks_in_y(3);
    builder
}

pub fn build_thin_number_base<T>(default_v: T) -> BlockGridBuilder<T>
where
    T: Clone,
{
    let mut builder = BlockGridBuilder::with_default(default_v);
    builder.block_size(1).blocks_in_x(3).blocks_in_y(5);
    builder
}

#[cfg(test)]
mod testing {
    use super::*;
    #[test]
    fn should_build_1() {
        let one = build_1(' ', '*').build();
        insta::assert_display_snapshot!(one);
    }
    #[test]
    fn should_build_2() {
        let two = build_2(' ', '*').build();
        insta::assert_display_snapshot!(two);
    }
    #[test]
    fn should_build_3() {
        let two = build_3(' ', '*').build();
        insta::assert_display_snapshot!(two);
    }
    #[test]
    fn should_build_4() {
        let four = build_4(' ', '*').build();
        insta::assert_display_snapshot!(four);
    }
    #[test]
    fn should_build_5() {
        let five = build_5(' ', '*').build();
        insta::assert_display_snapshot!(five);
    }
    #[test]
    fn should_build_6() {
        let six = build_6(' ', '*').build();
        insta::assert_display_snapshot!(six);
    }
    #[test]
    fn should_build_7() {
        let seven = build_7(' ', '*').build();
        insta::assert_display_snapshot!(seven);
    }
    #[test]
    fn should_build_8() {
        let eight = build_8(' ', '*').build();
        insta::assert_display_snapshot!(eight);
    }
    #[test]
    fn should_build_9() {
        let nine = build_9(' ', '*').build();
        insta::assert_display_snapshot!(nine);
    }
    #[test]
    fn should_build_0() {
        let zero = build_0(' ', '*').build();
        insta::assert_display_snapshot!(zero);
    }
    #[test]
    fn should_build_double_point() {
        let double_point = build_double_point(' ', '*').build();
        insta::assert_display_snapshot!(double_point);
    }
    #[test]
    fn should_space() {
        let space = build_space(' ').build();
        insta::assert_display_snapshot!(space);
    }
}
