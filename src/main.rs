use bevy::{prelude::*, window};

mod draw;
mod game;
mod ball;

use draw::to_coord;
use game::Game;
use ball::{check_collisions, Ball};

const BACKGROUND_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const WIDTH: f32 = 61.;
const HEIGHT: f32 = 31.;

fn main() {
	let mut game = Game::new(WIDTH, HEIGHT);

	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(0., 0., 0.)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(1., 0., 0.)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(0., 1., 0.)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(0., 0., 1.)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(1., 1., 0.)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(1., 0., 1.)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(0., 1., 1.)));
	game.ball.push(Ball::new(WIDTH, HEIGHT, Color::srgb(1., 1., 1.)));

	game.ball[0].x = -300.;
	game.ball[0].y = HEIGHT / 2. * -1.0;
	game.ball[1].x = -200.;
	game.ball[1].y = HEIGHT / 3. * -1.0;
	game.ball[2].x = -100.0;
	game.ball[2].y = HEIGHT / 4. * -1.0;
	game.ball[3].x = 0.0;
	game.ball[3].y = HEIGHT / 5. * -1.0;
	game.ball[4].x = 100.0;
	game.ball[4].y = HEIGHT / 6. * 1.0;
	game.ball[5].x = 200.0;
	game.ball[5].y = HEIGHT / 7. * 1.0;
	game.ball[6].x = 300.0;
	game.ball[6].y = HEIGHT / 8. * 1.0;
	game.ball[7].x = 400.0;
	game.ball[7].y = HEIGHT / 9. * 1.0;

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
			.add_systems(Update, check_collisions)
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
