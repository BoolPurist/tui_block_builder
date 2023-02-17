mod line_block_builder;

pub use line_block_builder::LineBlockBuilder;

use crate::grid_block::BlockGridBuilder;
use crate::{ascii_art_lib, grid_block::GridBlock};
use tui::text::Spans;
use tui::{
    style::{Color, Style},
    text::Span,
};

macro_rules! tui_block {
    ($fn_name:ident, $builder_name:ident) => {
        pub fn $fn_name(default_bg: Color, bg: Color) -> BlockGridBuilder<Span<'static>> {
            let builder = ascii_art_lib::$builder_name(
                Span::styled(" ", Style::default().bg(default_bg)),
                Span::styled(" ", Style::default().bg(bg)),
            );

            builder
        }
    };
}

/// **Panics** if blocks in slice `to_clue` do not have the same height
pub fn build_tui_line_block(to_clue: &[GridBlock<Span<'static>>]) -> Vec<Spans<'static>> {
    let max = to_clue
        .iter()
        .map(|block| block.height())
        .max_by_key(|&block| block)
        .unwrap_or(0);
    let mut all_spans: Vec<Spans<'static>> = Vec::with_capacity(max);

    for row in GridBlock::iter_top_left_bottom_right(to_clue, max) {
        let mut spans = Vec::new();
        for symbol in row {
            spans.push(symbol.clone());
        }

        all_spans.push(Spans::from(spans));
    }

    all_spans
}

pub fn create_tui_block_space(default_bg: Color) -> BlockGridBuilder<Span<'static>> {
    let builder = ascii_art_lib::build_space(Span::styled(" ", Style::default().bg(default_bg)));

    builder
}

tui_block! {create_tui_block_1, build_1}
tui_block! {create_tui_block_2, build_2}
tui_block! {create_tui_block_3, build_3}
tui_block! {create_tui_block_4, build_4}
tui_block! {create_tui_block_5, build_5}
tui_block! {create_tui_block_6, build_6}
tui_block! {create_tui_block_7, build_7}
tui_block! {create_tui_block_8, build_8}
tui_block! {create_tui_block_9, build_9}
tui_block! {create_tui_block_0, build_0}
tui_block! {create_tui_block_double_point, build_double_point}
