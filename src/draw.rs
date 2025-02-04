

use bevy::prelude::*;

const BLOCK_SIZE: f32 = 25.0;

pub fn to_coord(game_coord: f32) -> f32 {
	(game_coord as f32) * BLOCK_SIZE
}

pub fn draw_rectangle(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<ColorMaterial>>,
	x: f32,
	y: f32,
	width: f32,
	height: f32,
	color: Color,
) {
	commands.spawn((
		Mesh2d(meshes.add(Rectangle::default())),
		MeshMaterial2d(materials.add(color)),
		Transform::from_translation(Vec3::new(to_coord(x), to_coord(y), 0.)).with_scale(Vec3::new(to_coord(width), to_coord(height), 0.)),
	));
}

pub fn draw_dashed_line(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<ColorMaterial>>,
	mut start: Vec2,
	end: Vec2,
	dash_length: f32,
	gap_length: f32,
	color: Color,
) {
	while start.distance(end) > dash_length {
		let direction = (end - start).normalize();
		let dash_end = start + direction * dash_length;
		draw_rectangle(commands, meshes, materials, start.x, start.y, dash_length, dash_length, color);
		start = (dash_end + direction) * gap_length;
	}
}
