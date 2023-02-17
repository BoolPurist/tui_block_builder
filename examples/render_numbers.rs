use block_builder::grid_block::{BlockGridBuilder, GridBlock};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Layout},
    style::Color,
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();

    let layout = Layout::default()
        .margin(5)
        .constraints([Constraint::Percentage(100)])
        .split(size);

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Some Block numbers")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let numbers = create_numbers_render();

    let para = Paragraph::new(numbers)
        .block(Block::default())
        .alignment(Alignment::Center);

    f.render_widget(block, size);
    f.render_widget(para, layout[0]);
}

fn create_numbers_render() -> Vec<Spans<'static>> {
    use block_builder::tui_block;
    const BLOCK_SIZE: usize = 2;
    const NONE_COLOR: Color = Color::Black;
    const TAKEN_COLOR: Color = Color::White;

    let one = create_tui_block(tui_block::create_tui_block_1);

    let zero = create_tui_block(tui_block::create_tui_block_0);
    let two = create_tui_block(tui_block::create_tui_block_2);
    let seven = create_tui_block(tui_block::create_tui_block_7);
    let double_point = create_tui_block(tui_block::create_tui_block_double_point);

    let space = tui_block::create_tui_block_space(NONE_COLOR)
        .block_size(BLOCK_SIZE)
        .build();

    return tui_block::build_tui_line_block(&[
        one,
        space.clone(),
        zero.clone(),
        double_point,
        two,
        space.clone(),
        zero,
        space,
        seven,
    ]);

    fn create_tui_block(
        on_create: impl Fn(Color, Color) -> BlockGridBuilder<Span<'static>>,
    ) -> GridBlock<Span<'static>> {
        on_create(NONE_COLOR, TAKEN_COLOR)
            .block_size(BLOCK_SIZE)
            .build()
    }
}
