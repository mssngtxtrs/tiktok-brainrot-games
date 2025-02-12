//----IMPORTS----
//
use macroquad::prelude::*;
use ::rand::{Rng, rng};

//----CONSTANTS----
//
const BALLS_SPEED: f32 = 0.6; //hehe balls hehehe
const BALLS_RADIUS: f32 = 8.;
const PLAYER_SPEED: f32 = 1.2;
const PLAYER_RADIUS: f32 = 20.;
const WORLD_RADIUS: f32 = 250.;
const BORDER_RADIUS: f32 = 2.;


//----STRUCTURES----
//
//--Player--
struct Player {
    circle: Circle,
    direction: Vec2
}

//--Ball--
struct Ball {
    circle: Circle,
    direction: Vec2,
    collided: bool,
    color: Color
}

//--Game1--
pub struct Game1 {
    player: Player,
    balls: Vec<Ball>,
    world: Circle,
    color_delta: f32
}


//----STRUCTURES' FUNCTIONS----
//
//--Game1--
impl Game1 {
    //----INITIAL----
    //
    pub fn new() -> Self {
        let mut rng = rng();
        let mut game = Self{
            player: Player::new(vec2(rng.random_range(-4. .. 4.), rng.random_range(-2. .. 2.))),
            balls: vec!(),
            world: Circle::new(screen_width() / 2., screen_height() / 2., WORLD_RADIUS),
            color_delta: 0.
        };

        game.balls.push(Ball::new(vec2(rng.random_range(-1. .. 1.), rng.random_range(-1. .. 1.)), 0.));

        game
    }


    //----INPUT HANDLE----
    //
    pub fn input_handle(&mut self) -> bool {
        if is_key_down(KeyCode::Escape) { return true };
        false
    }


    //----UPDATE----
    //
    pub fn update(&mut self) {
        //Collision variables
        let mut collision = false;
        let mut balls_spawn = 1;

        //Moving player
        self.player.movement(&self.world);

        //Remember balls amount
        let balls_amount = self.balls.len();

        //Moving balls, checking for collision with player
        for ball in &mut self.balls {
            ball.movement();
            if !ball.collide(&self.world) {
                ball.collided = true;
                if balls_amount == 1 {
                    collision = true;
                }
            }
            if ball.collide(&self.player.circle) {
                ball.collided = true;
                collision = true;
                balls_spawn += 1;
            }
        }

        //Deleting collided balls
        self.balls.retain(|ball| !ball.collided);

        //Creating 2 new balls onm collision
        if collision {
            for _ in 0 .. balls_spawn {
                self.color_delta = (self.color_delta + 0.05) % 1.;
                let mut rng = rng();
                self.balls.push(Ball::new(vec2(rng.random_range(-1. .. 1.), rng.random_range(-1. .. 1.)), self.color_delta));
            }
        }

    }


    //----RENDER----
    //
    pub fn render(&mut self) {
        //Render world borders
        draw_circle(self.world.x, self.world.y, self.world.r + BORDER_RADIUS, WHITE);

        //Render world background
        draw_circle(self.world.x, self.world.y, self.world.r, BLACK);

        //Render player
        self.player.draw();

        //Render balls
        for ball in &self.balls {
            ball.draw();
        }
    }
}



//--Player--
impl Player {
    //--New player--
    fn new(direction: Vec2) -> Self {
        Self {
            circle: Circle::new(screen_width() / 2., screen_height() / 2. + 30., PLAYER_RADIUS),
            direction
        }
    }

    //--Moving--
    fn movement(&mut self, world: &Circle) {
        //Adding gravity
        self.direction += vec2(0. * get_frame_time(), 4.9 * get_frame_time());

        //Actually moving
        if !Circle::new(world.x, world.y, world.r - self.circle.r * 2.).overlaps(&(self.circle)) {    /*If out of bounds*/
            //Finding collision normal (in this case: player position)
            let normal = -self.circle.point();

            //Mirror
            self.direction = normal.normalize_or_zero().rotate(self.direction);

            //Moving to a new direction
            self.circle.move_to(self.circle.point() + self.direction);
        } else {    /*if inbounds*/
            self.circle.move_to(self.circle.point() + self.direction);
        }
    }

    //--Drawing--
    fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, WHITE);
    }
}

//--Ball--
impl Ball {
    //--New ball--
    fn new(direction: Vec2, color_delta: f32) -> Self {
        Self {
            circle: Circle::new(screen_width() / 2., screen_height() / 2., BALLS_RADIUS),
            direction,
            collided: false,
            color: Color::new(1., 1. - color_delta, color_delta, 1.)
        }
    }

    //--Moving--
    fn movement(&mut self) {
        self.circle.move_to(self.circle.point() + self.direction);
    }

    //--Collision check--
    fn collide(&self, collider: &Circle) -> bool {
        collider.overlaps(&self.circle)
    }

    //--Drawing--
    fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, self.color);
    }
}