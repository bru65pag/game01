extern crate sdl2;


// use sdl2::init;
use std::thread::sleep;
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

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

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255,0,255));
        canvas.clear(); // uses the color set in set_draw_color
        canvas.present();
        // sleep(Duration::from_millis(5000));


        // canvas.set_draw_color(Color::RGB(255, 210, 0));
        // // canvas.fill_rect(Rect::new(0,0,50,50)).unwrap();
        // canvas.fill_rect(None).unwrap(); // will completely fill the canvas
        // canvas.present();
    }



}
