use bevy::prelude::*;

use crate::ball::Ball;

#[derive(Component, Copy, Clone, Debug)]
pub struct Player {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
	pub color: Color,
}

impl Player {
	pub fn new(width: f32, height: f32, x: f32, y: f32, color: Color) -> Self {
		Self {
			x,
			y,
			width,
			height,
			color,
		}
	}

	pub fn draw(&mut self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) {
		commands.spawn((
			Mesh2d(meshes.add(Rectangle::default())),
			MeshMaterial2d(materials.add(ColorMaterial::from(self.color))),
			Transform::from_translation(Vec3::new(self.x, self.y, 0.)).with_scale(Vec3::new(self.width, self.height, 0.)),
			self.clone(),
		));
	}

	// pub fn update(&mut self, transform: &mut Transform, delta: f32) {
	// 	// transform.translation.x = self.x;
	// 	transform.translation.y = self.y;
	// }

	// pub fn move_up(&mut self, delta: f32) {
	// 	self.y += 300.0 * delta;
	// }

	// pub fn move_down(&mut self, delta: f32) {
	// 	self.y -= 300.0 * delta;
	// }

	pub fn check_collision(&mut self, ball: &Ball) -> bool {
		let player_x_left = self.x - self.width / 2.;
		let player_x_right = self.x + self.width / 2.;
		let player_y_top = self.y + self.height / 2.;
		let player_y_bottom = self.y - self.height / 2.;

		let ball_x_left = ball.x - ball.radius;
		let ball_x_right = ball.x + ball.radius;
		let ball_y_top = ball.y + ball.radius;
		let ball_y_bottom = ball.y - ball.radius;

		// ((Player no lado esquerdo) || (Player no lado direito))
		(
			(self.x < 0.0 && player_x_right >= ball_x_left)
			|| (self.x > 0.0 && player_x_left <= ball_x_right)
		)
		&& player_y_top >= ball_y_bottom
		&& player_y_bottom <= ball_y_top
	}
}

pub fn check_player_collisions(
	mut player_query: Query<(&mut Player, &Transform), With<Player>>,
	mut ball_query: Query<(&mut Ball, &mut Transform), (With<Ball>, Without<Player>)>,
) {
	for (mut player, player_transform) in player_query.iter_mut() {
		for (mut ball, mut ball_transform) in ball_query.iter_mut() {
			if player.check_collision(&ball) {
				let player_pos = player_transform.translation;
				let ball_pos = ball_transform.translation;

				// Detectar se a colisão foi na parte de cima ou de baixo do player
				if ball_pos.y > player_pos.y + player.height / 2.0 || ball_pos.y < player_pos.y - player.height / 2.0 {
					// Fas o deslocamento da bola para a parte de cima ou de baixo do player
					// para evitar que a bola fique presa no player
					ball_transform.translation.y = if ball_pos.y > player_pos.y {
						player_pos.y + player.height / 2.0 + ball.radius
					} else {
						player_pos.y - player.height / 2.0 - ball.radius
					};

					ball.velocity.y *= -1.;
				} else {
					// Colisão na parte da frente do player
					ball.velocity.x *= -1.0;
				}
			}
		}
	}
}
