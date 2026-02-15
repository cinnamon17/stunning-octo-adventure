use std::io;
use stunning_octo_adventure::bus::models::App;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}

