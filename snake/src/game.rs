use bevy::{prelude::*, time::FixedTimestep};

use crate::{
	food::{self, Food, FoodBundle},
	snake::{self, Direction, Head, Segment, Segments},
};

pub struct GameOverEvent;

pub fn system_set() -> SystemSet {
	SystemSet::new()
		.with_run_criteria(FixedTimestep::steps_per_second(5.0))
		.with_system(snake::movement)
		.with_system(handle_game_over.after(snake::movement))
		.with_system(snake::eating.after(snake::movement))
		.with_system(food::spawn.after(snake::eating))
		.with_system(snake::growth.after(snake::eating))
}

pub fn setup_game(mut commands: Commands, mut segments: ResMut<Segments>) {
	*segments = Segments::new(&mut commands);
	commands.spawn(FoodBundle::new());
}

fn handle_game_over(
	mut commands: Commands,
	reader: EventReader<GameOverEvent>,
	segments_res: ResMut<Segments>,
	food: Query<Entity, With<Food>>,
	segments: Query<Entity, With<Segment>>,
) {
	if !reader.is_empty() {
		reader.clear();

		for ent in food.iter().chain(segments.iter()) {
			commands.entity(ent).despawn();
		}

		setup_game(commands, segments_res);
	}
}

pub fn keyboard_input(input: Res<Input<KeyCode>>, mut positions: Query<&mut Head>) {
	let dir = if input.just_pressed(KeyCode::Left) || input.just_pressed(KeyCode::A) {
		Direction::Left
	} else if input.just_pressed(KeyCode::Right) || input.just_pressed(KeyCode::D) {
		Direction::Right
	} else if input.just_pressed(KeyCode::Down) || input.just_pressed(KeyCode::S) {
		Direction::Down
	} else if input.just_pressed(KeyCode::Up) || input.just_pressed(KeyCode::W) {
		Direction::Up
	} else {
		return;
	};

	for mut head in &mut positions {
		if Some(dir.opposite()) != head.direction {
			head.direction = Some(dir);
		}
	}
}

pub fn touch_input(touches: Res<Touches>, mut positions: Query<&mut Head>) {
	for touch in touches.iter_just_released() {
		let delta = touch.delta();
		if delta.normalize_or_zero() == Vec2::ZERO {
			continue;
		}

		let abs = delta.abs();

		let dir = if abs.x > abs.y {
			if delta.x > 0.0 {
				Direction::Right
			} else {
				Direction::Left
			}
		} else {
			if delta.y > 0.0 {
				Direction::Up
			} else {
				Direction::Down
			}
		};

		for mut head in &mut positions {
			if Some(dir.opposite()) != head.direction {
				head.direction = Some(dir);
			}
		}
	}
}
