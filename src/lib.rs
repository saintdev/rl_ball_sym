pub mod linear_algebra;
pub mod simulation;

use glam::{vec3a};
use simulation::ball::Ball;
use simulation::field::{initialize_dropshot, initialize_hoops, initialize_soccar, initialize_throwback};
use simulation::game::Game;
use simulation::mesh::Mesh;

use crate::simulation::field::InitializeThrowbackParams;

#[allow(unused)]
mod tables {
    include!(concat!(env!("OUT_DIR"), "/mesh_tables.rs"));
}

pub fn load_soccar() -> Game {
    let soccar_corner = Mesh {
        ids: tables::SOCCAR_CORNER_IDS.into(),
        vertices: tables::SOCCAR_CORNER_VERTICES.into(),
    };
    let soccar_goal = Mesh {
        ids: tables::SOCCAR_GOAL_IDS.into(),
        vertices: tables::SOCCAR_GOAL_VERTICES.into(),
    };
    let soccar_ramps_0 = Mesh {
        ids: tables::SOCCAR_RAMPS_0_IDS.into(),
        vertices: tables::SOCCAR_RAMPS_0_VERTICES.into(),
    };
    let soccar_ramps_1 = Mesh {
        ids: tables::SOCCAR_RAMPS_1_IDS.into(),
        vertices: tables::SOCCAR_RAMPS_1_VERTICES.into(),
    };

    let collision_mesh = initialize_soccar(&soccar_corner, &soccar_goal, &soccar_ramps_0, &soccar_ramps_1);

    let ball = Ball::initialize_soccar();

    let gravity = vec3a(0., 0., -650.);

    Game {
        gravity,
        collision_mesh,
        ball,
    }
}

pub fn load_hoops() -> Game {
    let hoops_corner = Mesh {
        ids: tables::HOOPS_CORNER_IDS.into(),
        vertices: tables::HOOPS_CORNER_VERTICES.into(),
    };
    let hoops_net = Mesh {
        ids: tables::HOOPS_NET_IDS.into(),
        vertices: tables::HOOPS_NET_VERTICES.into(),
    };
    let hoops_rim = Mesh {
        ids: tables::HOOPS_RIM_IDS.into(),
        vertices: tables::HOOPS_RIM_VERTICES.into(),
    };
    let hoops_ramps_0 = Mesh {
        ids: tables::HOOPS_RAMPS_0_IDS.into(),
        vertices: tables::HOOPS_RAMPS_0_VERTICES.into(),
    };
    let hoops_ramps_1 = Mesh {
        ids: tables::HOOPS_RAMPS_1_IDS.into(),
        vertices: tables::HOOPS_RAMPS_1_VERTICES.into(),
    };

    let collision_mesh = initialize_hoops(&hoops_corner, &hoops_net, &hoops_rim, &hoops_ramps_0, &hoops_ramps_1);

    let ball = Ball::initialize_hoops();

    let gravity = vec3a(0., 0., -650.);

    Game {
        gravity,
        collision_mesh,
        ball,
    }
}

pub fn load_dropshot() -> Game {
    let dropshot = Mesh {
        ids: tables::DROPSHOT_IDS.into(),
        vertices: tables::DROPSHOT_VERTICES.into(),
    };

    let collision_mesh = initialize_dropshot(&dropshot);

    let ball = Ball::initialize_dropshot();

    let gravity = vec3a(0., 0., -650.);

    Game {
        gravity,
        collision_mesh,
        ball,
    }
}

pub fn load_soccar_throwback() -> Game {
    println!("WARNING: THIS MAP IS KNOWN TO CAUSE EXTREME LAG WHEN GENERATING THE BALL PREDICTION STRUCT.");

    let back_ramps_lower = Mesh {
        ids: tables::THROWBACK_BACK_RAMPS_LOWER_IDS.into(),
        vertices: tables::THROWBACK_BACK_RAMPS_LOWER_VERTICES.into(),
    };
    let back_ramps_upper = Mesh {
        ids: tables::THROWBACK_BACK_RAMPS_UPPER_IDS.into(),
        vertices: tables::THROWBACK_BACK_RAMPS_UPPER_VERTICES.into(),
    };
    let corner_ramps_lower = Mesh {
        ids: tables::THROWBACK_CORNER_RAMPS_LOWER_IDS.into(),
        vertices: tables::THROWBACK_CORNER_RAMPS_LOWER_VERTICES.into(),
    };
    let corner_ramps_upper = Mesh {
        ids: tables::THROWBACK_CORNER_RAMPS_UPPER_IDS.into(),
        vertices: tables::THROWBACK_CORNER_RAMPS_UPPER_VERTICES.into(),
    };
    let corner_wall_0 = Mesh {
        ids: tables::THROWBACK_CORNER_WALL_0_IDS.into(),
        vertices: tables::THROWBACK_CORNER_WALL_0_VERTICES.into(),
    };
    let corner_wall_1 = Mesh {
        ids: tables::THROWBACK_CORNER_WALL_1_IDS.into(),
        vertices: tables::THROWBACK_CORNER_WALL_1_VERTICES.into(),
    };
    let corner_wall_2 = Mesh {
        ids: tables::THROWBACK_CORNER_WALL_2_IDS.into(),
        vertices: tables::THROWBACK_CORNER_WALL_2_VERTICES.into(),
    };
    let goal = Mesh {
        ids: tables::THROWBACK_GOAL_IDS.into(),
        vertices: tables::THROWBACK_GOAL_VERTICES.into(),
    };
    let side_ramps_lower = Mesh {
        ids: tables::THROWBACK_SIDE_RAMPS_LOWER_IDS.into(),
        vertices: tables::THROWBACK_SIDE_RAMPS_LOWER_VERTICES.into(),
    };
    let side_ramps_upper = Mesh {
        ids: tables::THROWBACK_SIDE_RAMPS_UPPER_IDS.into(),
        vertices: tables::THROWBACK_SIDE_RAMPS_UPPER_VERTICES.into(),
    };

    let params = InitializeThrowbackParams {
        back_ramps_lower: &back_ramps_lower,
        back_ramps_upper: &back_ramps_upper,
        corner_ramps_lower: &corner_ramps_lower,
        corner_ramps_upper: &corner_ramps_upper,
        corner_wall_0: &corner_wall_0,
        corner_wall_1: &corner_wall_1,
        corner_wall_2: &corner_wall_2,
        goal: &goal,
        side_ramps_lower: &side_ramps_lower,
        side_ramps_upper: &side_ramps_upper,
    };
    let collision_mesh = initialize_throwback(params);

    let ball = Ball::initialize_soccar();

    let gravity = vec3a(0., 0., -650.);

    Game {
        gravity,
        collision_mesh,
        ball,
    }
}
