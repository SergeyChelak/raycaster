pub enum ControlEvent {
    Keyboard(i32, bool), // key code | is pressed
}

#[derive(Default)]
pub struct ControllerState {
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub rotate_left_pressed: bool,
    pub rotate_right_pressed: bool,
}

impl ControllerState {
    const KEYCODE_W: i32 = 119;
    const KEYCODE_S: i32 = 115;
    const KEYCODE_A: i32 = 97;
    const KEYCODE_D: i32 = 100;
    const KEYCODE_LEFT: i32 = 1073741904;
    const KEYCODE_RIGHT: i32 = 1073741903;
    // up = 1073741906
    // down = 1073741905

    pub fn on_key_event(&mut self, key_code: i32, is_pressed: bool) {
        match key_code {
            Self::KEYCODE_W => self.up_pressed = is_pressed,
            Self::KEYCODE_S => self.down_pressed = is_pressed,
            Self::KEYCODE_A => self.left_pressed = is_pressed,
            Self::KEYCODE_D => self.right_pressed = is_pressed,
            Self::KEYCODE_LEFT => self.rotate_left_pressed = is_pressed,
            Self::KEYCODE_RIGHT => self.rotate_right_pressed = is_pressed,
            _ => {
                // don't care
                // println!("Code {key_code}")
            }
        }
    }
}
