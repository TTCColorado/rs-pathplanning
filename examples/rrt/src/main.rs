extern crate gnuplot;
extern crate pathplanning;
extern crate serde;

use geo::algorithm::euclidean_length::EuclideanLength;
use geo::{LineString, Polygon};
use gnuplot::{Color, Figure};
use pathplanning::rrt::{Robot, Space, RRT};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread::sleep;

#[derive(Serialize, Deserialize)]
struct Conf {
    bounds: Vec<(f64, f64)>,
    obstacles: Vec<Vec<(f64, f64)>>,
    // path: Vec<(f64, f64)>,
    start: (f64, f64, f64),
    goal: (f64, f64, f64),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut file = File::open(file_path).expect("file should open");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("should read file");

    let conf: Conf = serde_json::from_str(&contents).expect("should deserialize");

    let mut fg = Figure::new();

    let obstacle_list: Vec<Polygon<f64>> = conf
        .obstacles
        .into_iter()
        .map(|o| Polygon::new(LineString::from(o), vec![]))
        .collect();

    let bounds = Polygon::new(LineString::from(conf.bounds), vec![]);

    let (bx, by): (Vec<f64>, Vec<f64>) = bounds.exterior().points().map(|p| p.x_y()).unzip();

    let robot = Robot::new(0.0, 0.0, 0.8);
    let space = Space::new(bounds, robot, obstacle_list.clone());

    let buffer_obs = space.get_obs();
    let (bbx, bby): (Vec<f64>, Vec<f64>) = space
        .get_bounds()
        .exterior()
        .points()
        .map(|p| p.x_y())
        .unzip();

    let planner = RRT::new(
        (conf.start.0, conf.start.1).into(),
        conf.start.2,
        (conf.goal.0, conf.goal.1).into(),
        conf.goal.2,
        10,
        0.1,
        space,
    );

    let axes = Arc::new(Mutex::new(fg.axes2d()));
    axes.lock().unwrap().lines(&bx, &by, &[Color("black")])
        .lines(&bbx, &bby, &[Color("blue")])
        .points(
            &[conf.start.0, conf.goal.0],
            &[conf.start.1, conf.goal.1],
            &[Color("green")],
        );

    println!("Start planner");

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        // .stack_size(32 * 1024 * 1024)
        .build()
        .unwrap();

    pool.install(|| {
        (0..100)
            .into_par_iter()
            .map(|_|
                planner.plan_one_return_incomplete()
            )
            .filter_map(|r| r)
            .for_each(|path| {
                let (px, py): (Vec<f64>, Vec<f64>) = path.points().map(|p| p.x_y()).unzip();
                axes.lock().unwrap().lines(&px, &py, &[Color("orange")]);
            });
    });

    for obs in obstacle_list.iter() {
        let (ox, oy): (Vec<f64>, Vec<f64>) = obs.exterior().points().map(|p| p.x_y()).unzip();
        axes.lock().unwrap().lines(&ox, &oy, &[Color("red")]);
    }

    for obs in buffer_obs.iter() {
        let (ox, oy): (Vec<f64>, Vec<f64>) = obs.exterior().points().map(|p| p.x_y()).unzip();
        axes.lock().unwrap().lines(&ox, &oy, &[Color("blue")]);
    }

    fg.show();
    sleep(std::time::Duration::from_secs(10));
}
