use std::str::FromStr;
use termimad::Area;

pub fn view_area() -> Area {
    let mut area = Area::full_screen();
    area.pad_for_max_width(120); // we don't want a too wide text column
    area
}

#[derive(Clone)]
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
