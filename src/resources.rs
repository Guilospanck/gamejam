use std::time::Duration;

use crate::{
    prelude::*, AnimationIndices, AnimationInfo, AnimationTimer, RectangularDimensions, SpriteInfo,
    Sprites,
};

#[derive(Resource)]
pub struct PlayerHitAudioTimeout(pub Timer);

#[derive(Resource)]
pub struct CurrentGameLevel(pub u16);

#[derive(Resource)]
pub struct CurrentBoss(pub Option<u16>);

#[derive(Resource)]
pub struct CurrentWave(pub u16);

#[derive(Resource)]
pub struct CurrentScore(pub f32);

#[derive(Resource, Clone)]
pub struct CurrentTime {
    pub minutes: u16,
    pub seconds: u16,
}

#[derive(Resource)]
pub struct AutoShootingEnabled(pub bool);

#[derive(Resource)]
pub struct EnemyWaves(pub [EnemyByWave; NUMBER_OF_WAVES]);

#[derive(Resource)]
pub struct WeaponWaves(pub [WeaponByWave<'static>; NUMBER_OF_WAVES]);

#[derive(Resource)]
pub struct ItemWaves(pub [ItemByWave<'static>; NUMBER_OF_WAVES]);

#[derive(Resource)]
pub struct PowerLevels(pub [PowerByLevel; NUMBER_OF_POWERS]);

#[derive(Resource)]
pub struct SpritesResources(pub Sprites<'static>);

#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    #[default]
    Menu,
    Alive,
    Dead,
    Won,
    InBetweenLevels,
    Start,
}

#[derive(Resource)]
pub struct WindowResolutionResource {
    pub x_px: f32,
    pub y_px: f32,
}

#[derive(Resource)]
pub struct MouseDirectionWhenAutoShooting {
    pub x_px: f32,
    pub y_px: f32,
}

pub fn setup_resources(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.single();

    commands.insert_resource(CurrentWave(1));
    commands.insert_resource(CurrentBoss(None));
    commands.insert_resource(CurrentGameLevel(1));
    commands.insert_resource(EnemyWaves(ENEMIES_PER_WAVE));
    commands.insert_resource(WeaponWaves(WEAPONS_PER_WAVE));
    commands.insert_resource(ItemWaves(ITEMS_PER_WAVE));
    commands.insert_resource(PowerLevels(POWERS_PER_LEVEL));
    commands.insert_resource(SpritesResources(get_sprites()));
    commands.insert_resource(CurrentScore(0.));
    commands.insert_resource(AutoShootingEnabled(false));
    commands.insert_resource(MouseDirectionWhenAutoShooting {
        x_px: 0.0,
        y_px: 0.0,
    });
    commands.insert_resource(CurrentTime {
        minutes: 0,
        seconds: 30,
    });
    commands.insert_resource(PlayerHitAudioTimeout(Timer::new(
        Duration::from_secs(3),
        TimerMode::Repeating,
    )));
    commands.insert_resource(WindowResolutionResource {
        x_px: window.resolution.width(),
        y_px: window.resolution.height(),
    });
}

pub fn get_sprites() -> Sprites<'static> {
    const PLAYER_PIXEL_SIZE: u32 = 32;
    const PLAYER_ANIMATION_TIMER: f32 = 0.1;

    Sprites {
        level_1_bg: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 1920,
                height: 1080,
            },
            source: "textures/Background/level_1.png",
            animation: None,
            layout: TextureAtlasLayout::from_grid(UVec2::new(1920, 1080), 1, 1, None, None),
        },
        level_2_bg: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 1920,
                height: 1080,
            },
            source: "textures/Background/level_2.png",
            animation: None,
            layout: TextureAtlasLayout::from_grid(UVec2::new(1920, 1080), 1, 1, None, None),
        },
        level_3_bg: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 1920,
                height: 1080,
            },
            source: "textures/Background/level_3.png",
            animation: None,
            layout: TextureAtlasLayout::from_grid(UVec2::new(1920, 1080), 1, 1, None, None),
        },
        player_char_idle: SpriteInfo {
            dimensions: RectangularDimensions {
                width: PLAYER_PIXEL_SIZE,
                height: PLAYER_PIXEL_SIZE,
            },
            source: "textures/Alien/Alien_idle.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 3 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(
                UVec2::new(PLAYER_PIXEL_SIZE, PLAYER_PIXEL_SIZE),
                4,
                1,
                None,
                None,
            ),
        },
        player_char_walking: SpriteInfo {
            dimensions: RectangularDimensions {
                width: PLAYER_PIXEL_SIZE,
                height: PLAYER_PIXEL_SIZE,
            },
            source: "textures/Alien/Alien_run.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 5 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(
                UVec2::new(PLAYER_PIXEL_SIZE, PLAYER_PIXEL_SIZE),
                6,
                1,
                None,
                None,
            ),
        },
        orc_idle: SpriteInfo {
            dimensions: RectangularDimensions {
                width: PLAYER_PIXEL_SIZE,
                height: PLAYER_PIXEL_SIZE,
            },
            source: "textures/Enemy/Idle-Sheet.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 3 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(
                UVec2::new(PLAYER_PIXEL_SIZE, PLAYER_PIXEL_SIZE),
                4,
                1,
                None,
                None,
            ),
        },
        mage_idle: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 24,
                height: 24,
            },
            source: "textures/Astronaut/Astronaut_Idle.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 5 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(24, 24), 6, 1, None, None),
        },
        bow: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Weapon/Bow.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        arrow: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Weapon/Arrow.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        wand: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Weapon/Wand.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        magic_ball: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Weapon/MagicBall.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        mana_potion: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Effects/mana_potion.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        lightning: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Items/lightning.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        shield: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Items/shield.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        profile: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/UI/profile.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        diamond: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Items/Diamond.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        hp_pack: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Items/hp_pack.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        circle_of_death: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Powers/circle_of_death.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        laser: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Powers/laser.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        mine_bomb: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Powers/mine_bomb.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
        invisibility: SpriteInfo {
            dimensions: RectangularDimensions {
                width: 32,
                height: 32,
            },
            source: "textures/Items/invisibility.png",
            animation: Some(AnimationInfo {
                indices: AnimationIndices { first: 0, last: 0 },
                timer: AnimationTimer(Timer::from_seconds(
                    PLAYER_ANIMATION_TIMER,
                    TimerMode::Repeating,
                )),
            }),
            layout: TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None),
        },
    }
}
