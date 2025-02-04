use rand::prelude::*;
use bevy::prelude::*;

#[derive(Component, Copy, Clone, Debug)]
pub struct Ball {
	pub velocity: Vec3,
	pub x: f32,
	pub y: f32,
	pub radius: f32,
	pub width: f32,
	pub height: f32,
	pub color: Color,
}

impl Ball {
	pub fn new(width: f32, height: f32, color: Color) -> Self {
		let velocity_x = if rand::rng().random_bool(0.5) { -350.0 } else { 350.0 };
		let velocity_y = if rand::rng().random_bool(0.5) { -350.0 } else { 350.0 };

		Self {
			velocity: Vec3::new(velocity_x, velocity_y, 0.0),
			x: 0.0,
			y: 0.0,
			radius: 12.5,
			width: (width * 25. / 2.) - 24.5,
			height: (height * 25. / 2.) - 24.5,
			color,
		}
	}

	pub fn draw(&mut self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) {
		commands.spawn((
			Mesh2d(meshes.add(Mesh::from(Circle::default()))),
			MeshMaterial2d(materials.add(ColorMaterial::from(self.color))),
			Transform::from_translation(Vec3::new(self.x, self.y, 0.)).with_scale(Vec3::splat(25.)),
			self.clone(),
		));
	}

	pub fn update(&mut self, transform: &mut Transform, delta: f32) {
		if (transform.translation.x + self.radius >= self.width && self.velocity.x > 0.) || (transform.translation.x - self.radius <= self.width * -1. && self.velocity.x < 0.) {
			self.velocity.x *= -1.;
		}

		if (transform.translation.y + self.radius >= self.height && self.velocity.y > 0.) || (transform.translation.y - self.radius <= self.height * -1. && self.velocity.y < 0.) {
			self.velocity.y *= -1.;
		}

		transform.translation.x += self.velocity.x * delta;
		transform.translation.y += self.velocity.y * delta;

		self.x = transform.translation.x;
		self.y = transform.translation.y;
	}
}

pub fn check_collisions(mut query: Query<(&mut Transform, &mut Ball)>) {
	let mut balls = query.iter_mut().collect::<Vec<_>>();
	let len = balls.len();

	for i in 0..len {
		for j in (i + 1)..len {
			let distance = balls[i].0.translation.distance(balls[j].0.translation);
			let collision_distance = balls[i].1.radius + balls[j].1.radius;

			if distance < collision_distance {
				let idxs = [i, j];

				for idx in idxs.iter() {
					let space = collision_distance - distance;

					balls[*idx].1.velocity.x *= -1.;

					balls[*idx].0.translation.x += if balls[*idx].1.velocity.x >= 0. { space } else { -space };
					balls[*idx].0.translation.y += if balls[*idx].1.velocity.y >= 0. { space } else { -space };
				}
			}
		}
	}
}
