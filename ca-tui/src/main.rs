#![feature(iter_intersperse)]
#![feature(if_let_guard)]

mod app;
pub mod messages;
pub mod tab;
pub mod widgets;

use app::App;
use crossterm::event::{self, KeyEvent, KeyEventKind};

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear().unwrap();

    let mut app = App::new();

    loop {
        terminal.draw(|frame| app.draw(frame))?;
        // if let Ok(event::Event::Key(key_ev)) = event::read()
        //     && let KeyEventKind::Press = key_ev.kind
        if let Ok(event::Event::Key(
            key_ev @ KeyEvent {
                kind: KeyEventKind::Press,
                ..
            },
        )) = event::read()
        {
            if app.handle_key_press(key_ev) {
                break;
            }
        }
    }

    Ok(())
}
