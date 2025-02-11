//----IMPORTS----
//
use macroquad::prelude::*;
use ::rand::{Rng, rng};

//----CONSTANTS----
//
const BALLS_SPEED: f32 = 0.6; //hehe balls hehehe
const PLAYER_SPEED: f32 = 1.2;
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
    collided: bool
}

//--Game1--
pub struct Game1 {
    player: Player,
    balls: Vec<Ball>,
    world: Circle
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
            world: Circle::new(screen_width() / 2., screen_height() / 2., WORLD_RADIUS)
        };

        game.balls.push(Ball::new(vec2(rng.random_range(-1. .. 1.), rng.random_range(-1. .. 1.))));

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
        //Collision variable
        let mut collision = false;

        //Moving player
        self.player.movement(&self.world);

        //Moving balls, checking for collision with player
        for ball in &mut self.balls {
            ball.movement();
            if ball.collide(&self.player) {
                ball.collided = true;
                collision = true;
            }
        }

        //Deleting collided balls
        self.balls.retain(|ball| ball.collided);

        //Creating 2 new balls onm collision
        if collision {
            for _ in 0 .. 2 {
                let mut rng = rng();
                self.balls.push(Ball::new(vec2(rng.random_range(-1. .. 1.), rng.random_range(-1. .. 1.))));
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
            circle: Circle::new(screen_width() / 2., screen_height() / 2. + 30., 10.),
            direction
        }
    }

    //--Moving--
    fn movement(&mut self, world: &Circle) {
        //Adding gravity
        self.direction += vec2(0. * get_frame_time(), 4.9 * get_frame_time());

        //Actually moving
        if world.contains(&(self.circle.point() + self.direction)) {      //If inbounds
            self.circle.move_to((self.circle.point() + self.direction));
        } else {                                                               //If not
            //Finding collision normal (in this case: player position)
            let normal = self.circle.point() - vec2(screen_width() / 2., screen_height() / 2.);

            self.direction = -self.direction * normal.normalize_or_zero() * PLAYER_SPEED;

            //Moving to a new direction
            self.circle.move_to(self.circle.point() + self.direction);
        }
        //Changing direction on colliding with borders
    }

    //--Drawing--
    fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, WHITE);
    }
}

//--Ball--
impl Ball {
    //--New ball--
    fn new(direction: Vec2) -> Self {
        Self {
            circle: Circle::new(screen_width() / 2., screen_height() / 2., 4.),
            direction,
            collided: false
        }
    }

    //--Moving--
    fn movement(&mut self) {
        self.circle.move_to(self.circle.point() * self.direction);
    }

    //--Collision check--
    fn collide(&self, player: &Player) -> bool {
        player.circle.overlaps(&self.circle)
    }

    //--Drawing--
    fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, YELLOW);
    }
}