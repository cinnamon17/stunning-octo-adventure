use std::collections::HashMap;
use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use crate::bus::fetch::fetch_times;
#[derive(Debug, Default)]
pub struct App {
    pub times: HashMap<String, String>,
    pub exit: bool,
}

impl App {

    fn update_bus_times(&mut self) {

        let lineas = vec![
            ("9", "191"),
            ("7", "166"),
            ("12", "132"),
        ];

        for (nombre, id) in lineas {
            let url = format!("https://www.salamancadetransportes.com/tiempos-de-llegada/?ref={}", id);
            if let Ok(tiempo) = fetch_times(&url, nombre) {
                self.times.insert(nombre.to_string(), tiempo + " ");
            }
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut last_update = std::time::Instant::now();
        let update_interval = std::time::Duration::from_secs(60);
        self.update_bus_times();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if event::poll(std::time::Duration::from_millis(100))? {
                self.handle_events()?;
            }

            if last_update.elapsed() >= update_interval {
                self.update_bus_times();
                last_update = std::time::Instant::now();
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                self.handle_key_event(key_event);
            }
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_handle_key_event_exit() {
        let mut app = App::default();

        let key = KeyEvent::new(
            KeyCode::Char('q'),
            event::KeyModifiers::empty(),
        );

        app.handle_key_event(key);

        assert!(app.exit, "La aplicación debería marcar exit como true al pulsar 'q'");
    }
}
