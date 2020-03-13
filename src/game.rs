pub mod game {

    extern crate sdl2;
    extern crate sdl2_sys;

    use sdl2::pixels::Color;
    use sdl2::event::Event;
    use sdl2::keyboard;
    use sdl2::rect::Point;
    use sdl2::render::Canvas;
    use std::time::Duration;

    struct FPS {
        frame_count:u64,
        timer_fps:u64,
        last_frame:u64,
        last_time:u64,
    }

    struct WindowSize {
        height: u32,
        width: u32,
    }

    struct WindowLocation {
        x: i32,
        y: i32,
    }

    pub struct Game {
        title: String,
        sdl: sdl2::Sdl,
        canvas: Canvas<sdl2::video::Window>,
        fps: FPS,
        timer: sdl2::TimerSubsystem,
        running: bool,
    }

    impl Game {

        pub fn get_title(&self) {
            println!("Game title is {}", self.title);
        }

        pub fn new(title: String) -> Game {

            let window_size = WindowSize {height: 400, width: 400};
            let window_location = WindowLocation {x:320, y: 300};
            let sdl = sdl2::init().unwrap();
            let video = sdl.video().unwrap();
            let timer = sdl.timer().unwrap();
            let window = video.window(&title,window_size.height,window_size.width)
                .position(window_location.x,window_location.y)
                .resizable()
                .build()
                .unwrap();
            let canvas = window.into_canvas()
                .present_vsync()
                .build()
                .unwrap();
            Game {
                title: title,
                sdl: sdl,
                canvas: canvas,
                fps : FPS {
                    frame_count: 0,
                    timer_fps: 0,
                    last_frame: 0,
                    last_time: 0
                },
                timer: timer,
                running: true,
            }
        }

         pub fn game_loop(&mut self) {
            self.canvas.set_draw_color(Color::RGB(255,0,255));
            self.canvas.clear();
            while self.running {
                self.fps.last_frame = self.timer.performance_counter();
                if self.fps.last_frame >= self.fps.last_time+1000 {
                    self.fps.last_time = self.fps.last_frame;
                    self.fps.frame_count = 0;
                }
                self.render();
                self.input();
                self.update();
            }
        }

        fn render(&mut self) {

            // draw_game();

            self.fps.frame_count += 1;
            self.fps.last_frame = self.timer.performance_counter() - self.fps.last_frame;
            if self.fps.timer_fps<(1000/60) {
                std::thread::sleep(Duration::from_millis((1000/60)-self.fps.timer_fps));
            }
            self.canvas.present();
        }

        fn input(&mut self) {
            let mut event_pump = self.sdl.event_pump().unwrap();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        self.running = false;
                    }
                    Event::Window { timestamp: _, window_id: _, win_event } => {
                        match win_event {
                            sdl2::event::WindowEvent::SizeChanged(_x,_y) => {
                                self.canvas.set_draw_color(Color::RGB(255,0,255));
                                self.canvas.clear();
                            }
                            _ => {
                            }
                        }
                    }
                    Event::KeyDown { timestamp: _, window_id: _,
                        keycode , scancode: _, keymod: _, repeat: _ } => {
                            match keycode.unwrap() {
                                keyboard::Keycode::P => {
                                    self.canvas.set_draw_color(Color::RGB(255,0,255));
                                    self.canvas.clear();
                                }
                                keyboard::Keycode::B => {
                                    self.canvas.set_draw_color(Color::RGB(0,0,255));
                                    self.canvas.clear();
                                }
                                keyboard::Keycode::Y => {
                                    self.canvas.set_draw_color(Color::RGB(255,255,0));
                                    self.canvas.clear();
                                }
                                keyboard::Keycode::L => {
                                    self.canvas.set_draw_color(Color::RGB(0,0,0));
                                    let x = Point::new(10,10);
                                    let y = Point::new(50,50);
                                    self.canvas.draw_line(x,y).unwrap();
                                }
                                keyboard::Keycode::Escape => {
                                    self.running = false;
                                }
                                _ => {
                                }
                            }
                    }
                    _ => {
                    }
                }
            }
        }

        pub fn update(&mut self) {
        }
    }
}
