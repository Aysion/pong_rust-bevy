use bevy::{prelude::*, window};

mod draw;
mod game;
mod ball;

use draw::to_coord;
use game::Game;
use ball::{check_ball_collisions, Ball};

const BACKGROUND_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
const WIDTH: f32 = 61.;
const HEIGHT: f32 = 31.;

fn main() {
	let mut game = Game::new(WIDTH, HEIGHT);

	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(0.0, 0.0, 0.0)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(1.0, 0.0, 0.0)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(0.0, 1.0, 0.0)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(0.0, 0.0, 1.0)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(1.0, 1.0, 0.0)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(1.0, 0.0, 1.0)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(0.0, 1.0, 1.0)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(1.0, 1.0, 1.0)));

	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(0.5, 0.5, 0.5)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(0.5, 0.0, 0.0)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(0.0, 0.5, 0.0)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(0.0, 0.0, 0.5)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(0.5, 0.5, 0.0)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(0.5, 0.0, 0.5)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(0.0, 0.5, 0.5)));


	// iniciar o jogo com as bolas formando um circulo no centro da tela
	let mut angle: f32 = 0.0;
	let angle_step = 2.0 * std::f32::consts::PI / game.ball.len() as f32;
	for ball in &mut game.ball {
		ball.x = angle.cos() * 100.0;
		ball.y = angle.sin() * 100.0;
		angle += angle_step;
	}

	App::new()
			.insert_resource(ClearColor(BACKGROUND_COLOR))
			.insert_resource(game)
			.add_plugins(DefaultPlugins.set(WindowPlugin {
				primary_window: Some(Window {
					title: "Pong-Bevy".to_string(),
					resolution: window::WindowResolution::new(to_coord(WIDTH), to_coord(HEIGHT)),
					resizable: false,
					..default()
				}),
				..default()
			}))
			.add_systems(Startup, setup)
			// .add_systems(Update, |mut game: ResMut<Game>, time: Res<Time>| game.update(time.delta_secs()))
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
	for ball in  &mut game.ball {
		ball.draw(&mut commands, &mut meshes, &mut materials);
	}
}

fn update_balls(time: Res<Time>, mut query: Query<(&mut Transform, &mut Ball)>) {
	let delta = time.delta_secs();

	for mut params in &mut query {
		params.1.update(&mut params.0, delta);
	}
}
