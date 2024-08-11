use std::time::Duration;

use area_25_5::*;

use bevy::{
    prelude::*, sprite::Wireframe2dPlugin, time::common_conditions::on_timer,
    window::WindowResolution,
};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(
                        WINDOW_RESOLUTION.x_px,
                        WINDOW_RESOLUTION.y_px,
                    )
                    .with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            }),
        // INFO: this is used to generate meta files for the assets.
        // They are going to be generate at `imported_assets`.
        // Run with `cargo run --features bevy/asset_processor`
        // Just copy the contents and replace them on the assets/ folder.
        // Then, run the compilation to wasm.
        // .set(AssetPlugin {
        //     mode: AssetMode::Processed,
        //     ..default()
        // }),
        Wireframe2dPlugin,
    ));

    if cfg!(not(target_family = "wasm")) {
        // INFO: uncomment to inspect the world elements
        use bevy_inspector_egui::quick::WorldInspectorPlugin;

        app.register_type::<Weapon>()
            .register_type::<Ammo>()
            .register_type::<Item>()
            .register_type::<Buff>()
            .register_type::<BuffGroup>()
            .add_plugins(WorldInspectorPlugin::new());
    }

    app.insert_resource(Msaa::Off)
        // states
        .insert_state(GameState::Menu)
        // system sets
        .configure_sets(
            FixedUpdate,
            (
                CollisionSet.run_if(in_state(GameState::Alive)),
                MoveSet.run_if(in_state(GameState::Alive)),
                InputSet.run_if(in_state(GameState::Alive)),
            ),
        )
        // systems
        .add_systems(
            Startup,
            (
                setup_base_camera,
                setup_player_camera,
                setup_overlay_camera,
                setup_menu_camera,
                setup_resources,
                setup_sprite,
                setup_ui,
            )
                .chain(),
        )
        .add_systems(
            OnEnter(GameState::Alive),
            (
                cleanup_system::<MenuOverlay>,
                cleanup_system::<GameOverOverlay>,
                cleanup_system::<GameWonOverlay>,
                cleanup_system::<CleanupWhenPlayerDies>,
                reset_initial_state,
                setup_player,
            )
                .chain()
                .in_set(SetupSet),
        )
        .add_systems(
            FixedUpdate,
            (move_ammo, move_enemies_towards_player, animate_sprite).in_set(MoveSet),
        )
        .add_systems(
            FixedUpdate,
            (move_player, handle_click, handle_show_player_stats_ui).in_set(InputSet),
        )
        .add_systems(
            FixedUpdate,
            (
                check_for_ammo_collisions_with_enemy,
                check_for_player_collisions_to_enemy,
                check_for_item_collisions,
                check_for_weapon_collisions,
                check_for_offensive_buff_collisions_with_enemy,
            )
                .in_set(CollisionSet),
        )
        .add_systems(OnEnter(GameState::Menu), menu_screen)
        .add_systems(OnEnter(GameState::Dead), game_over_screen)
        .add_systems(OnEnter(GameState::Won), game_won_screen)
        .add_systems(
            FixedUpdate,
            (
                handle_start_game_click.run_if(in_state(GameState::Menu)),
                handle_restart_click.run_if(in_state(GameState::Dead)),
                handle_play_again_click.run_if(in_state(GameState::Won)),
            ),
        )
        .add_systems(
            FixedUpdate,
            tick_timer.run_if(on_timer(Duration::from_secs(1))),
        )
        .add_systems(
            FixedUpdate,
            remove_outdated_buffs.run_if(on_timer(Duration::from_secs(1))),
        )
        .add_systems(
            FixedUpdate,
            animate_player_buffs.run_if(on_timer(Duration::from_nanos(100))),
        )
        .observe(on_player_spawned)
        .observe(on_mouse_click)
        .observe(on_player_health_changed)
        .observe(on_player_speed_changed)
        .observe(on_player_armor_changed)
        .observe(on_enemy_health_changed)
        .observe(on_all_enemies_died)
        .observe(on_game_over)
        .observe(on_restart_click)
        .observe(on_score_changed)
        .observe(on_wave_changed)
        .observe(on_current_time_changed)
        .observe(on_buff_added)
        .observe(on_buff_add_ui)
        .observe(on_buff_remove_ui)
        .observe(on_weapon_found)
        .observe(on_player_profile_ui_set)
        .run();
}
