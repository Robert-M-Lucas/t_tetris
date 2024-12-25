/*

use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::*;
use bevy_dev_console::builtin_parser::Environment;
use bevy_dev_console::register;
use std::time::Duration;

fn echo(string: String) {
    info!("Echo: {string}");
}

fn fps(world: &mut World) {
    world.run_system_once(|time: Res<Time>| {
        info!(
            "Frametime: {:?} [{} fps]",
            time.delta(),
            1.0 / time.delta().as_secs_f32()
        );
    });
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum CountingFrames {
    #[default]
    NotCounting,
    Counting,
}

#[derive(Resource, Default)]
struct FrameCount {
    pub end: f32,
    pub count: usize,
    pub worst: f32,
}

fn fps_avg(world: &mut World) {
    info!("Starting fps profiling");
    world.run_system_once(
        |mut is_counting: ResMut<NextState<CountingFrames>>,
         mut count: ResMut<FrameCount>,
         time: Res<Time>| {
            is_counting.set(CountingFrames::Counting);
            count.count = 1;
            count.end = time.elapsed().as_secs_f32() + 10.0;
            count.worst = f32::INFINITY;
        },
    );
}

fn count_frames(
    mut count: ResMut<FrameCount>,
    mut is_counting: ResMut<NextState<CountingFrames>>,
    time: Res<Time>,
) {
    if time.elapsed().as_secs_f32() > count.end {
        let fps = count.count as f32 / 10.0;
        info!(
            "Avg FPS (10 seconds): {} | Worst frametime: {} [{} fps]",
            fps,
            count.worst,
            1.0 / count.worst
        );

        is_counting.set(CountingFrames::NotCounting);
    } else {
        count.count += 1;
        if time.delta().as_secs_f32() < count.worst {
            count.worst = time.delta().as_secs_f32();
        }
    }
}

pub fn dev_console_environment(app: &mut App) -> Environment {
    app.insert_resource(FrameCount::default());
    app.init_state::<CountingFrames>();
    app.add_systems(
        Update,
        (count_frames).run_if(in_state(CountingFrames::Counting)),
    );

    let mut environment = Environment::default();

    register!(&mut environment => {
        fn echo;
        fn fps;
        fn fps_avg;
    });

    environment
}

 */
