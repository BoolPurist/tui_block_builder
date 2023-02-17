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
    use block_builder::tui_block::LineBlockBuilder;
    const BLOCK_SIZE: usize = 2;
    const NONE_COLOR: Color = Color::Black;
    const TAKEN_COLOR: Color = Color::White;

    LineBlockBuilder::new(BLOCK_SIZE, TAKEN_COLOR, NONE_COLOR)
        .one()
        .space()
        .zero()
        .seperator()
        .two()
        .space()
        .zero()
        .space()
        .seven()
        .build_line()
}
