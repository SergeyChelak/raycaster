pub enum ControlEvent {
    Keyboard {
        key_code: i32,
        is_pressed: bool,
    },
    MouseMotion {
        x: i32,
        y: i32,
        x_rel: i32,
        y_rel: i32,
    },
}

#[derive(Default)]
pub struct ControllerState {
    pub forward_pressed: bool,
    pub backward_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub rotate_left_pressed: bool,
    pub rotate_right_pressed: bool,
    pub mouse_x_relative: i32,
    pub minimap_visible: bool,
}

impl ControllerState {
    const KEYCODE_W: i32 = 119;
    const KEYCODE_S: i32 = 115;
    const KEYCODE_A: i32 = 97;
    const KEYCODE_D: i32 = 100;
    const KEYCODE_LEFT: i32 = 1073741904;
    const KEYCODE_RIGHT: i32 = 1073741903;
    const KEYCODE_UP: i32 = 1073741906;
    const KEYCODE_DOWN: i32 = 1073741905;
    const KEYCODE_F2: i32 = 1073741883;

    pub fn on_key_event(&mut self, key_code: i32, is_pressed: bool) {
        match key_code {
            Self::KEYCODE_UP | Self::KEYCODE_W => self.forward_pressed = is_pressed,
            Self::KEYCODE_DOWN | Self::KEYCODE_S => self.backward_pressed = is_pressed,
            Self::KEYCODE_A => self.left_pressed = is_pressed,
            Self::KEYCODE_D => self.right_pressed = is_pressed,
            Self::KEYCODE_LEFT => self.rotate_left_pressed = is_pressed,
            Self::KEYCODE_RIGHT => self.rotate_right_pressed = is_pressed,
            Self::KEYCODE_F2 if is_pressed => self.minimap_visible = !self.minimap_visible,
            _ => {
                // don't care
                // println!("Code {key_code}")
            }
        }
    }

    pub fn reset_relative_values(&mut self) {
        self.mouse_x_relative = 0;
    }
}
