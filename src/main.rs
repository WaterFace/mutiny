use bevy::prelude::*;
use bevy_gltf_blueprints::{BlueprintsPlugin, GltfFormat};
use bevy_registry_export::ExportRegistryPlugin;

#[derive(Debug, Default, States, Hash, PartialEq, Eq, Clone, Copy)]
pub enum AppState {
    AppLoading,
    #[default]
    AppRunning,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ExportRegistryPlugin::default())
        .add_plugins(BlueprintsPlugin {
            legacy_mode: false,
            library_folder: "models/library".into(),
            format: GltfFormat::GLB,
            aabbs: true,
            ..Default::default()
        })
        .init_state::<AppState>()
        .run();
}
