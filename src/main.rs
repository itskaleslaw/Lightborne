use animation::SpriteAnimationPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::{asset::AssetMetaCheck, diagnostic::LogDiagnosticsPlugin};
use bevy_rapier2d::prelude::*;

use camera::{CameraPlugin, HIGHRES_LAYER};
use config::ConfigPlugin;
use debug::DebugPlugin;
use input::{init_cursor_world_coords, update_cursor_world_coords};
use level::LevelManagementPlugin;
use light::LightManagementPlugin;
use lighting::DeferredLightingPlugin;
use particle::ParticlePlugin;
use player::PlayerManagementPlugin;
use shared::{AnimationState, GameState, ResetLevel, UiState};
use sound::SoundPlugin;
use ui::level_select::LevelSelectPlugin;
use ui::pause::PausePlugin;
use ui::settings::SettingsPlugin;
use ui::start_menu::StartMenuPlugin;

mod animation;
mod camera;
mod config;
mod debug;
mod input;
mod level;
mod light;
mod lighting;
mod particle;
mod player;
mod shared;
mod sound;
mod ui;
mod utils;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Lightborne".into(),
                        name: Some("lightborne".into()),
                        present_mode: PresentMode::AutoNoVsync,
                        canvas: Some("#bevy-container".into()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    //https://github.com/bevyengine/bevy_github_ci_template/issues/48
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::NONE))
        .insert_gizmo_config::<DefaultGizmoConfigGroup>(
            DefaultGizmoConfigGroup,
            GizmoConfig {
                enabled: true,
                render_layers: HIGHRES_LAYER,
                ..Default::default()
            },
        )
        .add_plugins(bevy_mod_debugdump::CommandLineArgs)
        .add_plugins(ConfigPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(8.0).in_fixed_schedule())
        .add_plugins(SpriteAnimationPlugin)
        .add_plugins(PlayerManagementPlugin)
        .add_plugins(LevelManagementPlugin)
        .add_plugins(LightManagementPlugin)
        .add_plugins(SoundPlugin)
        .add_plugins(ParticlePlugin)
        .add_plugins(PausePlugin)
        .add_plugins(StartMenuPlugin)
        .add_plugins(LevelSelectPlugin)
        .add_plugins(SettingsPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin::default())
        .insert_state(GameState::Ui)
        .add_sub_state::<UiState>()
        .add_sub_state::<AnimationState>()
        .insert_state(UiState::StartMenu)
        .add_plugins(DeferredLightingPlugin)
        .add_event::<ResetLevel>()
        .add_systems(Startup, init_cursor_world_coords)
        .add_systems(Update, update_cursor_world_coords)
        .run();
}
