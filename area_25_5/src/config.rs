use bevy::prelude::*;
pub use bevy::render::view::RenderLayers;

pub(crate) const GAME_LAYER: RenderLayers = RenderLayers::layer(1);
pub(crate) const TILE_Z_INDEX: f32 = 0.;
pub(crate) const CHAR_Z_INDEX: f32 = 1.;

pub(crate) const PLAYER_MOVE_SPEED: f32 = 150.0;
pub(crate) const PLAYER_ARMOR: f32 = 100.0;
pub(crate) const PLAYER_HEALTH: f32 = 10000.;

pub(crate) const ENEMY_MOVE_SPEED: f32 = 100.0;
pub(crate) const ENEMY_HEALTH: f32 = 100.0;
pub(crate) const ENEMY_COLLISION_BOX_WIDTH: f32 = 19.;
pub(crate) const ENEMY_COLLISION_BOX_HEIGHT: f32 = 32.;
pub(crate) const ENEMY_RANDOM_SEED: u64 = 1987836746771;

pub(crate) const WEAPON_RANDOM_SEED: u64 = 1936836746771;
pub(crate) const AMMO_MOVE_SPEED: f32 = 100.0;
pub(crate) const AMMO_DAMAGE: f32 = 10.0;

pub(crate) const CAPSULE_LENGTH: f32 = 8.;
pub(crate) const CAPSULE_RADIUS: f32 = 4.;

pub(crate) const ITEM_SPEED_VALUE: f32 = 50.;
pub(crate) const ITEM_RANDOM_SEED: u64 = 1937836746771;

pub struct CustomWindowResolution {
    pub x_px: f32,
    pub y_px: f32,
}

pub const WINDOW_RESOLUTION: CustomWindowResolution = CustomWindowResolution {
    x_px: 1920.0,
    y_px: 1080.0,
};

pub const CAPSULE_COLLIDER: Vec2 =
    Vec2::new((CAPSULE_LENGTH + CAPSULE_RADIUS * 2.) / 2., CAPSULE_RADIUS);

pub const NUMBER_OF_WAVES: usize = 5;

// ##################### ENEMY #####################################
pub enum EnemyClassEnum {
    Orc,
}

pub struct EnemyType {
    pub damage: f32,
    pub health: f32,
    pub scale: Vec3,
    pub class: EnemyClassEnum,
}

const ENEMY_LVL_1: EnemyType = EnemyType {
    damage: 5.0,
    health: ENEMY_HEALTH,
    scale: Vec3::splat(1.0),
    class: EnemyClassEnum::Orc,
};
const ENEMY_LVL_2: EnemyType = EnemyType {
    damage: 10.0,
    health: ENEMY_HEALTH,
    scale: Vec3::new(1.2, 1.2, 1.0),
    class: EnemyClassEnum::Orc,
};
const ENEMY_LVL_3: EnemyType = EnemyType {
    damage: 15.0,
    health: ENEMY_HEALTH,
    scale: Vec3::new(1.4, 1.4, 1.0),
    class: EnemyClassEnum::Orc,
};
const ENEMY_LVL_4: EnemyType = EnemyType {
    damage: 20.0,
    health: ENEMY_HEALTH,
    scale: Vec3::new(1.6, 1.6, 1.0),
    class: EnemyClassEnum::Orc,
};
const ENEMY_LVL_5: EnemyType = EnemyType {
    damage: 25.0,
    health: ENEMY_HEALTH,
    scale: Vec3::new(1.8, 1.8, 1.0),
    class: EnemyClassEnum::Orc,
};

pub struct EnemyByLevel {
    pub level: usize,
    pub enemy: EnemyType,
    pub quantity: u32,
}
pub const ENEMIES_PER_WAVE: [EnemyByLevel; NUMBER_OF_WAVES] = [
    EnemyByLevel {
        level: 1,
        enemy: ENEMY_LVL_1,
        quantity: 5,
    },
    EnemyByLevel {
        level: 2,
        enemy: ENEMY_LVL_2,
        quantity: 10,
    },
    EnemyByLevel {
        level: 3,
        enemy: ENEMY_LVL_3,
        quantity: 15,
    },
    EnemyByLevel {
        level: 4,
        enemy: ENEMY_LVL_4,
        quantity: 20,
    },
    EnemyByLevel {
        level: 5,
        enemy: ENEMY_LVL_5,
        quantity: 25,
    },
];

// ##################### WEAPON #####################################
#[cfg_attr(not(web), derive(Reflect, Component, Default, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Default, Debug, Clone))]
pub enum WeaponTypeEnum {
    #[default]
    Bow,
    Wand,
}

pub struct WeaponType<'a> {
    pub damage: f32,
    pub source: &'a str,
    pub ammo_source: &'a str,
    pub weapon_type: WeaponTypeEnum,
}

const WEAPON_LVL_1: WeaponType = WeaponType {
    damage: 10.0,
    source: "textures/Weapon/Wand.png",
    ammo_source: "textures/Weapon/MagicBall.png",
    weapon_type: WeaponTypeEnum::Wand,
};

const WEAPON_LVL_2: WeaponType = WeaponType {
    damage: 20.0,
    source: "textures/Weapon/Bow.png",
    ammo_source: "textures/Weapon/Arrow.png",
    weapon_type: WeaponTypeEnum::Bow,
};

const WEAPON_LVL_3: WeaponType = WeaponType {
    damage: 30.0,
    source: "textures/Weapon/Bow.png",
    ammo_source: "textures/Weapon/Arrow.png",
    weapon_type: WeaponTypeEnum::Bow,
};

const WEAPON_LVL_4: WeaponType = WeaponType {
    damage: 40.0,
    source: "textures/Weapon/Bow.png",
    ammo_source: "textures/Weapon/Arrow.png",
    weapon_type: WeaponTypeEnum::Bow,
};

const WEAPON_LVL_5: WeaponType = WeaponType {
    damage: 50.0,
    source: "textures/Weapon/Bow.png",
    ammo_source: "textures/Weapon/Arrow.png",
    weapon_type: WeaponTypeEnum::Bow,
};

pub struct WeaponByLevel<'a> {
    pub level: usize,
    pub weapon: WeaponType<'a>,
    pub quantity: u32,
}

pub const WEAPONS_PER_WAVE: [WeaponByLevel; NUMBER_OF_WAVES] = [
    WeaponByLevel {
        level: 1,
        weapon: WEAPON_LVL_1,
        quantity: 1,
    },
    WeaponByLevel {
        level: 2,
        weapon: WEAPON_LVL_2,
        quantity: 1,
    },
    WeaponByLevel {
        level: 3,
        weapon: WEAPON_LVL_3,
        quantity: 1,
    },
    WeaponByLevel {
        level: 4,
        weapon: WEAPON_LVL_4,
        quantity: 1,
    },
    WeaponByLevel {
        level: 5,
        weapon: WEAPON_LVL_5,
        quantity: 1,
    },
];

// ##################### ITEMS #####################################
#[cfg_attr(not(web), derive(Reflect, Component, Default, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Default, Debug, Clone))]
pub enum ItemStatsType {
    #[default]
    Speed,
    Armor,
}

pub struct ItemType<'a> {
    pub value: f32,
    pub source: &'a str,
    pub item_type: ItemStatsType,
}

const ITEM_LVL_1: ItemType = ItemType {
    value: 10.0,
    source: "textures/Effects/speed_potion.png",
    item_type: ItemStatsType::Speed,
};

const ITEM_LVL_2: ItemType = ItemType {
    value: 20.0,
    source: "textures/Effects/speed_potion.png",
    item_type: ItemStatsType::Speed,
};

const ITEM_LVL_3: ItemType = ItemType {
    value: 30.0,
    source: "textures/Effects/speed_potion.png",
    item_type: ItemStatsType::Speed,
};

const ITEM_LVL_4: ItemType = ItemType {
    value: 40.0,
    source: "textures/Effects/speed_potion.png",
    item_type: ItemStatsType::Speed,
};

const ITEM_LVL_5: ItemType = ItemType {
    value: 50.0,
    source: "textures/Effects/speed_potion.png",
    item_type: ItemStatsType::Speed,
};

pub struct ItemByLevel<'a> {
    pub level: usize,
    pub item: ItemType<'a>,
    pub quantity: u32,
}

pub const ITEMS_PER_WAVE: [ItemByLevel; NUMBER_OF_WAVES] = [
    ItemByLevel {
        level: 1,
        item: ITEM_LVL_1,
        quantity: 1,
    },
    ItemByLevel {
        level: 2,
        item: ITEM_LVL_2,
        quantity: 2,
    },
    ItemByLevel {
        level: 3,
        item: ITEM_LVL_3,
        quantity: 2,
    },
    ItemByLevel {
        level: 4,
        item: ITEM_LVL_4,
        quantity: 1,
    },
    ItemByLevel {
        level: 5,
        item: ITEM_LVL_5,
        quantity: 3,
    },
];
