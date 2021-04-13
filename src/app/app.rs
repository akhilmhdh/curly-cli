use crate::app::{Navigation, RouteID};
use crate::ui::home_screen;
use ::tui::{backend::Backend, layout::Rect, Frame};

pub struct App {
    navigation: Navigation,
}

impl Default for App {
    fn default() -> Self {
        App {
            navigation: Navigation::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, size: Rect) {
        match self.navigation.get_current_route().id {
            RouteID::HomeScreen => home_screen::draw(f, size).unwrap(),
            _ => println!("Unknown route"),
        }
    }
}
