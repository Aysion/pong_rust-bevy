use bevy::{prelude::*, window};

mod draw;
mod game;
mod ball;
mod player;

use game::Game;
use player::check_player_collisions;
use ball::{check_ball_collisions, update_balls};

const BACKGROUND_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
const WIDTH: f32 = 61. * 25.;
const HEIGHT: f32 = 31. * 25.;

fn main() {
	let game = Game::new(WIDTH, HEIGHT);

	App::new()
			.insert_resource(ClearColor(BACKGROUND_COLOR))
			.insert_resource(game)
			.add_plugins(DefaultPlugins.set(WindowPlugin {
				primary_window: Some(Window {
					title: "Pong-Bevy".to_string(),
					resolution: window::WindowResolution::new(WIDTH, HEIGHT),
					resizable: false,
					..default()
				}),
				..default()
			}))
			.add_systems(Startup, setup)
			// .add_systems(Update, |mut game: ResMut<Game>, time: Res<Time>| game.update(time.delta_secs()))
			.add_systems(Update, check_player_collisions)
			.add_systems(Update, check_ball_collisions)
			.add_systems(Update, update_balls)
			.run();
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut game: ResMut<Game>,
) {
	commands.spawn(Camera2d);
	game.draw(&mut commands, &mut meshes, &mut materials);
}
