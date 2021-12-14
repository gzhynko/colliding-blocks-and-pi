use nannou::prelude::*;

const STROKE_WEIGHT: f32 = 3.;
const SMALL_BLOCK_SIZE: f32 = 40.;
const LARGE_BLOCK_SIZE: f32 = 100.;

const WALL_X: f32 = -500.;
const FLOOR_Y: f32 = -300.;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    small_block_x: f32,
    small_block_velocity: f32,
    large_block_x: f32,
    large_block_mass_factor: i64,
    large_block_velocity: f32,
    collision_count: i32,
}

fn model(_app: &App) -> Model {
    Model {
        small_block_x: 0.,
        small_block_velocity: 0.,
        large_block_x: 2. * LARGE_BLOCK_SIZE,
        large_block_mass_factor: 6,
        large_block_velocity: -60.,
        collision_count: 0,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let time_sec = update.since_last.as_secs_f32();

    let time_factor = if model.large_block_x - LARGE_BLOCK_SIZE / 2. - WALL_X > SMALL_BLOCK_SIZE { 1 } else { model.large_block_mass_factor };

    let large_block_mass = pow(10, model.large_block_mass_factor as usize);

    // the for loop is here to do the calculations faster by doing them multiple times (the time_factor var) per frame update
    for _i in 0..time_factor {
        model.small_block_x += model.small_block_velocity * time_sec;
        model.large_block_x += model.large_block_velocity * time_sec;

        let large_block_left_x = model.large_block_x - LARGE_BLOCK_SIZE / 2.;
        let small_block_left_x = model.small_block_x - SMALL_BLOCK_SIZE / 2.;
        let small_block_right_x = model.small_block_x + SMALL_BLOCK_SIZE / 2.;

        let masses_sum = (1 + large_block_mass) as f32;

        // check if blocks collide
        if large_block_left_x <= small_block_right_x || small_block_right_x >= large_block_left_x {
            let small_block_new_vel = ((1. - large_block_mass as f32) / masses_sum * model.small_block_velocity) + ((2. * large_block_mass as f32) / masses_sum * model.large_block_velocity);
            let large_block_new_vel = ((large_block_mass as f32 - 1.) / masses_sum * model.large_block_velocity) + ((2. * 1.) / masses_sum * model.small_block_velocity);

            model.small_block_velocity = small_block_new_vel;
            model.large_block_velocity = large_block_new_vel;

            model.collision_count += 1;
        }

        // reverse the small block speed if it collides with the wall
        if small_block_left_x <= WALL_X {
            model.small_block_velocity = -model.small_block_velocity;
            model.collision_count += 1;
        }

        // correct positions if one of the blocks clipped into something
        if small_block_right_x > large_block_left_x {
            let diff = small_block_right_x - large_block_left_x;
            model.small_block_x -= diff;
        }
        if small_block_left_x < WALL_X {
            let diff = WALL_X - small_block_left_x;
            model.small_block_x += diff;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    frame.clear(WHITE);

    let win_rect = app.window_rect();
    setup_scene(&draw, &model, &win_rect);

    draw.to_frame(app, &frame).unwrap();
}

fn setup_scene(draw: &Draw, model: &Model, win_rect: &Rect) {
    let win_padding = win_rect.pad(10.);

    let mut small_block_x = model.small_block_x;
    let mut large_block_x = model.large_block_x;

    // pause the blocks so that they don't clip into the wall
    if model.large_block_x - LARGE_BLOCK_SIZE / 2. - WALL_X < SMALL_BLOCK_SIZE {
        small_block_x = WALL_X + SMALL_BLOCK_SIZE / 2.;
        large_block_x = WALL_X + SMALL_BLOCK_SIZE + LARGE_BLOCK_SIZE / 2.;
    }

    // floor
    draw.line()
        .start(vec2(win_padding.mid_left().x, FLOOR_Y))
        .end(vec2(win_padding.mid_right().x, FLOOR_Y))
        .weight(STROKE_WEIGHT)
        .finish();

    // wall
    draw.line()
        .start(vec2(WALL_X, win_padding.mid_top().y))
        .end(vec2(WALL_X, win_padding.mid_bottom().y))
        .weight(STROKE_WEIGHT)
        .finish();

    // small block
    draw.rect()
        .wh(vec2(SMALL_BLOCK_SIZE, SMALL_BLOCK_SIZE))
        .no_fill()
        .stroke_weight(STROKE_WEIGHT)
        .xy(vec2(small_block_x, FLOOR_Y + SMALL_BLOCK_SIZE / 2.))
        .finish();

    // large block
    draw.rect()
        .wh(vec2(LARGE_BLOCK_SIZE, LARGE_BLOCK_SIZE))
        .no_fill()
        .stroke_weight(STROKE_WEIGHT)
        .xy(vec2(large_block_x, FLOOR_Y + LARGE_BLOCK_SIZE / 2.))
        .finish();

    // collision counter
    draw.text(format!("Collisions: {}", model.collision_count).as_str())
        .xy(win_padding.top_right() - vec2(100., 0.))
        .font_size(20)
        .left_justify()
        .color(BLACK)
        .finish();
}
