use core::fmt::{Display, Formatter};
use crate::game::cinematic_intro::make_cinematic_intro;
use crate::ui::simple_menu::SimpleMenu;

pub fn make_main_menu() -> SimpleMenu<MainMenuItem> {
    SimpleMenu::new(
        "Gothic",
        &[
            MainMenuItem::NewGame,
            MainMenuItem::Settings,
            MainMenuItem::Authors,
        ],
        |item, context| {
            // println!("[Main menu] Selected item: {}", item);
            match item {
                MainMenuItem::NewGame => {
                    // context.music.stop();
                    context.navigator.push_view(make_cinematic_intro());
                }
                MainMenuItem::Settings => {
                    // println!("Settings not implemented yet");
                }
                MainMenuItem::Authors => {
                    // println!("Authors not implemented yet");
                }
            }
        },
    )
}

#[derive(Clone, Eq, PartialEq)]
pub enum MainMenuItem {
    NewGame,
    Settings,
    Authors,
}

impl Display for MainMenuItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            MainMenuItem::NewGame => f.write_str("Nowa gra"),
            MainMenuItem::Settings => f.write_str("Ustawienia"),
            MainMenuItem::Authors => f.write_str("Autorzy"),
        }
    }
}