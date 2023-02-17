use std::collections::HashMap;

use tui::{
    style::Color,
    text::{Span, Spans},
};

use crate::grid_block::{BlockGridBuilder, GridBlock};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static NUMBER_MAPPER: Lazy<
    Mutex<HashMap<u32, fn(Color, Color) -> BlockGridBuilder<Span<'static>>>>,
> = Lazy::new(|| {
    let mut map: HashMap<u32, fn(Color, Color) -> BlockGridBuilder<Span<'static>>> = HashMap::new();
    map.insert(0, super::create_tui_block_0);
    map.insert(1, super::create_tui_block_1);
    map.insert(2, super::create_tui_block_2);
    map.insert(3, super::create_tui_block_3);
    map.insert(4, super::create_tui_block_4);
    map.insert(5, super::create_tui_block_5);
    map.insert(6, super::create_tui_block_6);
    map.insert(7, super::create_tui_block_7);
    map.insert(8, super::create_tui_block_8);
    map.insert(9, super::create_tui_block_9);
    Mutex::new(map)
});

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

    pub fn number(&mut self, mut number: u32) -> &mut Self {
        let mapper = NUMBER_MAPPER
            .lock()
            .expect("Unexpected: somewhere else poisned the mutext to the number mapper.");

        let mut buffer: Vec<BlockGridBuilder<Span<'static>>> = Vec::new();

        loop {
            let rest = number % 10;

            let to_invoke = mapper
                .get(&rest)
                .expect("Unexpected: a number as digit could not mapped.");

            let next = (to_invoke)(self.default_value, self.taken_value);
            buffer.push(next);
            buffer.push(super::create_tui_block_space(self.default_value));

            number /= 10;

            if number == 0 {
                break;
            }
        }

        _ = buffer.pop();
        buffer.reverse();

        self.builders.append(&mut buffer);

        self
    }
}
