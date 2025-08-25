use animation::{flip_player_direction, set_animation, PlayerAnimationType};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use match_player::update_match_player_z;
use strand::PlayerStrandPlugin;

use crate::{animation::AnimationConfig, level::LevelSystems};

use kill::PlayerKillPlugin;
use light::{PlayerLightInventory, PlayerLightPlugin};
use movement::{PlayerMovement, PlayerMovementPlugin};
use spawn::{init_player_bundle, update_player_entity};

mod animation;
pub mod kill;
pub mod light;
pub mod match_player;
pub mod movement;
mod spawn;
mod strand;

/// [`Plugin`] for anything player based.
pub struct PlayerManagementPlugin;

impl Plugin for PlayerManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerLightPlugin)
            .add_plugins(PlayerMovementPlugin)
            .add_plugins(PlayerKillPlugin)
            .add_plugins(PlayerStrandPlugin)
            .add_systems(
                PreUpdate,
                update_player_entity.in_set(LevelSystems::Processing),
            )
            .add_systems(Update, update_match_player_z)
            .add_systems(
                FixedUpdate,
                (
                    flip_player_direction,
                    set_animation.in_set(LevelSystems::Simulation),
                ),
            );
    }
}

/// Add to player to prevent movement/other inputs
#[derive(Component)]
pub struct InputLocked;

pub fn not_input_locked(q_player: Query<Option<&InputLocked>, With<PlayerMarker>>) -> bool {
    !q_player
        .get_single()
        .is_ok_and(|input_locked| input_locked.is_some())
}

/// [`Component`] to signal our own code to finish the initialization of the player (adding sensors, etc)
#[derive(Component, Default)]
pub struct PlayerMarker;

/// Attached to player hurtbox
#[derive(Default, Component)]
pub struct PlayerHurtMarker;

/// [`Bundle`] that will be initialized with [`init_player_bundle`] and inserted to the player
/// [`Entity`] by Ldtk.
#[derive(Bundle)]
pub struct PlayerBundle {
    body: RigidBody,
    controller: KinematicCharacterController,
    controller_output: KinematicCharacterControllerOutput,
    collider: Collider,
    collision_groups: CollisionGroups,
    friction: Friction,
    restitution: Restitution,
    player_movement: PlayerMovement,
    light_inventory: PlayerLightInventory,
    // point_lighting: LineLight2d,
    animation_config: AnimationConfig,
    animation_type: PlayerAnimationType,
}

/// [`Bundle`] registered with Ldtk that will be spawned in with the level.
#[derive(Bundle, LdtkEntity)]
pub struct LdtkPlayerBundle {
    #[default]
    player_marker: PlayerMarker,
    #[with(init_player_bundle)]
    player: PlayerBundle,
    #[worldly]
    worldly: Worldly,
    #[from_entity_instance]
    instance: EntityInstance,
}
