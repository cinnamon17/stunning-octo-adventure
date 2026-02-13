use std::{fs::OpenOptions, io};
use std::io::Write;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
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
    linea_nueve: String,
    linea_siete: String,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if event::poll(std::time::Duration::from_millis(5000))? {
                self.handle_events()?;
            }
            self.update_bus_times();
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
        let url = "https://www.salamancadetransportes.com/tiempos-de-llegada/?ref=110";
        
        if let Ok(response) = reqwest::blocking::get(url) {
            if let Ok(html) = response.text() {
                let doc = dom_query::Document::from(&html);
                let texto_fila = doc.select(".arrival_times_results_row  span.right");
                self.linea_nueve = texto_fila.text().to_string();
            }
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Horarios Buses ".bold());
        let instructions = Line::from(vec![" Salir ".into(), "(Q) ".blue().bold()]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let inner_area = block.inner(area);
        block.render(area, buf);

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), 
                Constraint::Length(1),
            ])
            .split(inner_area);

        render_linea_bus("Linea 9: ", &self.linea_nueve, rows[0], buf);
        render_linea_bus("Linea 7: ", &self.linea_siete, rows[1], buf);
    }
}

fn render_linea_bus(etiqueta: &str, tiempo: &str, area: Rect, buf: &mut Buffer) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    Paragraph::new(etiqueta).left_aligned().render(chunks[0], buf);

    let tiempo_texto = if tiempo.is_empty() { "0" } else { tiempo };
    let tiempo_fmt = format!("{} min", tiempo_texto).yellow();
    Paragraph::new(tiempo_fmt).right_aligned().render(chunks[1], buf);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn test_render_bus_ui() {
        let app = App {
            linea_nueve: "5".to_string(),
            linea_siete: "12".to_string(),
            exit: false,
        };

        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 6));
        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━━━━ Horarios Buses ━━━━━━━━━━━━━━━━┓",
            "┃Linea 9:                                   5 min┃",
            "┃Linea 7:                                  12 min┃",
            "┃                                                ┃",
            "┃                                                ┃",
            "┗━━━━━━━━━━━━━━━━━━ Salir (Q) ━━━━━━━━━━━━━━━━━━━┛",
        ]);

        let title_style = Style::new().bold();
        let key_style = Style::new().blue().bold();
        let time_style = Style::new().yellow();

        expected.set_style(Rect::new(17, 0, 16, 1), title_style);

        expected.set_style(Rect::new(44, 1, 5, 1), time_style);
        expected.set_style(Rect::new(43, 2, 6, 1), time_style);

        expected.set_style(Rect::new(26, 5, 4, 1), key_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn test_handle_key_event_exit() {
        let mut app = App::default();
        let key = KeyEvent::new(
            KeyCode::Char('q'), 
            event::KeyModifiers::empty() 
        );
        app.handle_key_event(key);
        assert!(app.exit, "La aplicación debería marcar exit como true al pulsar 'q'");
    }
}
