use crate::{
    prelude::*, ChangeBackgroundTexture, CurrentBoss, CurrentGameLevelChanged,
    CurrentGameLevelUI, CurrentScore, CurrentTime, CurrentTimeUI, CurrentWave, CurrentWaveUI,
    PlayerHealthChanged, PlayerProfileUISet, ScoreChanged, SetupNewTime,
};

#[derive(Component, Clone)]
pub struct CleanupWhenPlayerDies;

pub fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    q.iter().for_each(|e| {
        commands.entity(e).despawn_recursive();
    });
}

pub fn reset_initial_state(
    mut commands: Commands,
    mut current_boss: ResMut<CurrentBoss>,
    mut current_wave: ResMut<CurrentWave>,
    mut current_time: ResMut<CurrentTime>,
    mut current_score: ResMut<CurrentScore>,
    mut current_wave_ui: Query<
        (&mut Text, &CurrentWaveUI),
        (Without<CurrentTimeUI>, Without<CurrentGameLevelUI>),
    >,
) {
    current_boss.0 = None;

    // update current game level and its UI
    commands.trigger(CurrentGameLevelChanged(1));

    // update texture background
    commands.trigger(ChangeBackgroundTexture);

    // Update UI
    current_wave.0 = 1u16;
    if let Ok((mut text, _)) = current_wave_ui.get_single_mut() {
        text.sections.first_mut().unwrap().value = format!("Wave #{}", current_wave.0);
    }
    current_time.minutes = 0u16;
    current_time.seconds = 30u16;
    current_score.0 = 0.0;

    commands.trigger(ScoreChanged { score: 0.0 });
    commands.trigger(SetupNewTime);
    commands.trigger(PlayerProfileUISet);

    commands.trigger(PlayerHealthChanged {
        health: PLAYER_HEALTH,
    });
}
