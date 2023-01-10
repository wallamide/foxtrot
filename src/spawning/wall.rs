use crate::spawning::{GameObject, PrimedGameObjectSpawner};
use bevy::ecs::system::EntityCommands;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const PATH: &str = "scenes/wallWood.glb";

pub fn load_scene(asset_server: &Res<AssetServer>) -> Handle<Gltf> {
    asset_server.load(PATH)
}

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn_wall(&'a mut self) -> EntityCommands<'w, 's, 'a> {
        let gltf = self
            .gltf
            .get(&self.handles.scenes[&GameObject::Wall])
            .unwrap_or_else(|| panic!("Failed to load scene from {PATH}"));
        let mut entity_commands = self.commands.spawn((
            SceneBundle {
                scene: gltf.scenes[0].clone(),
                transform: Transform::from_scale(Vec3::splat(3.)),
                ..default()
            },
            Name::new("Wall Wood"),
        ));
        entity_commands.with_children(|parent| {
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(-0.45, 0.5, 0.)),
                Collider::cuboid(0.04, 0.5, 0.5),
            ));
        });
        entity_commands
    }
}