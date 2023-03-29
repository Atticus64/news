use colored::*;
use std::io::stdout;
use std::io::Stdout;
use std::io::Write;
use termimad::crossterm::cursor::Hide;
use termimad::crossterm::cursor::Show;
use termimad::crossterm::event;
use termimad::crossterm::event::KeyCode::{Char, Down, Esc, PageDown, PageUp, Up};
use termimad::crossterm::event::{Event, KeyEvent};
use termimad::crossterm::terminal::ClearType;
use termimad::crossterm::terminal::{Clear, EnterAlternateScreen, LeaveAlternateScreen};
use termimad::crossterm::{queue, terminal};
use termimad::MadView;

use crate::utils::clipboard::copy_to_clipboard;

use super::skin::make_skin;
use super::view::view_area;

fn render_markdown(
    mut view: MadView,
    mut w: Stdout,
    link: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        view.write_on(&mut w)?;
        w.flush()?;
        let kill_render = listen_keymaps(&mut view, &mut w, link);
        if kill_render {
            break;
        }
    }
    terminal::disable_raw_mode()?;
    queue!(w, Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}

fn listen_keymaps(view: &mut MadView, w: &mut Stdout, link: &str) -> bool {
    let mut kill_render = false;

    match event::read() {
        Ok(Event::Key(KeyEvent { code, .. })) => match code {
            Up => view.try_scroll_lines(-1),
            Down => view.try_scroll_lines(1),
            PageUp => view.try_scroll_pages(-1),
            PageDown => view.try_scroll_pages(1),
            Char('k') => view.try_scroll_lines(-1),
            Char('j') => view.try_scroll_lines(1),
            Char('K') => view.try_scroll_pages(-1),
            Char('J') => view.try_scroll_pages(1),
            Char('q') => kill_render = true,
            Char('Q') => kill_render = true,
            Esc => kill_render = true,
            Char('y') => copy_to_clipboard(link),
            Char('Y') => copy_to_clipboard(link),
            _ => {}
        },
        Ok(Event::Resize(..)) => {
            queue!(w, Clear(ClearType::All)).expect("clear failed unexpecteds");
            view.resize(&view_area());
        }
        _ => {}
    }

    kill_render
}

pub fn generate_view(markdown: &str, link: &str) -> Result<(), Box<dyn std::error::Error>> {
    let skin = make_skin();

    let mut w = stdout(); // we could also have used stderr
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?; // hiding the cursor
    let notification = format!(
        "{}: k up, j down, K pageup, J pagedown {}: Esc and q, {}: y or Y",
        "Navigation".green(),
        "Exit".red(),
        "Copy url to clipboard".yellow()
    );
    let md = format!("{notification} \n {markdown}");
    //  let skin = termimad::get_default_skin();
    let view = MadView::from(md, view_area(), skin);
    render_markdown(view, w, link)?;
    Ok(())
}
