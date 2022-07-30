use anyhow::{Context, Result};
use crossterm::{terminal, ExecutableCommand};
use tui::backend::{Backend, CrosstermBackend};

mod app;

pub fn main() -> Result<()> {
    terminal::enable_raw_mode()?;

    let mut backend = CrosstermBackend::new(std::io::stdout());
    backend
        .execute(terminal::EnterAlternateScreen)?
        .execute(terminal::Clear(terminal::ClearType::All))?
        .hide_cursor()?;
    let mut terminal = tui::Terminal::new(backend)?;

    app::App::new().run(&mut terminal)?;

    terminal::disable_raw_mode()?;
    terminal
        .backend_mut()
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(terminal::LeaveAlternateScreen)?
        .show_cursor()
        .context("reset terminal error")
}
