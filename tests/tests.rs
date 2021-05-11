use rl_ball_sym::{linear_algebra::vector::Vec3, load_soccar, simulation::ball::Ball};
#[macro_use]
extern crate json;
use std::fs;
use std::time::Instant;

#[test]
fn gamemode_soccar() {
    let game = load_soccar(0, 0);

    // test all the default values to make sure they're proper

    assert_eq!(game.index, 0 as u8);
    assert_eq!(game.team, 0 as u8);

    assert_eq!(game.field.field_mesh.ids.len(), 24084);
    assert_eq!(game.field.field_mesh.vertices.len(), 13152);

    assert_eq!(game.gravity.x as i64, 0);
    assert_eq!(game.gravity.y as i64, 0);
    assert_eq!(game.gravity.z as i64, -650);

    dbg!(game.field.collision_mesh.root.box_);

    assert_eq!(game.field.collision_mesh.num_leaves, 8028 as u64);

    assert_eq!(game.ball.time as i64, 0);
    assert_eq!(game.ball.location.x as i64, 0);
    assert_eq!(game.ball.location.y as i64, 0);
    assert_eq!(game.ball.location.z as i64, 102);
    assert_eq!(game.ball.velocity.x as i64, 0);
    assert_eq!(game.ball.velocity.y as i64, 0);
    assert_eq!(game.ball.velocity.z as i64, 0);
    assert_eq!(game.ball.angular_velocity.x as i64, 0);
    assert_eq!(game.ball.angular_velocity.y as i64, 0);
    assert_eq!(game.ball.angular_velocity.z as i64, 0);
    assert_eq!(game.ball.radius as i64, 91);
    assert_eq!(game.ball.collision_radius as i64, 93);
}

#[test]
fn basic_predict() {
    let game = load_soccar(0, 0);

    assert_eq!(game.ball.time as i64, 0);
    assert_eq!(game.ball.location.x as i64, 0);
    assert_eq!(game.ball.location.y as i64, 0);
    assert_eq!(game.ball.location.z as i64, 102);
    assert_eq!(game.ball.velocity.x as i64, 0);
    assert_eq!(game.ball.velocity.y as i64, 0);
    assert_eq!(game.ball.velocity.z as i64, 0);
    assert_eq!(game.ball.angular_velocity.x as i64, 0);
    assert_eq!(game.ball.angular_velocity.y as i64, 0);
    assert_eq!(game.ball.angular_velocity.z as i64, 0);
    assert_eq!(game.ball.radius as i64, 91);
    assert_eq!(game.ball.collision_radius as i64, 93);

    let ball = Ball {
        time: 0.098145,
        location: Vec3 {
            x: -2294.524658,
            y: 1684.135986,
            z: 317.176727,
        },
        velocity: Vec3 {
            x: 1273.753662,
            y: -39.792305,
            z: 763.282715,
        },
        angular_velocity: Vec3 {
            x: 2.3894,
            y: -0.8755,
            z: 3.8078,
        },
        radius: game.ball.radius,
        collision_radius: game.ball.collision_radius,
        moi: game.ball.moi,
    };

    let start = Instant::now();
    let time = 10.;
    let ball_prediction = ball.get_ball_prediction_struct_for_time(&game, time);
    println!("Ran ball prediction in {}", start.elapsed().as_secs_f32());
    let last_slice = &ball_prediction.slices[ball_prediction.num_slices - 1];

    assert_eq!(ball_prediction.num_slices, time as usize * 120);
    println!("{:?}", last_slice);
    assert!(last_slice.location.z > 0.);

    let mut json_obj = json::JsonValue::new_array();
    for ball in ball_prediction.slices {
        json_obj
            .push(object! {
                time: ball.time,
                location: object! {
                    x: ball.location.x,
                    y: ball.location.y,
                    z: ball.location.z
                },
                velocity: object! {
                    x: ball.velocity.x,
                    y: ball.velocity.y,
                    z: ball.velocity.z
                }
            })
            .unwrap();
    }
    fs::write("ball_prediction.json", json_obj.dump()).expect("Unable to write file");
}

#[test]
fn fast_execution() {
    let game = load_soccar(0, 0);
    let runs = 5000;
    let mut times: Vec<f32> = Vec::with_capacity(runs);
    println!("Testing for average ball prediction struct generation time - running function {} times.", &runs);

    for _ in 0..runs {
        let start = Instant::now();
        game.ball.get_ball_prediction_struct(&game);
        times.push(start.elapsed().as_secs_f32());
    }

    let elapsed: f32 = times.iter().sum::<f32>() / (runs as f32);
    let elapsed_ms = elapsed * 1000.;
    println!("Ran ball prediction in an average of {} seconds ({}ms)", &elapsed, elapsed_ms);

    let ball_prediction = game.ball.get_ball_prediction_struct(&game);
    assert_eq!(ball_prediction.num_slices, 720);
    assert!(elapsed_ms < 2.5);
}
