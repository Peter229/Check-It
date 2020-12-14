#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

#[cfg(not(target_arch = "wasm32"))]
extern crate rodio;

extern crate instant;

mod audio;
mod song;

use quicksilver::{
    geom::{Rectangle, Vector, Transform},
    graphics::{Color, Image, VectorFont},
    input::{Event, Key, MouseButton, KeyboardEvent},
    run, Graphics, Input, Result, Settings, Timer, Window,
};

use std::collections::HashMap;
use instant::Instant;

use std::fs::File;
use std::io::prelude::*;

fn main() {

    run(
        Settings {
            size: Vector::new(1024.0, 768.0),
            vsync: false, 
            title: "Hit It",       
            ..Settings::default()
        },
        app,
    );
}

const TIME_OFFSET: i32 = 200;

enum songs {

    xepher,
    karin,
}

enum main_menu_state {

    play,
    settings,
    exit,
}

pub enum game_state {

    web_load,
    web_wait_start,
    init_boot,
    boot,
    splash_screen,
    main_menu,
    song_select,
    load_song,
    song_game_screen_start,
    song_game_screen_intro,
    song_game_screen_play,
    song_finish,
    exit,
}

pub struct Game {

    pub game_state: game_state,
    pub images: HashMap<String, Image>,
    audio: audio::Audio,
    main_menu_state: main_menu_state,
    song: songs,
    timer: Instant,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    songs: HashMap<String, song::Song>,
    score: i32,
    current_song: String,
}

impl Game {

    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {

        let audio = audio::Audio::new();
        let images: HashMap<String, Image> = HashMap::new();
        let timer = Instant::now();
        let mut songs: HashMap<String, song::Song> = HashMap::new();
        let current_song = "karin".to_string();
        songs.insert("karin".to_string(), song::Song::new());
        songs.insert("xepher".to_string(), song::Song::new());
        Self {
            game_state: game_state::init_boot,
            images,
            audio,
            main_menu_state: main_menu_state::play,
            song: songs::karin,
            timer,
            left: false,
            right: false,
            up: false,
            down: false,
            songs,
            score: 0,
            current_song,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Self {

        let audio = audio::Audio::new();
        let images: HashMap<String, Image> = HashMap::new();
        let timer = Instant::now();
        let mut songs: HashMap<String, song::Song> = HashMap::new();
        let current_song = "karin".to_string();
        songs.insert("karin".to_string(), song::Song::new());
        songs.insert("xepher".to_string(), song::Song::new());
        Self {
            game_state: game_state::web_load,
            images,
            audio,
            main_menu_state: main_menu_state::play,
            song: songs::karin,
            timer,
            left: false,
            right: false,
            up: false,
            down: false,
            songs,
            score: 0,
            current_song,
        }
    }

    pub fn input(&mut self, key: KeyboardEvent) {

        match self.game_state {
            game_state::web_wait_start => {
                self.input_web_wait_start(key);
            },
            game_state::splash_screen => {
                self.input_splash_screen(key);
            },
            game_state::main_menu => {
                self.input_main_menu(key);
            },
            game_state::song_select => {
                self.input_song_select(key);
            },
            game_state::song_game_screen_play => {
                self.input_song_game_screen_play(key);
            },
            game_state::song_finish => {

            },
            _ => (),
        }

    }

    pub fn update(&mut self) {

        match self.game_state {
            game_state::web_wait_start => {
                self.update_web_wait_start();
            },
            game_state::splash_screen => {
                self.update_splash_screen();
            },
            game_state::main_menu => {

            },
            game_state::song_select => {

            },
            game_state::song_game_screen_intro => {

            }
            game_state::song_game_screen_play => {

            },
            game_state::song_finish => {

            },
            _ => (),
        }

    }

    async fn render(&mut self, gfx: &mut Graphics) {

        match self.game_state {
            game_state::web_load => {
                self.render_web_load(gfx).await;
            }
            game_state::web_wait_start => {
                self.render_web_wait_start(gfx);
            },
            game_state::init_boot => {
                self.render_init_boot(gfx).await;
            }
            game_state::boot => {
                self.render_boot(gfx).await;
            },
            game_state::splash_screen => {
                self.render_splash_screen(gfx);
            },
            game_state::main_menu => {
                self.render_main_menu(gfx);
            },
            game_state::song_select => {
                self.render_song_select(gfx);
            },
            game_state::load_song => {
                self.render_load_song().await;
            },
            game_state::song_game_screen_start => {
                self.render_song_game_screen_start().await;
            },
            game_state::song_game_screen_intro => {
                self.render_song_game_screen_intro(gfx);
            }
            game_state::song_game_screen_play => {
                self.render_game_screen_play(gfx);
            },
            game_state::song_finish => {
                self.render_song_finish().await;
            },
            _ => (),
        }
    }

    //Song finished
    async fn render_song_finish(&mut self) {

        /*let mut file = File::create("xepher.txt").unwrap();
        let mut file_contents: String = String::from("");
        for i in 0..(self.songs.get(&"xepher".to_string()).unwrap().keys.len()) {
            let (mut a, mut b) = self.songs.get(&"xepher".to_string()).unwrap().keys[i];
            file_contents.push_str(&a.to_string());
            file_contents.push_str(",");
            file_contents.push_str(&b.to_string());
            if i < self.songs.get(&"xepher".to_string()).unwrap().keys.len() - 1 {
                file_contents.push_str("\n");
            }
        }
        file.write_all(&file_contents.into_bytes()).unwrap();*/
        self.left = false;
        self.right = false;
        self.up = false;
        self.down = false;
        self.game_state = game_state::main_menu;
    }

    //While playing song
    //Calc score
    fn calulate_song_game_screen_play(&mut self, dir: i32) {

        let mut hit: bool = false;
        let temp_len = self.songs.get(&self.current_song).unwrap().keys.len() as i32;
        let temp_time = (self.timer.elapsed().as_millis() as i32) - TIME_OFFSET;
        self.songs.get_mut(&self.current_song).unwrap().keys.retain(|&(i, o)| i > temp_time);
        let temp_len_2 = self.songs.get(&self.current_song).unwrap().keys.len() as i32;
        self.score = self.score - (300 * (temp_len - temp_len_2));
        if self.songs.get(&self.current_song).unwrap().keys.len() == 0 {

            self.score = self.score - 300;
        } 
        else {
            for i in 0..(self.songs.get(&self.current_song).unwrap().keys.len()) {

                let (time, pos) = self.songs.get(&self.current_song).unwrap().keys[i];
                if time > (self.timer.elapsed().as_millis() as i32) + TIME_OFFSET {
                    //Early
                    self.score = self.score - 300;
                    break;
                }
                else if time > (self.timer.elapsed().as_millis() as i32) - TIME_OFFSET && time < (self.timer.elapsed().as_millis() as i32) + TIME_OFFSET {
                    //On time
                    if dir == pos {
                        self.score = self.score + 150;
                        self.songs.get_mut(&self.current_song).unwrap().keys.remove(i);
                        break;
                    }
                }
            }
        }
    }

    fn input_song_game_screen_play(&mut self, key: KeyboardEvent) {

        if key.key() == Key::D || key.key() == Key::Left {
            if key.is_down() {
                self.left = true;
                self.calulate_song_game_screen_play(song::LEFT);
                //self.songs.get_mut(&"xepher".to_string()).unwrap().keys.push((self.timer.elapsed().as_millis() as i32, song::LEFT));
            }
            else {
                self.left = false;
            }
        }
        else if key.key() == Key::F || key.key() == Key::Down {
            if key.is_down() {
                self.down = true;
                self.calulate_song_game_screen_play(song::DOWN);
                //self.songs.get_mut(&"xepher".to_string()).unwrap().keys.push((self.timer.elapsed().as_millis() as i32, song::DOWN));
            }
            else {
                self.down = false;
            }
        }
        else if key.key() == Key::J || key.key() == Key::Up {
            if key.is_down() {
                self.up = true;
                self.calulate_song_game_screen_play(song::UP);
                //self.songs.get_mut(&"xepher".to_string()).unwrap().keys.push((self.timer.elapsed().as_millis() as i32, song::UP));
            }
            else {
                self.up = false;
            }
        }
        else if key.key() == Key::K || key.key() == Key::Right {
            if key.is_down() {
                self.right = true;
                self.calulate_song_game_screen_play(song::RIGHT);
                //self.songs.get_mut(&"xepher".to_string()).unwrap().keys.push((self.timer.elapsed().as_millis() as i32, song::RIGHT));
            }
            else {
                self.right = false;
            }
        }
    }

    fn render_game_screen_play(&mut self, gfx: &mut Graphics) {

        if self.left {
            let region = Rectangle::new(Vector::new(50.0, 500.0), self.images.get(&"left_hud_selected".to_string()).unwrap().size());
            gfx.draw_image(&self.images.get(&"left_hud_selected".to_string()).unwrap(), region);
        }
        else {
            let region = Rectangle::new(Vector::new(50.0, 500.0), self.images.get(&"left_hud".to_string()).unwrap().size());
            gfx.draw_image(&self.images.get(&"left_hud".to_string()).unwrap(), region);
        }

        if self.down {
            let region = Rectangle::new(Vector::new(250.0, 500.0), self.images.get(&"down_hud_selected".to_string()).unwrap().size());
            gfx.draw_image(&self.images.get(&"down_hud_selected".to_string()).unwrap(), region);
        }
        else {
            let region = Rectangle::new(Vector::new(250.0, 500.0), self.images.get(&"down_hud".to_string()).unwrap().size());
            gfx.draw_image(&self.images.get(&"down_hud".to_string()).unwrap(), region);
        }

        if self.up {
            let region = Rectangle::new(Vector::new(450.0, 500.0), self.images.get(&"up_hud_selected".to_string()).unwrap().size());
            gfx.draw_image(&self.images.get(&"up_hud_selected".to_string()).unwrap(), region);
        }
        else {
            let region = Rectangle::new(Vector::new(450.0, 500.0), self.images.get(&"up_hud".to_string()).unwrap().size());
            gfx.draw_image(&self.images.get(&"up_hud".to_string()).unwrap(), region);
        }

        if self.right {
            let region = Rectangle::new(Vector::new(650.0, 500.0), self.images.get(&"right_hud_selected".to_string()).unwrap().size());
            gfx.draw_image(&self.images.get(&"right_hud_selected".to_string()).unwrap(), region);
        }
        else {
            let region = Rectangle::new(Vector::new(650.0, 500.0), self.images.get(&"right_hud".to_string()).unwrap().size());
            gfx.draw_image(&self.images.get(&"right_hud".to_string()).unwrap(), region);
        }

        let temp_len = self.songs.get(&self.current_song).unwrap().keys.len() as i32;
        let temp_time = (self.timer.elapsed().as_millis() as i32) - TIME_OFFSET;
        self.songs.get_mut(&self.current_song).unwrap().keys.retain(|&(i, o)| i > temp_time);
        let temp_len_2 = self.songs.get(&self.current_song).unwrap().keys.len() as i32;
        self.score = self.score - (300 * (temp_len - temp_len_2));

        let max_to_render: usize = 24;

        gfx.set_view(Transform::translate(Vector::new(0.0, (self.timer.elapsed().as_millis() as f32))));
        for i in 0..(max_to_render.min(self.songs.get(&self.current_song).unwrap().keys.len())) {
            let (time_in_millis, dir) = self.songs.get(&self.current_song).unwrap().keys[i];
            //println!("{}", time_in_millis);
            match dir {

                song::LEFT => {
                    let region = Rectangle::new(Vector::new(50.0, (500 - time_in_millis) as f32), self.images.get(&"left".to_string()).unwrap().size());
                    gfx.draw_image(&self.images.get(&"left".to_string()).unwrap(), region);
                },
                song::DOWN => {
                    let region = Rectangle::new(Vector::new(250.0, (500 - time_in_millis) as f32), self.images.get(&"down".to_string()).unwrap().size());
                    gfx.draw_image(&self.images.get(&"down".to_string()).unwrap(), region);
                },
                song::UP => {
                    let region = Rectangle::new(Vector::new(450.0, (500 - time_in_millis) as f32), self.images.get(&"up".to_string()).unwrap().size());
                    gfx.draw_image(&self.images.get(&"up".to_string()).unwrap(), region);
                },
                song::RIGHT => {
                    let region = Rectangle::new(Vector::new(650.0, (500 - time_in_millis) as f32), self.images.get(&"right".to_string()).unwrap().size());
                    gfx.draw_image(&self.images.get(&"right".to_string()).unwrap(), region);
                },
                _ => (),
            }
        }
        gfx.set_view(Transform::translate(Vector::new(0.0, 0.0)));

        if self.songs.get(&self.current_song).unwrap().keys.len() < 1 {

            self.game_state = game_state::song_finish;
        }
    }


    //Intro
    fn render_song_game_screen_intro(&mut self, gfx: &mut Graphics) {

        if self.timer.elapsed().as_millis() < 1000 {
            let region = Rectangle::new(Vector::new(300.0, 250.0), self.images.get(&"three".to_string()).unwrap().size());
            gfx.draw_image(&self.images.get(&"three".to_string()).unwrap(), region);
        }
        else if self.timer.elapsed().as_millis() < 2000 {
            let region = Rectangle::new(Vector::new(300.0, 250.0), self.images.get(&"two".to_string()).unwrap().size());
            gfx.draw_image(&self.images.get(&"two".to_string()).unwrap(), region);
        }
        else if self.timer.elapsed().as_millis() < 3000 {
            let region = Rectangle::new(Vector::new(300.0, 250.0), self.images.get(&"one".to_string()).unwrap().size());
            gfx.draw_image(&self.images.get(&"one".to_string()).unwrap(), region);
        }
        else {
            match self.song {

                songs::karin => {
                    self.audio.play((audio::SINK_1, "karin".to_string(), audio::PLAY));
                },
                songs::xepher => {
                    self.audio.play((audio::SINK_1, "xepher".to_string(), audio::PLAY));
                },
                _ => (),
            }
            self.game_state = game_state::song_game_screen_play;
            self.timer = Instant::now();
        }
    }


    //Start song
    async fn render_song_game_screen_start(&mut self) {

        self.game_state = game_state::song_game_screen_intro;
        self.timer = Instant::now();
    }


    //Load song
    async fn render_load_song(&mut self) {

        match self.song {

            songs::karin => {
                self.audio.play((audio::SINK_1, "karin".to_string(), audio::PAUSE));
                self.audio.play((audio::SINK_1, "karin.mp3".to_string(), audio::LOAD));
                self.songs.get_mut(&"karin".to_string()).unwrap().reload("karin".to_string());
            },
            songs::xepher => {
                self.audio.play((audio::SINK_1, "xepher".to_string(), audio::PAUSE));
                self.audio.play((audio::SINK_1, "xepher.mp3".to_string(), audio::LOAD));
                self.songs.get_mut(&"xepher".to_string()).unwrap().reload("xepher".to_string());
            },
            _ => (),
        }
        self.score = 0;
        self.game_state = game_state::song_game_screen_start;
    }


    //Song select
    fn input_song_select(&mut self, key: KeyboardEvent) {

        if key.key() == Key::Escape && key.is_down() {

            self.game_state = game_state::main_menu;
        }
        else if (key.key() == Key::S || key.key() == Key::Down || key.key() == Key::W || key.key() == Key::Up) && key.is_down() {

            match self.song {

                songs::karin => {
                    self.song = songs::xepher;
                },
                songs::xepher => {
                    self.song = songs::karin;
                },
                _ => (),
            }
        }
        else if (key.key() == Key::Return || key.key() == Key::Space) && key.is_down() {

            match self.song {

                songs::karin => {
                    self.game_state = game_state::load_song;
                    self.current_song = "karin".to_string();
                },
                songs::xepher => {
                    self.game_state = game_state::load_song;
                    self.current_song = "xepher".to_string();
                },
                _ => (),
            }
        }
    }

    fn render_song_select(&self, gfx: &mut Graphics) {

        match self.song {
            songs::karin => {

                let mut region = Rectangle::new(Vector::new(200.0, 0.0), self.images.get(&"song_select_karin_selected".to_string()).unwrap().size());
                gfx.draw_image(&self.images.get(&"song_select_karin_selected".to_string()).unwrap(), region);

                region = Rectangle::new(Vector::new(200.0, 250.0), self.images.get(&"song_select_xepher".to_string()).unwrap().size());
                gfx.draw_image(&self.images.get(&"song_select_xepher".to_string()).unwrap(), region);
            },
            songs::xepher => {

                let mut region = Rectangle::new(Vector::new(200.0, 0.0), self.images.get(&"song_select_karin".to_string()).unwrap().size());
                gfx.draw_image(&self.images.get(&"song_select_karin".to_string()).unwrap(), region);

                region = Rectangle::new(Vector::new(200.0, 250.0), self.images.get(&"song_select_xepher_selected".to_string()).unwrap().size());
                gfx.draw_image(&self.images.get(&"song_select_xepher_selected".to_string()).unwrap(), region);
            },
            _ => (),
        }
    }


    //Main Menu
    #[cfg(not(target_arch = "wasm32"))]
    fn input_main_menu(&mut self, key: KeyboardEvent) {

        if key.key() == Key::Escape && key.is_down() {

            self.game_state = game_state::splash_screen;
        }
        else if (key.key() == Key::S || key.key() == Key::Down) && key.is_down() {
            
            match self.main_menu_state {

                main_menu_state::play => {
                    self.main_menu_state = main_menu_state::settings;
                },
                main_menu_state::settings => {
                    self.main_menu_state = main_menu_state::exit;
                },
                main_menu_state::exit => {
                    self.main_menu_state = main_menu_state::play;
                },
                _ => (),
            }
        }
        else if (key.key() == Key::W || key.key() == Key::Up) && key.is_down() {
            
            match self.main_menu_state {

                main_menu_state::play => {
                    self.main_menu_state = main_menu_state::exit;
                },
                main_menu_state::settings => {
                    self.main_menu_state = main_menu_state::play;
                },
                main_menu_state::exit => {
                    self.main_menu_state = main_menu_state::settings;
                },
                _ => (),
            }
        }
        else if (key.key() == Key::Return || key.key() == Key::Space) && key.is_down() {

            match self.main_menu_state {

                main_menu_state::play => {
                    self.game_state = game_state::song_select;
                },
                main_menu_state::settings => {
                    //self.main_menu_state = main_menu_state::play;
                },
                main_menu_state::exit => {
                    self.game_state = game_state::exit;
                },
                _ => (),
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn input_main_menu(&mut self, key: KeyboardEvent) {

        if key.key() == Key::Escape && key.is_down() {

            self.game_state = game_state::splash_screen;
        }
        else if (key.key() == Key::S || key.key() == Key::Down || key.key() == Key::W || key.key() == Key::Up) && key.is_down() {
            
            match self.main_menu_state {

                main_menu_state::play => {
                    self.main_menu_state = main_menu_state::settings;
                },
                main_menu_state::settings => {
                    self.main_menu_state = main_menu_state::play;
                },
                _ => (),
            }
        }
        else if (key.key() == Key::Return || key.key() == Key::Space) && key.is_down() {

            match self.main_menu_state {

                main_menu_state::play => {
                    self.game_state = game_state::song_select;
                },
                main_menu_state::settings => {
                    //self.main_menu_state = main_menu_state::play;
                },
                _ => (),
            }
        }
    } 

    #[cfg(not(target_arch = "wasm32"))]
    fn render_main_menu(&self, gfx: &mut Graphics) {
        
        match self.main_menu_state {
            main_menu_state::play => {

                let mut region = Rectangle::new(Vector::new(200.0, 0.0), self.images.get(&"main_menu_play_selected".to_string()).unwrap().size());
                gfx.draw_image(&self.images.get(&"main_menu_play_selected".to_string()).unwrap(), region);

                region = Rectangle::new(Vector::new(200.0, 250.0), self.images.get(&"main_menu_settings".to_string()).unwrap().size());
                gfx.draw_image(&self.images.get(&"main_menu_settings".to_string()).unwrap(), region);

                region = Rectangle::new(Vector::new(200.0, 500.0), self.images.get(&"main_menu_exit".to_string()).unwrap().size());
                gfx.draw_image(&self.images.get(&"main_menu_exit".to_string()).unwrap(), region);
            },
            main_menu_state::settings => {

                let mut region = Rectangle::new(Vector::new(200.0, 0.0), self.images.get(&"main_menu_play".to_string()).unwrap().size());
                gfx.draw_image(&self.images.get(&"main_menu_play".to_string()).unwrap(), region);

                region = Rectangle::new(Vector::new(200.0, 250.0), self.images.get(&"main_menu_settings_selected".to_string()).unwrap().size());
                gfx.draw_image(&self.images.get(&"main_menu_settings_selected".to_string()).unwrap(), region);

                region = Rectangle::new(Vector::new(200.0, 500.0), self.images.get(&"main_menu_exit".to_string()).unwrap().size());
                gfx.draw_image(&self.images.get(&"main_menu_exit".to_string()).unwrap(), region);
            },
            main_menu_state::exit => {

                let mut region = Rectangle::new(Vector::new(200.0, 0.0), self.images.get(&"main_menu_play".to_string()).unwrap().size());
                gfx.draw_image(&self.images.get(&"main_menu_play".to_string()).unwrap(), region);

                region = Rectangle::new(Vector::new(200.0, 250.0), self.images.get(&"main_menu_settings".to_string()).unwrap().size());
                gfx.draw_image(&self.images.get(&"main_menu_settings".to_string()).unwrap(), region);

                region = Rectangle::new(Vector::new(200.0, 500.0), self.images.get(&"main_menu_exit_selected".to_string()).unwrap().size());
                gfx.draw_image(&self.images.get(&"main_menu_exit_selected".to_string()).unwrap(), region);
            },
            _ => (),
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn render_main_menu(&self, gfx: &mut Graphics) {
        
        match self.main_menu_state {
            main_menu_state::play => {

                let mut region = Rectangle::new(Vector::new(200.0, 0.0), self.images.get(&"main_menu_play_selected".to_string()).unwrap().size());
                gfx.draw_image(&self.images.get(&"main_menu_play_selected".to_string()).unwrap(), region);

                region = Rectangle::new(Vector::new(200.0, 250.0), self.images.get(&"main_menu_settings".to_string()).unwrap().size());
                gfx.draw_image(&self.images.get(&"main_menu_settings".to_string()).unwrap(), region);
            },
            main_menu_state::settings => {

                let mut region = Rectangle::new(Vector::new(200.0, 0.0), self.images.get(&"main_menu_play".to_string()).unwrap().size());
                gfx.draw_image(&self.images.get(&"main_menu_play".to_string()).unwrap(), region);

                region = Rectangle::new(Vector::new(200.0, 250.0), self.images.get(&"main_menu_settings_selected".to_string()).unwrap().size());
                gfx.draw_image(&self.images.get(&"main_menu_settings_selected".to_string()).unwrap(), region);
            },
            _ => (),
        }
    }

    //Load web screen
    async fn render_web_load(&mut self, gfx: &mut Graphics) {

        self.images.insert("web_wait_screen".to_string(), Image::load(gfx, "web_wait_screen.png").await.unwrap());
        let region = Rectangle::new(Vector::new(0.0, 0.0), self.images.get(&"web_wait_screen".to_string()).unwrap().size());
        gfx.draw_image(&self.images.get(&"web_wait_screen".to_string()).unwrap(), region);
        self.game_state = game_state::web_wait_start;
    }

    //Load boot screen
    async fn render_init_boot(&mut self, gfx: &mut Graphics) {

        self.images.clear();
        self.images.insert("boot_screen".to_string(), Image::load(gfx, "boot_screen.png").await.unwrap());
        let region = Rectangle::new(Vector::new(0.0, 0.0), self.images.get(&"boot_screen".to_string()).unwrap().size());
        gfx.draw_image(&self.images.get(&"boot_screen".to_string()).unwrap(), region);
        self.game_state = game_state::boot;
    }

    //Load resources
    async fn render_boot(&mut self, gfx: &mut Graphics) {

        self.images.insert("main_menu_settings".to_string(), Image::load(gfx, "main_menu_settings.png").await.unwrap());
        self.images.insert("main_menu_play".to_string(), Image::load(gfx, "main_menu_play.png").await.unwrap());
        self.images.insert("main_menu_exit".to_string(), Image::load(gfx, "main_menu_exit.png").await.unwrap());
        self.images.insert("main_menu_settings_selected".to_string(), Image::load(gfx, "main_menu_settings_selected.png").await.unwrap());
        self.images.insert("main_menu_play_selected".to_string(), Image::load(gfx, "main_menu_play_selected.png").await.unwrap());
        self.images.insert("main_menu_exit_selected".to_string(), Image::load(gfx, "main_menu_exit_selected.png").await.unwrap());
        self.images.insert("splash_screen".to_string(), Image::load(gfx, "splash_screen.png").await.unwrap());
        self.images.insert("song_select_karin".to_string(), Image::load(gfx, "song_select_karin.png").await.unwrap());
        self.images.insert("song_select_karin_selected".to_string(), Image::load(gfx, "song_select_karin_selected.png").await.unwrap());
        self.images.insert("song_select_xepher".to_string(), Image::load(gfx, "song_select_xepher.png").await.unwrap());
        self.images.insert("song_select_xepher_selected".to_string(), Image::load(gfx, "song_select_xepher_selected.png").await.unwrap());
        self.images.insert("three".to_string(), Image::load(gfx, "three.png").await.unwrap());
        self.images.insert("two".to_string(), Image::load(gfx, "two.png").await.unwrap());
        self.images.insert("one".to_string(), Image::load(gfx, "one.png").await.unwrap());
        self.images.insert("up".to_string(), Image::load(gfx, "up.png").await.unwrap());
        self.images.insert("up_hud".to_string(), Image::load(gfx, "up_hud.png").await.unwrap());
        self.images.insert("up_hud_selected".to_string(), Image::load(gfx, "up_hud_selected.png").await.unwrap());
        self.images.insert("down".to_string(), Image::load(gfx, "down.png").await.unwrap());
        self.images.insert("down_hud".to_string(), Image::load(gfx, "down_hud.png").await.unwrap());
        self.images.insert("down_hud_selected".to_string(), Image::load(gfx, "down_hud_selected.png").await.unwrap());
        self.images.insert("left".to_string(), Image::load(gfx, "left.png").await.unwrap());
        self.images.insert("left_hud".to_string(), Image::load(gfx, "left_hud.png").await.unwrap());
        self.images.insert("left_hud_selected".to_string(), Image::load(gfx, "left_hud_selected.png").await.unwrap());
        self.images.insert("right".to_string(), Image::load(gfx, "right.png").await.unwrap());
        self.images.insert("right_hud".to_string(), Image::load(gfx, "right_hud.png").await.unwrap());
        self.images.insert("right_hud_selected".to_string(), Image::load(gfx, "right_hud_selected.png").await.unwrap());
        self.game_state = game_state::splash_screen;
    }


    //Splash Screen
    fn input_splash_screen(&mut self, key: KeyboardEvent) {

        if key.is_down() {

            self.game_state = game_state::main_menu;
        }

        #[cfg(not(target_arch = "wasm32"))]
        if key.key() == Key::Escape && key.is_down() {

            self.game_state = game_state::exit;
        }
    }

    fn update_splash_screen(&mut self) {


    }

    fn render_splash_screen(&self, gfx: &mut Graphics) {

        let region = Rectangle::new(Vector::new(0.0, 0.0), self.images.get(&"splash_screen".to_string()).unwrap().size());
        gfx.draw_image(&self.images.get(&"splash_screen".to_string()).unwrap(), region);
    }


    //Web Wait Screen (web does not allow sound to play on start, so wait for them to interact before booting the game)
    fn input_web_wait_start(&mut self, key: KeyboardEvent) {

        if key.is_down() {

            self.game_state = game_state::init_boot;
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn update_web_wait_start(&mut self) {


    }

    //Should not end up here, but if we do go straight to boot
    #[cfg(not(target_arch = "wasm32"))]
    fn update_web_wait_start(&mut self) {

        self.game_state = game_state::init_boot;
    }

    fn render_web_wait_start(&self, gfx: &mut Graphics) {

        let region = Rectangle::new(Vector::new(0.0, 0.0), self.images.get(&"web_wait_screen".to_string()).unwrap().size());
        gfx.draw_image(&self.images.get(&"web_wait_screen".to_string()).unwrap(), region);
    }
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {

    gfx.set_camera_size(Vector::new(1024.0, 768.0));

    let mut seconds_timer = Timer::time_per_second(1.0);
    let mut update_timer = Timer::time_per_second(60.0);
    let mut draw_timer = Timer::time_per_second(60.0);

    let ttf = VectorFont::load("lady_radical.ttf").await?;
    let mut font = ttf.to_renderer(&gfx, 56.0)?;

    let mut frames: u32 = 0;
    let mut fps = String::from("fps ");
    fps.push_str(&frames.to_string());

    let mut running = true;

    let mut camera_pos = Vector::new(0.0, 0.0);

    let mut game = Game::new();

    while running {
        while let Some(event) = input.next_event().await {

            match event {

                Event::KeyboardInput(key) => {

                    if key.key() == Key::Key0 && key.is_down() {

                        game.audio.play((audio::SINK_1, "karin".to_string(), audio::PLAY));
                    }
                    else if key.key() == Key::Key9 && key.is_down() {

                        game.audio.play((audio::SINK_1, "xepher".to_string(), audio::PLAY));
                    }

                    game.input(key);
                },
                Event::PointerInput(button) if button.is_down() => {

                    //One off case, if waiting to start web version allow user to click to start
                    if button.button() == MouseButton::Left {
                        
                        match game.game_state {
                            
                            game_state::web_wait_start => {

                                game.game_state = game_state::init_boot;
                            },
                            _ => (),
                        }
                    }
                },
                _ => (),
            }
        }

        //Calculate fps and create renderable string
        frames += 1;
        if seconds_timer.exhaust().is_some() {
            fps = String::from("fps ");
            fps.push_str(&frames.to_string());
            frames = 0;
        }

        if draw_timer.exhaust().is_some() {

            //Clear Window
            gfx.clear(Color::BLACK);

            //Render Game
            game.render(&mut gfx).await;

            //Draw FPS
            gfx.set_view(Transform::translate(Vector::new(0.0, 0.0)));
            font.draw(&mut gfx, &fps, Color::WHITE, Vector::new(32.0, 32.0))?;

            let mut score_string = String::from("score ");
            score_string.push_str(&game.score.to_string());

            font.draw(&mut gfx, &score_string, Color::WHITE, Vector::new(500.0, 32.0))?;

            //Finally show image
            gfx.present(&window)?;
        }

        //Quit Game
        match game.game_state {

            game_state::exit => {

                running = false;
            },
            _ => (),
        }
    }

    Ok(())
}