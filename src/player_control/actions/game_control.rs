use bevy::prelude::{Input, Res};
use leafwing_input_manager::{Actionlike, scan_codes::QwertyScanCode};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
    Sprint,
    Jump,
    ToggleEditor,
    Interact,
}

macro_rules! generate_bindings {
    ( $( $game_control:pat => $key_codes:expr, )+ ) => {
        impl GameControl {
            #[allow(dead_code)]
            pub fn just_released(&self, keyboard_input: &Res<Input<QwertyScanCode>>) -> bool {
                match self {
                    $ (
                        $game_control => keyboard_input.any_just_released($key_codes),
                    )+
                }
            }

            #[allow(dead_code)]
            pub fn just_pressed(&self, keyboard_input: &Res<Input<QwertyScanCode>>) -> bool {
                match self {
                    $ (
                        $game_control => keyboard_input.any_just_pressed($key_codes),
                    )+
                }
            }

            pub fn pressed(&self, keyboard_input: &Res<Input<QwertyScanCode>>) -> bool {
                match self {
                    $ (
                        $game_control => keyboard_input.any_pressed($key_codes),
                    )+
                }
            }
        }
    };
    ( $( $game_control:pat => $key_codes:expr ),+ ) => {
        generate_bindings!($($game_control => $key_codes,)+);
    };
}

pub fn get_movement(control: GameControl, input: &Res<Input<QwertyScanCode>>) -> f32 {
    if control.pressed(input) {
        1.0
    } else {
        0.0
    }
}

// MacOS: sampled by hand
// Windows: <https://superuser.com/a/1454198>
// Linux: <http://www.quadibloc.com/comp/scan.htm>
generate_bindings! {
    GameControl::Up => [
         // W
        QwertyScanCode::W,
        // Up arrow
        QwertyScanCode::Up,
    ],
    GameControl::Down => [
        // S
        QwertyScanCode::S,
        // Down arrow
        QwertyScanCode::Down,
    ],
    GameControl::Left => [
        // A
        QwertyScanCode::A,
        // Left arrow
        QwertyScanCode::Left,
    ],
    GameControl::Right => [
        // D
        QwertyScanCode::D,
        // Right arrow
        QwertyScanCode::Right,
    ],
    GameControl::Sprint => [
        // Left shift
        QwertyScanCode::LShift,
    ],
    GameControl::Jump => [
        // Space
        QwertyScanCode::Space
    ],
    GameControl::ToggleEditor => [
        // Q
        QwertyScanCode::Q,
    ],
    GameControl::Interact => [
        // E
        QwertyScanCode::E,
    ],
}
