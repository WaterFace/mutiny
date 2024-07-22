use bevy::{math::VectorSpace, prelude::*};
use bevy_gltf_blueprints::{BlueprintName, SpawnHere};

use crate::{
    assets::CoreAssets,
    states::{AppState, GameState},
};

fn setup_main_menu(mut commands: Commands, core_assets: Res<CoreAssets>, gltfs: Res<Assets<Gltf>>) {
    let Some(ref main_menu_scene) = core_assets.main_menu_scene else {
        error!("No main_menu_scene defined in core assets!");
        panic!();
    };

    let Some(main_menu_scene) = gltfs.get(main_menu_scene) else {
        error!("main_menu_scene not loaded");
        panic!();
    };

    commands.insert_resource(AmbientLight {
        color: bevy::color::palettes::basic::WHITE.into(),
        brightness: 0.2,
        ..Default::default()
    });

    info!("spawning main menu scene");
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1,
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
            ..Default::default()
        },
        StateScoped(GameState::MainMenu),
    ));
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "MUTINY",
            TextStyle {
                font: core_assets.font.clone().unwrap(),
                font_size: 72.0,
                ..Default::default()
            },
        ),
        ..Default::default()
    });

    commands.spawn((
        BlueprintName("Loading Spinner".to_string()),
        SpawnHere,
        TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
    ));

    commands.spawn((
        SceneBundle {
            scene: main_menu_scene.scenes[0].clone(),
            ..Default::default()
        },
        StateScoped(GameState::MainMenu),
    ));
}

fn go_to_main_menu(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::MainMenu);
}

fn test(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<&BlueprintName, Added<BlueprintName>>,
    spinners: Query<Entity, With<crate::ui::Spinner>>,
) {
    for bp in query.iter() {
        info!("trying to spawn {}", bp.0);
    }
    if input.just_pressed(KeyCode::Space) {
        debug!("{}", spinners.iter().count());
        commands.spawn((
            BlueprintName("Loading Spinner".to_string()),
            SpawnHere,
            TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
        ));
    }
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.enable_state_scoped_entities::<GameState>()
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(AppState::CoreLoading), go_to_main_menu)
            .add_systems(Update, test.run_if(in_state(GameState::MainMenu)));
    }
}
