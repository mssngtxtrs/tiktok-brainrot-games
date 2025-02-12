//----IMPORTS----
//
use macroquad::prelude::*;


//----CONSTANTS----
//
const DELAY: f32 = 0.3;
const GAME_NAMES: [&str; 3] = ["BALL EATER", "(WIP) Bouncing", "(WIP) Bouncing, reverse"];
/*----GAME LIST----
1. BALL EATER (Complete, debugging)
    Player ball eats other balls
2. Bouncing (WIP)
    Player ball trying to escape from several semicircles
3. Bouncing, reverse (WIP)
 */


//----STRUCTURES----
//
//--MainMenu--
pub struct MainMenu {
    //--Public variables--
    pub current: u8,

    //--Private variables--
    input_delay: f32
}

//----STRUCTURES' FUNCTIONS----
//
//--Main menu--
impl MainMenu {
    //----INITIAL----
    //
    pub fn new() -> Self {
        Self {
            current: 0,
            input_delay: 0.
        }
    }


    //----INPUT HANDLE----
    //
    pub fn input_handle(&mut self) -> bool {
        if is_key_down(KeyCode::Left) && self.current != 0 && self.input_delay <= 0. {
            self.current -= 1;
            self.input_delay = DELAY;
        }
        if is_key_down(KeyCode::Right) && self.current < 2 && self.input_delay <= 0. {
            self.current += 1;
            self.input_delay = DELAY;
        }
        if is_key_down(KeyCode::Enter) {
            return true
        }
        false
    }


    //----UPDATE----
    //
    pub fn update(&mut self) {
        //Decreasing input delay if exists
        if self.input_delay > 0. {
            self.input_delay -= get_frame_time();
        }
    }


    //----RENDER----
    //
    pub fn render(&mut self) {
        draw_text(GAME_NAMES[self.current as usize], 25., screen_height() / 2. - 80., 80., Color::new(1., 1., 0., 1.));
        draw_text("In game: Press Esc to return to menu", 25., screen_height() - 25., 48., Color::new(0.7, 0.7, 0.7, 1.));
        draw_text("Press Enter to continue", 25., screen_height() - 85., 48., Color::new(0.7, 0.7, 0.7, 1.));
    }


    //----PRIVATE FUNCTIONS----
    //
}