/*

Main file for the tui
this is where the crossterm
and ratatui apps run

the only thing that happens here is
entering and exiting the alternate screen

*/

use std::io;

use crossterm::{
    event, 
    execute, 
    terminal::{
        EnterAlternateScreen, 
        LeaveAlternateScreen,
        disable_raw_mode,
        enable_raw_mode
    }
};
use ratatui::{
    Terminal, 
    backend::CrosstermBackend, 
    widgets::Paragraph, 
    Frame,
};

use color_eyre::Result;



fn main() -> Result<()> {

    // starting the tui
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // the app temporarily will be all in here
    // just to get something running

    let mut should_quit = false;

    // show hello world message 
    // and any key press will exit the tui
    while !should_quit {

        terminal.draw(|frame: &mut Frame| {
            frame.render_widget(Paragraph::new("Hello World!!!!"), frame.area());
        })?;

        let event = event::read()?;

        if event.is_key_press() {
            should_quit = true;
        }

    }


    // end clean up
    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    terminal.show_cursor()?;

    Ok(())
}
