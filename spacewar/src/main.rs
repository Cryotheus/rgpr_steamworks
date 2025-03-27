use bevy::prelude::*;
use rgpr_steamworks::prelude::*;
use std::f32::consts::{PI, TAU};

use bevy::render::camera::CameraProjection;
use bevy::render::primitives::Frustum;

#[derive(Component, Debug)]
struct Player {
	color: Color,
	steam_id: SteamId,
}

#[derive(Component, Debug)]
struct Ship {
	color: Color,
	direction: f32,
	position: Vec2,
	player_entity: Entity,
	velocity: Vec2,
	shoot_cooldown: Timer,
}

#[derive(Component, Debug)]
struct ShipAssets {}

fn main() {
	let mut app = App::new();

	app.add_plugins(DefaultPlugins).add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin).add_systems(Startup, startup);

	//run the app!
	app.run();
}

fn startup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
	const PX_PER_METER: f32 = 96.0;

	//setup view
	let mut ortho = OrthographicProjection::default_2d();
	ortho.scale = 1.0 / PX_PER_METER;

	let frustum: Frustum = ortho.compute_frustum(&GlobalTransform::from(Transform::default()));

	commands.spawn((Camera2d, ortho, frustum));

	//setup ground
	let rect = meshes.add(Rectangle::new(8.0, 1.0));
	let color = Color::linear_rgb(1.0, 0.2, 0.1);
	let material_2d = MeshMaterial2d(materials.add(color));

	commands.spawn((Mesh2d(rect), material_2d.clone(), Transform::from_xyz(0.0, -0.5, 0.0)));
}

fn spawn_ships(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, players: Query<(Entity, &Player)>) {
	let players_vec = players.iter().collect::<Vec<_>>();
	let players_count = players_vec.len();

	for (index, (entity, player)) in players_vec.into_iter().enumerate() {
		let color = player.color;
		let angle = (index as f32 / players_count as f32) * TAU;

		const RADIUS: f32 = 24.0;

		commands.spawn((
			Ship {
				color,
				direction: (angle + PI) % TAU,
				position: Vec2::new(angle.cos() * RADIUS, angle.sin() * RADIUS),
				player_entity: entity,
				velocity: Default::default(),
				shoot_cooldown: Default::default(),
			},
			//stay open!
			ShipAssets {
				//juice
			},
		));
	}
}
