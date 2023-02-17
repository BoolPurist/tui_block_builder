use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
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
    let mut counter = 0;
    loop {
        terminal.draw(|f| ui(f, counter))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    return Ok(());
                }
                KeyCode::Char('a') => {
                    counter += 1;
                }
                _ => (),
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, counter: u32) {
    let size = f.size();

    let layout = Layout::default()
        .margin(5)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(size);

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Some Block numbers")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let numbers = create_numbers_render(counter);

    let para = Paragraph::new(numbers)
        .block(Block::default())
        .alignment(Alignment::Center);
    let help_lines: Vec<Spans<'static>> = vec![
        Spans::from(vec![Span::styled(
            "Press q to quit",
            Style::default().fg(Color::Red),
        )]),
        Spans::from(vec![Span::styled(
            "Press a to increment to counter",
            Style::default().fg(Color::Blue),
        )]),
    ];

    let help_text = Paragraph::new(help_lines)
        .block(Block::default())
        .alignment(Alignment::Center);

    f.render_widget(block, size);
    f.render_widget(para, layout[0]);
    f.render_widget(help_text, layout[2]);
}

fn create_numbers_render(counter: u32) -> Vec<Spans<'static>> {
    use block_builder::tui_block::LineBlockBuilder;
    const BLOCK_SIZE: usize = 2;
    const NONE_COLOR: Color = Color::Black;
    const TAKEN_COLOR: Color = Color::White;

    LineBlockBuilder::new(BLOCK_SIZE, TAKEN_COLOR, NONE_COLOR)
        .number(counter)
        .build_line()
}
