use bevy::app::{App, Plugin};
use bevy::asset::{AssetServer, Handle};
use bevy::prelude::{Res, ResMut, Resource, Startup};
use bevy::scene::Scene;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missiles: Handle<Scene>,
}

pub struct AssetsLoaderPlugin;

impl Plugin for AssetsLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>().add_systems(Startup, load_assets);
    }
}

fn load_assets(
    mut scene_assets: ResMut<SceneAssets>,
    asset_server: Res<AssetServer>,
) {
    *scene_assets = SceneAssets {
        asteroid: asset_server.load("Asteroid.glb#Scene0"),
        spaceship: asset_server.load("Spaceship.glb#Scene0"),
        missiles: asset_server.load("Missile.glb#Scene0")
    }
}