mod groq;

use std::{io, time::Duration};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Layout, Direction, Constraint},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    dotenv().ok();
    let api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY not set");

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();
    let mut messages: Vec<String> = vec![];

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1),
                    Constraint::Length(3),
                ])
                .split(f.size());

            let chat = Paragraph::new(messages.join("\n\n"))
                .block(Block::default().borders(Borders::ALL).title("Groq AI Chat"));

            let input_box = Paragraph::new(input.as_str())
                .block(Block::default().borders(Borders::ALL).title("You"));

            f.render_widget(chat, chunks[0]);
            f.render_widget(input_box, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Backspace => { input.pop(); }
                    KeyCode::Enter => {
                        let user_msg = input.clone();
                        messages.push(format!("🧑 You: {}", user_msg));

                        input.clear();

                        let reply = match groq::ask_groq(&user_msg, &api_key).await {
    Ok(r) => r,
    Err(e) => format!("Groq error: {}", e),
};

                        messages.push(format!("🤖 AI: {}", reply));
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

