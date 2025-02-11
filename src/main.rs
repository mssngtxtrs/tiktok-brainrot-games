#![windows_subsystem = "windows"]


//----OTHER SCRIPTS----
//
mod menu;
mod game_1;


//----IMPORTS----
//
use macroquad::prelude::*;
use crate::menu::*;
use crate::game_1::*;


//----CONSTANTS----
//


//----STRUCTURES / ENUMERATIONS----
//
enum Window{
    MainMenu,
    FirstGame
}


//--GAME WINDOW CONFIGURATION--
fn conf() -> Conf {
    Conf {
        window_title: "TikTok brainrot games".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        high_dpi: true,
        fullscreen: false,
        sample_count: 1,
        ..Default::default()
    }
}

//----MAIN FUNCTION----
//
#[macroquad::main(conf)]
async fn main() {
    //Setting up resource folder
    set_pc_assets_folder("resources");


    //----VARIABLES----
    //
    //--Current window--
    let mut current_window = Window::MainMenu;

    //--Main menu--
    let mut menu = MainMenu::new();

    //--First game--
    let mut game1 = Game1::new();



    //----START----
    //


    //----MAIN LOOP----
    //
    loop {
        //Clearing background
        clear_background(BLACK);


        //----INPUT HANDLE----
        //
        match current_window {
            Window::MainMenu => { match menu.input_handle() {
                    true => { match menu.current {
                            0 => { current_window = Window::FirstGame; }
                            _ => {}
                        }
                    }
                    _ => {}
                };
            }
            Window::FirstGame => { match game1.input_handle() {
                    true => {
                        current_window = Window::MainMenu;
                        game1 = Game1::new();
                    }
                    _ => {}
                }
            }
        }


        //----UPDATE----
        //
        match current_window {
            Window::MainMenu => { menu.update(); }
            Window::FirstGame => { game1.update(); }
        }

        //----RENDER----
        //
        match current_window {
            Window::MainMenu => { menu.render(); }
            Window::FirstGame => { game1.render(); }
        }


        //Waiting for next frame
        next_frame().await;
    }
}