extern crate sdl2;


// use sdl2::init;
//use std::thread::sleep;
//use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard;
use sdl2::rect::Point;

//use sdl2::rect::Rect;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("Window Title",400,400)
        .position(300,200)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();

        canvas.set_draw_color(Color::RGB(255,0,255)); // nice pink background
        canvas.clear(); // uses the color set in set_draw_color
        canvas.present();




    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'main
                }
                Event::Window { timestamp: _, window_id: _, win_event } => {
                    match win_event {
                        sdl2::event::WindowEvent::SizeChanged(_x,_y) => {
                            canvas.set_draw_color(Color::RGB(255,0,255));
                            canvas.clear();
                            // canvas.fill_rect(Rect::new(0,0,50,50)).unwrap();
                            // canvas.fill_rect(None).unwrap(); // rect matches canvas size

                        }
                        _ => {}
                    }
                }
                Event::KeyDown { timestamp: _, window_id: _,
                    keycode , scancode: _, keymod: _, repeat: _ } => {
                        match keycode.unwrap() {
                            keyboard::Keycode::P => {
                                canvas.set_draw_color(Color::RGB(255,0,255));
                                canvas.clear();
                            }
                            keyboard::Keycode::B => {
                                canvas.set_draw_color(Color::RGB(0,0,255));
                                canvas.clear();
                            }
                            keyboard::Keycode::Y => {
                                canvas.set_draw_color(Color::RGB(255,255,0));
                                canvas.clear();
                            }
                            keyboard::Keycode::L => {
                                canvas.set_draw_color(Color::RGB(0,0,0));
                                let x = Point::new(10,10);
                                let y = Point::new(50,50);
                                canvas.draw_line(x,y).unwrap();
                                // canvas.present();
                            }
                            keyboard::Keycode::Escape => {
                                break 'main
                            }
                            _ => {}
                        }
                    }
                _ => {}
            }

            canvas.present();
        }
    }
}
