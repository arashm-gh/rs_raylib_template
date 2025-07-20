use raylib::prelude::*;

use super::{
    Entity, EntityType, GAME_FPS, SET_FULLSCREEN, WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH,
};

pub enum GameState {
    Init,
    Update,
    Quit,
}

pub struct Game {
    running: bool,
    gs: GameState,
    rl: RaylibHandle,
    thread: RaylibThread,
    entities: Vec<Entity>,
}

impl Game {
    pub fn new() -> Self {
        let (mut rl, thread) = raylib::init()
            .size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .title(WINDOW_TITLE)
            .build();
        if SET_FULLSCREEN {
            rl.toggle_fullscreen();
        }
        rl.set_target_fps(GAME_FPS);
        Game {
            running: true,
            gs: GameState::Init,
            rl,
            thread,
            entities: vec![],
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.input();
            self.update();
            self.draw();
        }
    }

    fn input(&mut self) {
        if self.rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            self.gs = GameState::Quit;
        }
    }

    fn update(&mut self) {
        match self.gs {
            GameState::Init => self.gs = self.init(),
            GameState::Update => self.gs = self.update_game(),
            GameState::Quit => self.running = false,
        }
    }

    fn draw(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::GRAY);
    }
}

impl Game {
    fn init(&mut self) -> GameState {
        self.entities
            .push(Entity::new(Vector2::zero(), EntityType::Wall));
        GameState::Update
    }

    fn update_game(&mut self) -> GameState {
        GameState::Update
    }
}
