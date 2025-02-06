use bevy::prelude::*;

use crate::draw::{draw_rectangle, draw_dashed_line};
use crate::ball::Ball;
use crate::player::Player;

const BORDER_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Resource)]
pub struct Game {
	width: f32,
	height: f32,
	pub ball: Vec<Ball>,
	pub player1: Player,
	pub player2: Player,
}

impl Game {
	pub fn new(width: f32, height: f32) -> Self {
		let mut balls = Vec::new();

		balls.push(Ball::new(width, height, Color::srgb(0.0, 0.0, 0.0)));
		balls.push(Ball::new(width, height, Color::srgb(1.0, 0.0, 0.0)));
		balls.push(Ball::new(width, height, Color::srgb(0.0, 1.0, 0.0)));
		balls.push(Ball::new(width, height, Color::srgb(0.0, 0.0, 1.0)));
		balls.push(Ball::new(width, height, Color::srgb(1.0, 1.0, 0.0)));
		balls.push(Ball::new(width, height, Color::srgb(1.0, 0.0, 1.0)));
		balls.push(Ball::new(width, height, Color::srgb(0.0, 1.0, 1.0)));
		balls.push(Ball::new(width, height, Color::srgb(1.0, 1.0, 1.0)));
		balls.push(Ball::new(width, height, Color::srgb(0.5, 0.5, 0.5)));
		balls.push(Ball::new(width, height, Color::srgb(0.5, 0.0, 0.0)));
		balls.push(Ball::new(width, height, Color::srgb(0.0, 0.5, 0.0)));
		balls.push(Ball::new(width, height, Color::srgb(0.0, 0.0, 0.5)));
		balls.push(Ball::new(width, height, Color::srgb(0.5, 0.5, 0.0)));
		balls.push(Ball::new(width, height, Color::srgb(0.5, 0.0, 0.5)));
		balls.push(Ball::new(width, height, Color::srgb(0.0, 0.5, 0.5)));


		// iniciar o jogo com as bolas formando um circulo no centro da tela
		let mut angle: f32 = 0.0;
		let angle_step = 2.0 * std::f32::consts::PI / balls.len() as f32;
		for ball in &mut balls {
			ball.x = angle.cos() * 100.0;
			ball.y = angle.sin() * 100.0;
			angle += angle_step;
		}

		Self {
			width,
			height,
			ball: balls,
			player1: Player::new(25., 25. * 5., -(width / 2. - 37.5), 0., Color::srgb(0.0, 0.0, 0.0)),
			player2: Player::new(25., 25. * 5., width / 2. - 37.5, 0., Color::srgb(0.0, 0.0, 0.0)),
		}
	}

	// pub fn update(&mut self, delta: f32) {
	// 	println!("delta: {:?}, ball: {:?}", delta, self.ball);
	// }

	pub fn draw(&mut self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) {
		// Borda superior
		draw_rectangle(commands, meshes, materials, 0., self.height / 2. - 12.5, self.width, 25., BORDER_COLOR);
		// Borda inferior
		draw_rectangle(commands, meshes, materials, 0., (self.height / 2.) * -1. + 12.5, self.width, 25., BORDER_COLOR);
		// Borda esquerda
		draw_rectangle(commands, meshes, materials, -(self.width / 2. - 12.5), 0., 25., self.height, BORDER_COLOR);
		// Borda direita
		draw_rectangle(commands, meshes, materials, self.width / 2. - 12.5, 0., 25., self.height, BORDER_COLOR);
		// linha do meio
		draw_dashed_line(commands, meshes, materials, Vec2::new(0., self.height / 2.0 - 37.5), Vec2::new(0., -(self.height / 2.)), 25.0, 25.0, BORDER_COLOR);

		self.player1.draw(commands, meshes, materials);
		self.player2.draw(commands, meshes, materials);

		for ball in &mut self.ball {
			ball.draw(commands, meshes, materials);
		}
	}
}
