use bevy::prelude::*;

#[derive(Debug, Default, Component, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct Panel;

pub fn setup_panels(mut commands: Commands, query: Query<Entity, Added<Panel>>) {}
