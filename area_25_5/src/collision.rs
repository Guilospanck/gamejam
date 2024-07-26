use crate::{
    enemy::Enemy,
    events::{AlienHealthChanged, AlienSpeedChanged},
    item::{Item, ItemStatsType},
    player::Alien,
    prelude::*,
    weapon::Ammo,
    AllEnemiesDied, Weapon,
};

pub fn check_for_ammo_collisions(
    mut commands: Commands,
    ammos: Query<(Entity, &Transform, &Ammo), (With<Ammo>, Without<Alien>)>,
    mut enemies: Query<(Entity, &Transform, &mut Enemy), With<Enemy>>,
) {
    let number_of_enemies = enemies.iter().len();
    if number_of_enemies == 0 {
        commands.trigger(AllEnemiesDied);
        return;
    }

    for (enemy_entity, enemy_transform, mut enemy) in enemies.iter_mut() {
        let enemy_collider = Aabb2d::new(
            enemy_transform.translation.truncate(),
            Vec2::new(
                ENEMY_COLLISION_BOX_WIDTH / 2.,
                ENEMY_COLLISION_BOX_HEIGHT / 2.,
            ),
        );

        for (ammo_entity, ammo_transform, ammo) in ammos.iter() {
            let ammo_collider =
                Aabb2d::new(ammo_transform.translation.truncate(), CAPSULE_COLLIDER);

            if ammo_collider.intersects(&enemy_collider) {
                damage_enemy(
                    &mut commands,
                    ammo_entity,
                    enemy_entity,
                    &mut enemy,
                    ammo.damage,
                );
                continue;
            }
        }
    }
}

pub fn check_for_alien_collisions_to_enemy(
    mut commands: Commands,
    mut enemies: Query<(&Transform, &mut Enemy), With<Enemy>>,
    mut alien: Query<(Entity, &Transform, &mut Alien)>,
) {
    for (enemy_transform, enemy) in enemies.iter_mut() {
        let enemy_collider = Aabb2d::new(
            enemy_transform.translation.truncate(),
            Vec2::new(
                ENEMY_COLLISION_BOX_WIDTH / 2.,
                ENEMY_COLLISION_BOX_HEIGHT / 2.,
            ),
        );

        if let Ok(result) = alien.get_single_mut() {
            let (alien_entity, alien_transform, mut alien) = result;
            let alien_collider =
                Aabb2d::new(alien_transform.translation.truncate(), CAPSULE_COLLIDER);

            if alien_collider.intersects(&enemy_collider) {
                damage_alien(&mut commands, alien_entity, &mut alien, enemy.damage);
            }
        }
    }
}

pub fn check_for_item_collisions(
    mut commands: Commands,
    mut alien: Query<(&Transform, &mut Alien)>,
    items: Query<(Entity, &Transform, &Item)>,
) {
    for (item_entity, item_transform, item) in items.iter() {
        let item_collider =
            Aabb2d::new(item_transform.translation.truncate(), CAPSULE_COLLIDER + 5.);

        if let Ok(result) = alien.get_single_mut() {
            let (alien_transform, mut alien) = result;
            let alien_collider =
                Aabb2d::new(alien_transform.translation.truncate(), CAPSULE_COLLIDER);

            if alien_collider.intersects(&item_collider) {
                alien.speed += item.value;
                match item.stats {
                    ItemStatsType::Speed => {
                        commands.trigger(AlienSpeedChanged { speed: alien.speed });
                    }
                    ItemStatsType::Armor => todo!(),
                }
                commands.entity(item_entity).despawn();
            }
        }
    }
}

pub fn check_for_weapon_collisions(
    mut commands: Commands,
    mut alien: Query<(&Transform, &mut Alien)>,
    weapons: Query<(Entity, &Transform, &Weapon)>,
) {
    for (weapon_entity, weapon_transform, weapon) in weapons.iter() {
        let weapon_collider = Aabb2d::new(
            weapon_transform.translation.truncate(),
            CAPSULE_COLLIDER + 5.,
        );

        if let Ok(result) = alien.get_single_mut() {
            let (alien_transform, mut alien) = result;
            let alien_collider =
                Aabb2d::new(alien_transform.translation.truncate(), CAPSULE_COLLIDER);

            if alien_collider.intersects(&weapon_collider) {
                alien.weapon = weapon.clone();
                commands.entity(weapon_entity).despawn();
            }
        }
    }
}

fn damage_enemy(
    commands: &mut Commands,
    ammo_entity: Entity,
    enemy_entity: Entity,
    enemy: &mut Enemy,
    damage: f32,
) {
    enemy.health -= damage;

    // Always despawns ammo
    commands.entity(ammo_entity).despawn();

    if enemy.health <= 0. {
        commands.entity(enemy_entity).despawn();
    }
}

fn damage_alien(commands: &mut Commands, alien_entity: Entity, alien: &mut Alien, damage: f32) {
    let new_damage = damage - alien.armor * 0.02;
    let mut new_alien_health = alien.health - new_damage;
    if new_alien_health <= 0. {
        new_alien_health = 0.;
    }

    alien.health = new_alien_health;

    commands.trigger(AlienHealthChanged {
        health: alien.health,
    });

    if alien.health <= 0. {
        // YOU DIED!!!
        println!("DEAD");
        commands.entity(alien_entity).despawn();
    }
}
