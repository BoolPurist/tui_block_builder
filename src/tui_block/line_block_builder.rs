use tui::{
    style::Color,
    text::{Span, Spans},
};

use crate::grid_block::{BlockGridBuilder, GridBlock};

pub struct LineBlockBuilder {
    block_size: usize,
    taken_value: Color,
    default_value: Color,
    builders: Vec<BlockGridBuilder<Span<'static>>>,
}

macro_rules! next_block {
    ($name:ident, $func:ident) => {
        pub fn $name(&mut self) -> &mut Self {
            self.builders
                .push(super::$func(self.default_value, self.taken_value));

            self
        }
    };
}
impl LineBlockBuilder {
    pub fn new(block_size: usize, taken_value: Color, default_value: Color) -> Self {
        Self {
            taken_value,
            default_value,
            block_size,
            builders: Default::default(),
        }
    }

    pub fn space(&mut self) -> &mut Self {
        self.builders
            .push(super::create_tui_block_space(self.default_value));

        self
    }

    next_block! {one, create_tui_block_1}
    next_block! {two, create_tui_block_2}
    next_block! {three, create_tui_block_3}
    next_block! {four, create_tui_block_4}
    next_block! {five, create_tui_block_5}
    next_block! {six, create_tui_block_6}
    next_block! {seven, create_tui_block_7}
    next_block! {eight, create_tui_block_8}
    next_block! {nine, create_tui_block_9}
    next_block! {zero, create_tui_block_0}
    next_block! {seperator, create_tui_block_double_point}

    pub fn build_line(&mut self) -> Vec<Spans<'static>> {
        let were_build: Vec<GridBlock<Span<'static>>> = self
            .builders
            .iter_mut()
            .map(|builder| builder.block_size(self.block_size).build())
            .collect();

        super::build_tui_line_block(&were_build)
    }
}
