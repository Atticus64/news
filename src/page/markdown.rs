use crate::scrape::link::NewsLink;
use colored::*;
use html2text::from_read;
use inquire::Select;
use std::io::Stdout;
use std::io::{stdout, Write};
use std::str::FromStr;
use termimad::crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode::*, KeyEvent},
    queue,
    style::Color::*,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use termimad::*;

#[derive(Clone, Debug)]
pub enum View {
    Web,
    Terminal,
}

impl FromStr for View {
    type Err = ();

    fn from_str(input: &str) -> Result<View, Self::Err> {
        match input {
            "Web" => Ok(View::Web),
            "Terminal" => Ok(View::Terminal),
            _ => Err(()),
        }
    }
}

fn view_area() -> Area {
    let mut area = Area::full_screen();
    area.pad_for_max_width(120); // we don't want a too wide text column
    area
}

pub fn get_markdonwwn_content(html: &str) -> String {
    let bytes = html.as_bytes();

    from_read(bytes, 80)
}

fn render_markdown(mut view: MadView, mut w: Stdout) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        view.write_on(&mut w)?;
        w.flush()?;
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
                Char('q') => break,
                Char('Q') => break,
                Esc => break,
                _ => {}
            },
            Ok(Event::Resize(..)) => {
                queue!(w, Clear(ClearType::All))?;
                view.resize(&view_area());
            }
            _ => {}
        }
    }
    terminal::disable_raw_mode()?;
    queue!(w, Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}

pub fn generate_view(markdown: &str) -> Result<(), Box<dyn std::error::Error>> {
    let skin = make_skin();

    let mut w = stdout(); // we could also have used stderr
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?; // hiding the cursor
    let notification = format!(
        "{}: k up, j down, K pageup, J pagedown",
        "Navigation".green()
    );
    let md = format!("{notification} \n {markdown}");
    // write!(&mut w, " {}: q or Esc", "Quit".red()).unwrap();
    //  let skin = termimad::get_default_skin();
    let view = MadView::from(md.to_string(), view_area(), skin);
    render_markdown(view, w)?;
    Ok(())
}

pub async fn show_news(new: &NewsLink) -> Result<(), Box<dyn std::error::Error>> {
    let link = &new.link;
    let response = reqwest::get(link).await?;
    let url = response.url();

    // if is a video of youtube
    if url
        .domain()
        .expect("error not is a domain")
        .contains("youtube")
    {
        if webbrowser::open(new.link.as_str()).is_ok() {}
        return Ok(());
    }

    let view_select = Select::new("What view do you like to do?", vec!["Web", "Terminal"])
        .with_help_message("Enter the view of the new")
        .prompt()
        .expect("error reading prompt view select");

    let view = View::from_str(view_select).expect("failed to parse view");
    match view {
        View::Terminal => {
            println!("{link}");

            let html = response.text().await?;
            let markdown = get_markdonwwn_content(&html);
            if markdown.is_empty() {
                println!("Content of new cannot be loaded in terminal");
                println!("Opening browser instead");
                if webbrowser::open(new.link.as_str()).is_ok() {}
            } else {
                generate_view(markdown.as_str()).expect("failed to generate a markdown view");
            }
        }
        View::Web => if webbrowser::open(new.link.as_str()).is_ok() {},
    }

    Ok(())
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.table.align = Alignment::Center;
    skin.set_headers_fg(AnsiValue(178));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = Alignment::Center;
    skin
}
