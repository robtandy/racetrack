use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::{PrimaryWindow, WindowResolution};
use gloo_timers::future::TimeoutFuture;
use leptos::*;
use racetrack::*;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct Point {
    lat: f32,
    long: f32,
}

#[derive(Resource)]
struct Track {
    points: Arc<Mutex<Vec<Point>>>,
    new_points: Arc<Mutex<Vec<Point>>>,
}

impl Track {}
impl FromWorld for Track {
    fn from_world(world: &mut World) -> Self {
        let points = Arc::new(Mutex::new(Vec::new()));
        let new_points = Arc::new(Mutex::new(Vec::new()));

        let npc = Arc::clone(&new_points);

        watch_location(move |lat, long| {
            let mut guard = npc.lock().unwrap();
            guard.push(Point {
                lat: lat as f32,
                long: long as f32,
            });
        });
        /*spawn_local(async move {
            let mut counter = 0.0;
            loop {
                TimeoutFuture::new(100).await;
                let mut guard = npc.lock().unwrap();
                guard.push(Point {
                    long: -81.0 + counter,
                    lat: 41.0 + counter,
                });
                counter += 1.0;
            }
        });*/

        Track { points, new_points }
    }
}

pub fn main() {
    console_log::init_with_level(log::Level::Info).expect("console log to work");
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <LocatorComponent/>
        }
    });

    log::info!("in main");

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // provide the ID selector string here
            canvas: Some("#map-canvas".into()),
            resolution: WindowResolution::new(500., 500.),
            ..default()
        }),
        ..default()
    }))
    .add_systems(Startup, setup)
    .add_systems(Update, check_for_new_points)
    .add_systems(Update, move_camera)
    .init_resource::<Track>()
    .run();
}

fn move_camera(
    mut q_camera: Query<(&mut Transform, &mut Camera, &mut OrthographicProjection)>,
    mut q_window: Query<(&mut Window, &PrimaryWindow)>,
    mut track: ResMut<Track>,
) {
    let (mut transform, mut cam, mut proj) = q_camera.get_single_mut().expect("valid cam");
    let (mut window, _) = q_window.get_single_mut().expect("valid window");

    let (w, h) = (window.width(), window.height());

    let points = track.points.lock().expect("locked_points");

    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for point in points.iter() {
        min_x = min_x.min(point.lat);
        max_x = max_x.max(point.lat);
        min_y = min_y.min(point.long);
        max_y = max_y.max(point.long);
    }

    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;

    transform.translation = Vec3::new(center_x, center_y, transform.translation.z);

    let scale_x = (max_x - min_x) / w * 1.5;
    let scale_y = (max_y - min_y) / h * 1.5;

    //window.resolution.set_scale_factor(scale_x.min(scale_y));
    info!("current scale factor {}", window.resolution.scale_factor());
    let scale = scale_x.max(scale_y);
    if points.len() > 2 {
        info!(
            "setting to scale factor {}, len tracks {}",
            scale,
            points.len()
        );
        proj.scale = scale;
    }
}

fn check_for_new_points(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut track: ResMut<Track>,
) {
    let color_material = materials.add(Color::srgb(0.8, 0.8, 0.8));

    let mut new_points = track.new_points.lock().expect("locked new points");
    let mut points = track.points.lock().expect("locked points");

    for new_point in new_points.iter() {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 3.0 })),
            material: color_material.clone(),
            transform: Transform::from_xyz(new_point.lat, new_point.long, 0.0),
            ..default()
        });
        points.push(new_point.clone());
    }
    *new_points = vec![];
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
