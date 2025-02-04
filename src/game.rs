use bevy::prelude::*;

use crate::draw::{draw_rectangle, draw_dashed_line};
use crate::ball::Ball;

const BORDER_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

#[derive(Resource)]
pub struct Game {
	width: f32,
	height: f32,
	pub ball: Vec<Ball>,
}

impl Game {
	pub fn new(width: f32, height: f32) -> Self {
		Self {
			width,
			height,
			ball: Vec::new(),
			// player1: Player::new(1),
			// player2: Player::new(2),
		}
	}

	// pub fn update(&mut self, delta: f32) {
	// 	println!("delta: {:?}, ball: {:?}", delta, self.ball);
	// }

	pub fn draw(&mut self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) {
		// Borda superior
		draw_rectangle(commands, meshes, materials, 0., self.height / 2. - 0.5, self.width, 1., BORDER_COLOR);
		// Borda inferior
		draw_rectangle(commands, meshes, materials, 0., (self.height / 2.) * -1. + 0.5, self.width, 1., BORDER_COLOR);
		// Borda esquerda
		draw_rectangle(commands, meshes, materials, self.width / 2. - 0.5, 0., 1., self.height, BORDER_COLOR);
		// Borda direita
		draw_rectangle(commands, meshes, materials, (self.width / 2.) * -1. + 0.5, 0., 1., self.height, BORDER_COLOR);
		// linha do meio
		draw_dashed_line(commands, meshes, materials, Vec2::new(0., self.height / 2. - 0.5), Vec2::new(0., (self.height / 2.) * -1. + 0.5), 1., 1., BORDER_COLOR);
	}
}
