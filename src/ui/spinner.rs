use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, Reflect)]
#[reflect(Component)]
pub struct Spinner;

#[derive(Debug, Default, Resource)]
pub struct SpinnerMaterialMesh {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct SpinnerMarker;

pub fn setup_spinners(
    mut commands: Commands,
    query: Query<Entity, Added<Spinner>>,
    core_assets: Res<crate::assets::CoreAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    spinner_material: Option<Res<SpinnerMaterialMesh>>,
) {
    // let (mesh, material) = if let Some(spinner_material) = spinner_material.as_ref() {
    //     (
    //         spinner_material.mesh.clone(),
    //         spinner_material.material.clone(),
    //     )
    // } else {
    //     let mesh = meshes.add(Rectangle::new(1.0, 1.0));
    //     let mat = StandardMaterial {
    //         base_color_texture: Some(core_assets.spinner_texture.clone()),
    //         alpha_mode: AlphaMode::Blend,
    //         ..Default::default()
    //     };
    //     let material = materials.add(mat);
    //     commands.insert_resource(SpinnerMaterialMesh {
    //         mesh: mesh.clone(),
    //         material: material.clone(),
    //     });
    //     (mesh.clone(), material.clone())
    // };

    for e in query.iter() {
        // let child = commands
        //     .spawn((
        //         MaterialMeshBundle {
        //             mesh: mesh.clone(),
        //             material: material.clone(),
        //             ..Default::default()
        //         },
        //         SpinnerMarker,
        //     ))
        //     .id();
        let child = commands
            .spawn((
                SpriteBundle {
                    texture: core_assets.spinner_texture.clone(),
                    ..Default::default()
                },
                SpinnerMarker,
            ))
            .id();
        let Some(mut parent) = commands.get_entity(e) else {
            continue;
        };

        parent.add_child(child);
    }
}

pub fn spin_spinners(mut spinners: Query<&mut Transform, With<SpinnerMarker>>, time: Res<Time>) {
    for mut transform in spinners.iter_mut() {
        transform.rotate_local_z(time.delta_seconds() * std::f32::consts::PI * -1.0);
    }
}
