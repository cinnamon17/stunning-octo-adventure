use crate::bus::models::App;
use ratatui::{
    buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::{Style, Stylize}, symbols::border, text::Line, widgets::{Block, Paragraph, Widget}
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(Line::from(" TIEMPOS DE LLEGADA EN PARADAS CERCANAS BUSES ").bold().yellow())
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
}
