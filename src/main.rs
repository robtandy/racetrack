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

const fake_data: &[(f32, f32)] = &[
    (41.2614911, -81.4245030),
    (41.2614044, -81.4248437),
    (41.2613177, -81.4248973),
    (41.2612270, -81.4249268),
    (41.2611221, -81.4249134),
    (41.2609407, -81.4248517),
    (41.2605939, -81.4244628),
    (41.2604729, -81.4240497),
    (41.2603842, -81.4235348),
    (41.2598962, -81.4234650),
    (41.2595575, -81.4232183),
    (41.2592268, -81.4227033),
    (41.2589768, -81.4224029),
    (41.2587832, -81.4221239),
    (41.2586139, -81.4219308),
    (41.2585010, -81.4218128),
    (41.2583155, -81.4215767),
    (41.2581541, -81.4213300),
    (41.2579444, -81.4210618),
    (41.2577912, -81.4208472),
    (41.2575734, -81.4205039),
    (41.2573637, -81.4202464),
    (41.2571621, -81.4199889),
    (41.2570169, -81.4197636),
    (41.2569363, -81.4194739),
    (41.2569282, -81.4191091),
    (41.2570653, -81.4187980),
    (41.2573315, -81.4184439),
    (41.2575976, -81.4181006),
    (41.2577509, -81.4177895),
    (41.2580009, -81.4175642),
    (41.2581622, -81.4172530),
    (41.2582751, -81.4169204),
    (41.2583235, -81.4168131),
    (41.2584042, -81.4165449),
    (41.2586623, -81.4166522),
    (41.2589849, -81.4167380),
    (41.2592752, -81.4169955),
    (41.2597753, -81.4174247),
    (41.2600495, -81.4178753),
    (41.2604205, -81.4186049),
    (41.2605334, -81.4192057),
    (41.2605656, -81.4199138),
    (41.2605495, -81.4205360),
    (41.2605495, -81.4211154),
    (41.2606302, -81.4214158),
    (41.2607108, -81.4216948),
    (41.2608237, -81.4220166),
    (41.2609689, -81.4223170),
    (41.2610334, -81.4224672),
    (41.2610979, -81.4226174),
    (41.2610334, -81.4228106),
    (41.2608882, -81.4230037),
    (41.2607753, -81.4231968),
    (41.2606140, -81.4233685),
];

impl Track {}
impl FromWorld for Track {
    fn from_world(world: &mut World) -> Self {
        let points = Arc::new(Mutex::new(Vec::new()));
        let new_points = Arc::new(Mutex::new(Vec::new()));

        let npc = Arc::clone(&new_points);

        watch_location(move |lat, long| {
            let mut guard = npc.lock().unwrap();
            guard.push(Point {
                lat: lat as f32 * 10000.0,
                long: long as f32 * 10000.0,
            });
        });
        /*spawn_local(async move {
            let mut i = 0;
            loop {
                TimeoutFuture::new(200).await;
                let mut guard = npc.lock().unwrap();
                guard.push(Point {
                    long: fake_data[i].0 * 10000.0,
                    lat: fake_data[i].1 * 10000.0,
                });
                i += 1;
                if i >= fake_data.len() {
                    i = 0;
                }
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
            //resolution: WindowResolution::new(500., 500.),
            fit_canvas_to_parent: true,
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

    let scale_x = (max_x - min_x) / w * 1.25;
    let scale_y = (max_y - min_y) / h * 1.25;

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
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 2.0 })),
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
