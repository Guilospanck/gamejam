use super::*;

pub(crate) const POWER_RANDOM_SEED: u64 = 1242436746771;
pub(crate) const POWER_SPRITE_SIZE: u8 = 32;

#[cfg_attr(not(web), derive(Reflect, Component, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Debug, Clone))]
pub enum PowerTypeEnum {
    Explosions,
    CircleOfDeath,
}

#[cfg_attr(not(web), derive(Reflect, Component, Default, Debug, Clone))]
#[cfg_attr(not(web), reflect(Component))]
#[cfg_attr(web, derive(Component, Default, Debug, Clone))]
pub enum StoppingCondition {
    #[default]
    Instances,
    Limit,
    // ScreenBounces,
}

pub struct PowerType {
    pub damage: f32,
    pub mana_needed: f32,
    pub power_type: PowerTypeEnum,
    pub stopping_condition: StoppingCondition,
    pub max_value: u32,
}

const POWER_LVL_1: PowerType = PowerType {
    damage: 10.0,
    mana_needed: 10.0,
    power_type: PowerTypeEnum::CircleOfDeath,
    stopping_condition: StoppingCondition::Limit,
    max_value: 0,
};

const POWER_LVL_2: PowerType = PowerType {
    damage: 10.0,
    mana_needed: 10.0,
    power_type: PowerTypeEnum::Explosions,
    stopping_condition: StoppingCondition::Instances,
    max_value: 5,
};

pub struct PowerByLevel {
    pub level: usize,
    pub power: PowerType,
    pub quantity: u32,
}

pub const POWERS_PER_WAVE: [PowerByLevel; NUMBER_OF_WAVES] = [
    PowerByLevel {
        level: 2,
        power: POWER_LVL_1,
        quantity: 1,
    },
    PowerByLevel {
        level: 3,
        power: POWER_LVL_1,
        quantity: 1,
    },
    PowerByLevel {
        level: 4,
        power: POWER_LVL_1,
        quantity: 1,
    },
    PowerByLevel {
        level: 5,
        power: POWER_LVL_1,
        quantity: 1,
    },
    // TODO: remove as the powers are only spawned *after* the wave is done
    PowerByLevel {
        level: 6,
        power: POWER_LVL_1,
        quantity: 1,
    },
];
