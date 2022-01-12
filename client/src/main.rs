use bevy::{
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    prelude::*,
};

mod networking;
use networking::*;

use bootleg_networking::NetworkingPlugin;

use wasm_timer::SystemTime;

fn main() {
    let startup_time = SystemTime::now();

    App::new()
        .insert_resource(WindowDescriptor {
            width: 300.,
            height: 300.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(NetworkingPlugin)
        .insert_resource(startup_time)
        .add_startup_system(setup)
        .add_system(send)
        .add_system(receive)
        // One time greet
        .add_startup_system(hello_wasm_system)
        // Track ticks (sanity check, whether game loop is running)
        .add_system(counter)
        // Track input events
        .add_system(track_input_events)
        .run();
}

fn hello_wasm_system() {
    info!("hello wasm");
}

fn counter(mut state: Local<CounterState>, time: Res<Time>) {
    if state.count % 60 == 0 {
        info!(
            "tick {} @ {:?} [Δ{}]",
            state.count,
            time.time_since_startup(),
            time.delta_seconds()
        );
    }
    state.count += 1;
}

#[derive(Default)]
struct CounterState {
    count: u32,
}

use bootleg_networking::NetworkResource;
fn track_input_events(
    mut net: ResMut<NetworkResource>,
    mut ev_keys: EventReader<KeyboardInput>,
    mut ev_cursor: EventReader<CursorMoved>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_mousebtn: EventReader<MouseButtonInput>,
    mut ev_scroll: EventReader<MouseWheel>,
) {
    // Keyboard input
    for ev in ev_keys.iter() {
        if ev.state.is_pressed() {
            info!("Just pressed key: {:?}", ev.key_code);
            let message = include_str!("../textfile.txt").to_string();
            net.broadcast_message(&message, &networking::MESSAGE_CHANNEL_ID)
                .unwrap();
        } else {
            info!("Just released key: {:?}", ev.key_code);
        }
    }

    // Absolute cursor position (in window coordinates)
    for ev in ev_cursor.iter() {
        info!("Cursor at: {}", ev.position);
    }

    // Relative mouse motion
    for ev in ev_motion.iter() {
        info!("Mouse moved {} pixels", ev.delta);
    }

    // Mouse buttons
    for ev in ev_mousebtn.iter() {
        if ev.state.is_pressed() {
            info!("Just pressed mouse button: {:?}", ev.button);
        } else {
            info!("Just released mouse button: {:?}", ev.button);
        }
    }

    // scrolling (mouse wheel, touchpad, etc.)
    for ev in ev_scroll.iter() {
        info!(
            "Scrolled vertically by {} and horizontally by {}.",
            ev.y, ev.x
        );
    }
}
