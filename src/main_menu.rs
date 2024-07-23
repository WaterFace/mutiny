use bevy::prelude::*;
use blenvy::{BlueprintInfo, HideUntilReady, SpawnBlueprint};

use crate::states::{AppState, GameState};

fn setup_main_menu(mut commands: Commands) {
    info!("spawning main menu scene");

    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 2,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((
        BlueprintInfo::from_path("levels/MainMenu.gltf"),
        SpawnBlueprint,
        HideUntilReady,
        TransformBundle::from_transform(Transform::default()),
        StateScoped(GameState::MainMenu),
    ));
}

fn go_to_main_menu(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::MainMenu);
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.enable_state_scoped_entities::<GameState>()
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(AppState::CoreLoading), go_to_main_menu);
    }
}
