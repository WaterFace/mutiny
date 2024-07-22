use bevy::prelude::*;
use blenvy::BlenvyPlugin;

mod assets;
mod main_menu;
mod states;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BlenvyPlugin::default())
        .add_plugins(states::StatesPlugin)
        .add_plugins(assets::AssetsPlugin::default())
        .add_plugins(ui::UiPlugin)
        .add_plugins(main_menu::MainMenuPlugin)
        .run();
}
