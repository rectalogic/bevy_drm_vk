use bevy::{color::palettes, prelude::*, render::RenderPlugin, winit::WinitPlugin};
use bevy_drm::{DrmPlugin, render_creation};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                // Prevents WindowPlugin::build from inserting RawHandleWrapperHolder which creates a surface in bevy_render::renderer::initialize_renderer from RenderPlugin::build
                // Must set WGPU_ADAPTER_NAME env var so we find the correct GPU
                primary_window: None,
                ..default()
            })
            .set(RenderPlugin {
                render_creation: render_creation(),
                ..default()
            })
            .build()
            .disable::<WinitPlugin>(),
        DrmPlugin,
    ))
    .add_systems(Startup, setup);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 3.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(palettes::css::RED))),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(palettes::css::GREEN))),
        Transform::from_xyz(-1.5, 1.0, 0.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 2.0))),
        MeshMaterial3d(materials.add(Color::from(palettes::css::MEDIUM_BLUE))),
        Transform::from_xyz(0.0, -1.0, 1.5),
    ));
    commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 3.0, 2.0)));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(1.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
