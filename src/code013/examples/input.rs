#![allow(unused_variables)]

use bevy::input::gamepad::{GamepadSettings, AxisSettings, ButtonSettings};
use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseButtonInput, MouseWheel};
use bevy::input::keyboard::KeyboardInput;

// ANCHOR: keyboard-input
fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        // Space was pressed
    }
    if keys.just_released(KeyCode::LControl) {
        // Left Ctrl was released
    }
    if keys.pressed(KeyCode::W) {
        // W is being held down
    }
    // we can check multiple at once with `.any_*`
    if keys.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
        // Either the left or right shift are being held down
    }
    if keys.any_just_pressed([KeyCode::Delete, KeyCode::Back]) {
        // Either delete or backspace was just pressed
    }
}
// ANCHOR_END: keyboard-input

// ANCHOR: keyboard-events
fn keyboard_events(
    mut key_evr: EventReader<KeyboardInput>,
) {
    use bevy::input::ButtonState;

    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Key press: {:?} ({})", ev.key_code, ev.scan_code);
            }
            ButtonState::Released => {
                println!("Key release: {:?} ({})", ev.key_code, ev.scan_code);
            }
        }
    }
}
// ANCHOR_END: keyboard-events

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(configure_gamepads)
        .add_system(file_drop)
        .add_system(keyboard_input)
        .add_system(keyboard_events)
        .add_system(gamepad_connections)
        .add_system(gamepad_input)
        .add_system(gamepad_input_events)
        .add_system(gamepad_print_allevents)
        .add_system(touches)
        .add_system(touch_events)
        .run();
}
