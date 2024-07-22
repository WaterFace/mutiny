use bevy::{
    pbr::wireframe::WireframePlugin,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dPlugin},
};

use crate::states::AppState;

/// Registers types and systems for UI tag components
#[derive(Debug, Default)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Spinner>()
            .add_systems(
                Update,
                (handle_spinners, spin_spinners).run_if(not(in_state(AppState::CoreLoading))),
            )
            .add_plugins(Wireframe2dPlugin)
            .add_plugins(WireframePlugin);
    }
}

#[derive(Component, Clone, Copy, Debug, Reflect)]
#[reflect(Component)]
pub struct Spinner;

#[derive(Debug, Default, Resource)]
struct SpinnerMaterialMesh {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
}

#[derive(Component, Clone, Copy, Debug)]
struct SpinnerMarker;

fn handle_spinners(
    mut commands: Commands,
    query: Query<Entity, Added<Spinner>>,
    core_assets: Res<crate::assets::CoreAssets>,
    mut color_mats: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    spinner_material: Option<Res<SpinnerMaterialMesh>>,
) {
    let (mesh, material) = if let Some(spinner_material) = spinner_material.as_ref() {
        (
            spinner_material.mesh.clone(),
            spinner_material.material.clone(),
        )
    } else {
        let mesh = meshes.add(Rectangle::new(1.0, 1.0));
        let mat = ColorMaterial {
            texture: Some(core_assets.spinner_texture.clone()),
            ..Default::default()
        };
        let material = color_mats.add(mat);
        commands.insert_resource(SpinnerMaterialMesh {
            mesh: Mesh2dHandle(mesh.clone()),
            material: material.clone(),
        });
        (Mesh2dHandle(mesh.clone()), material.clone())
    };

    for e in query.iter() {
        let child = commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: mesh.clone(),
                    material: material.clone(),
                    ..Default::default()
                },
                SpinnerMarker,
            ))
            .id();
        let Some(mut parent) = commands.get_entity(e) else {
            continue;
        };
        info!("created spinner");
        parent.add_child(child);
    }
}

fn spin_spinners(mut spinners: Query<&mut Transform, With<SpinnerMarker>>, time: Res<Time>) {
    for mut transform in spinners.iter_mut() {
        transform.rotate_local_z(time.delta_seconds() * std::f32::consts::PI * 2.0);
    }
}
