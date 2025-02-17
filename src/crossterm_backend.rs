use crate::{
    app::App,
    ui::draw
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use core::time;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend}, terminal, Terminal
};

pub fn run() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let app = App::new("My App");
    run_app_loop(&mut terminal, app)?;
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, mut app: App) -> io::Result<()> {
    let mut last_tick = Instant::now(); 
    loop {
        terminal.draw(|f| draw(f, &mut app))?;
        let timeout = Duration::from_millis(1000 / 60);
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    KeyCode::Char('q') => app.on_quit(),
                    KeyCode::Left => app.on_left(),
                    KeyCode::Right => app.on_right(),
                    KeyCode::Down => app.on_down(),
                    KeyCode::Up => app.on_up(),
                    KeyCode::Char('p') => app.play(),
                    _ => {}
                }
            }
        }
        if app.should_quit {
            return Ok(());
        }
    }
}