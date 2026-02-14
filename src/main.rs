use std::{fs::OpenOptions, io};
use std::collections::HashMap; 
use std::io::Write;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::style::Style;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}

fn log_debug(mensaje: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("debug.log")
        .unwrap();
    writeln!(file, "{}", mensaje).unwrap();
}

#[derive(Debug, Default)]
pub struct App {
    times: HashMap<String, String>,
    exit: bool,
}

impl App {
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
    fn update_bus_times(&mut self) {

        let lineas = vec![
            ("9", "191"),
            ("7", "151"),
            ("12", "132"),
        ];

        for (nombre, id) in lineas {
            let url = format!("https://www.salamancadetransportes.com/tiempos-de-llegada/?ref={}", id);
            if let Ok(tiempo) = self.fetch_times(&url) {
                self.times.insert(nombre.to_string(), tiempo + " ");
            }
        }
    }

    fn fetch_times(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::builder()
            .user_agent("Mozilla/5.0...")
            .build()?;

        let response = client.get(url).send()?.text()?;
        let doc = dom_query::Document::from(&response);

        let tiempos: Vec<String> = doc.select(".arrival_times_results_row span.right")
            .iter()
            .map(|element| element.text().trim().to_string())
            .filter(|t| !t.is_empty())
            .collect();

        if tiempos.is_empty() {
            Ok("Sin datos".to_string())
        } else {
            Ok(tiempos.iter().take(2).cloned().collect::<Vec<_>>().join(" | "))
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(Line::from(" BUSES SALAMANCA ").bold().yellow())
            .border_set(border::THICK);

        let inner_area = block.inner(area);
        block.render(area, buf);

        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1) 
            .constraints([
                Constraint::Length(2), 
                Constraint::Length(2), 
                Constraint::Length(2),
            ])
            .split(inner_area);

        let lineas = [("9", 0), ("7", 1), ("12", 2)];
        for (num, idx) in lineas {
            let tiempo = self.times.get(num).cloned().unwrap_or_else(|| "...".to_string());
            render_linea_bus(&format!("Línea {}:", num), &tiempo, vertical_chunks[idx], buf);
        }
    }
}

fn render_linea_bus(etiqueta: &str, tiempo: &str, area: Rect, buf: &mut Buffer) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(12), 
            Constraint::Min(10),   
        ])
        .split(area);

    Paragraph::new(etiqueta.bold().cyan()).render(chunks[0], buf);
    
    let estilo_tiempo = if tiempo.contains("LLEGANDO") {
        Style::default().fg(ratatui::style::Color::Red).bold()
    } else {
        Style::default().fg(ratatui::style::Color::Yellow)
    };

    Paragraph::new(tiempo).style(estilo_tiempo).render(chunks[1], buf);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::{Style, Stylize};
    use std::collections::HashMap;

#[test]
fn test_render_bus_ui() {
    let mut tiempos_mock = HashMap::new();
    tiempos_mock.insert("9".to_string(), "5 min".to_string());
    tiempos_mock.insert("7".to_string(), "12 min".to_string());
    tiempos_mock.insert("12".to_string(), "10 min".to_string());

    let app = App {
    times: tiempos_mock,
        exit: false,
    };

    let area = Rect::new(0, 0, 60, 10);
    let mut buf = Buffer::empty(area);

    Widget::render(&app, area, &mut buf);
    let contenido = buf.content
        .iter()
        .map(|c| c.symbol().to_string())
        .collect::<String>();

    assert!(contenido.contains("Línea 9"), "No se encontró 'Línea 9' en el buffer");
    assert!(contenido.contains("5 min"), "No se encontró el tiempo '5 min'");
    assert!(contenido.contains("Línea 7"), "No se encontró 'Línea 7'");
    assert!(contenido.contains("12 min"), "No se encontró el tiempo '12 min'");
}

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
