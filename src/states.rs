use bevy::prelude::*;

#[allow(unused)]
#[derive(Debug, Default, States, Hash, PartialEq, Eq, Clone, Copy)]
pub enum AppState {
    #[default]
    /// Loading the bare minimum to show the main menu, loading ui, etc.
    CoreLoading,
    /// In the main menu
    InMenu,
    /// Loading game data
    AppLoading,
    /// Game running
    AppRunning,
    /// Game shutting down
    AppEnding,
}

#[allow(unused)]
#[derive(Debug, Default, States, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    #[default]
    None,
    MainMenu,
    InGame,
    Paused,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>().init_state::<GameState>();
    }
}
