use bevy::prelude::*;
use rand::random_bool;

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
		let velocity_x = if random_bool(0.5) { -300.0 } else { 300.0 };
		let velocity_y = if random_bool(0.5) { -300.0 } else { 300.0 };

		Self {
			velocity: Vec3::new(velocity_x, velocity_y, 0.0),
			x: 0.0,
			y: 0.0,
			radius: 12.5,
			width: (width / 2.) - 24.5,
			height: (height / 2.) - 24.5,
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

pub fn update_balls(time: Res<Time>, mut query: Query<(&mut Transform, &mut Ball)>) {
	let delta = time.delta_secs();

	for mut params in &mut query {
		params.1.update(&mut params.0, delta);
	}
}

pub fn check_ball_collisions(mut query: Query<(&mut Transform, &mut Ball)>) {
	let mut balls = query.iter_mut().collect::<Vec<_>>();
	let len = balls.len();

	for a in 0..len {
		for b in (a + 1)..len {
			let distance = balls[a].0.translation.distance(balls[b].0.translation);
			let collision_distance = balls[a].1.radius + balls[b].1.radius;

			if distance < collision_distance {
				// Calcular a resposta de colisão
				let normal = (balls[b].0.translation - balls[a].0.translation).normalize();
				let relative_velocity = balls[b].1.velocity - balls[a].1.velocity;
				let velocity_along_normal = relative_velocity.dot(normal);

				if velocity_along_normal > 0.0 {
					continue;
				}

				let restitution = 1.005; // Coeficiente de restituição (1.0 para colisão elástica); 0.0 para colisão inelástica (bolas grudam)
				let impulse_scalar = -restitution * velocity_along_normal;
				let mut impulse = impulse_scalar * normal;

				if impulse.x.is_nan() || impulse.y.is_nan() {
					continue;
				}

				// limitar a velocidade de resposta
				let max_impulse = 2000.0;
				if impulse.x.abs() > max_impulse { impulse.x = impulse.x.signum() * max_impulse; }
				if impulse.y.abs() > max_impulse { impulse.y = impulse.y.signum() * max_impulse; }

				balls[a].1.velocity -= impulse;
				balls[b].1.velocity += impulse;
			}
		}
	}
}
