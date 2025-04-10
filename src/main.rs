// This program is exported as a binary named `amzn-halllmic-rust-tui`.
//
// You can run it via Brazil:
//
// ```console
// $ brazil-build # needed once
// $ brazil-runtime-exec amzn-halllmic-rust-tui
// ```

// use amzn_halllmic_rust_tui::hello;

// fn main() {
//     println!("{}", hello("Halllmic-RustTui"));
// }
use std::{error::Error, io};

use tokio::runtime::Runtime;

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen},
    ui::ui,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture);

    // App Startup
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    
    app.get_table_names().await;
    let res = run_app(&mut terminal, &mut app);

    // Tear down
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}


fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {

    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    KeyCode::Char('z') => {
                        // Display a Cat!
                        app.show_cat();
                    }
                    KeyCode::Char('r') => {
                        tokio::runtime::Runtime::new().unwrap().block_on(app.get_table_names());
                    }
                    _ => {}
                }
                _ => {
                    // If Q is pressed Quit no matter the screen, just in case
                    match key.code {
                        KeyCode::Char('q') => {
                            return Ok(true);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}