extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::sys::image::IMG_InitFlags_IMG_INIT_PNG;
use std::time::{Duration,SystemTime};
use std::thread::sleep;
use sdl2::video::{Window,WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::rect::Rect;
use sdl2::image::{LoadTexture, InitFlag};
use std::fs::File;
use std::io::{self, Write};



#[derive(Clone,Copy)]
enum TextureColor {
    Green,
    Blue,
}

fn create_texture_rect<'a>(canvas: &mut Canvas<Window>,texture_creator: &'a TextureCreator<WindowContext>, color: TextureColor, size: u32) -> Option<Texture<'a>> {
    if let Ok(mut square_texture)= texture_creator.create_texture_target(None, size, size){
        canvas.with_texture_canvas(&mut square_texture, |texture|{
            match  color {
                TextureColor::Green =>
                texture.set_draw_color(Color::RGB(0, 255, 0)),
                TextureColor::Blue =>
                texture.set_draw_color(Color::RGB(0, 0, 255)),
            }
            texture.clear();
        }).expect("Failed to color texture");
        Some(square_texture)
    }else {
        None
    }
}

fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut f = File::create(file_name)?;
    f.write_all(content.as_bytes());
}


pub fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem.");

    sdl2::image::init(InitFlag::JPG | InitFlag::PNG).expect("Couldn't initialize");



    let _now = SystemTime::now();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600).position_centered().opengl().build().expect("Failed to create window.");

    let mut canvas = window.into_canvas().target_texture().present_vsync().build().expect("Failed to convert window into canvas.");

    let texture_creator : TextureCreator<_> = canvas.texture_creator();
    const TEXTURE_SIZE: u32 = 32;

    let image_texture = texture_creator.load_texture("src/assets/1955.528.jpg").expect("Couldn't load image.");


    let mut green_square: Texture = texture_creator.create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE).expect("Failed to create a texture.");

    canvas.with_texture_canvas(&mut green_square, |texture| {
        texture.set_draw_color(Color::RGB(0, 255, 0));
        texture.clear();
    }).expect("Failed to color a texture.");


    let  blue_square = create_texture_rect(&mut canvas, &texture_creator, TextureColor::Blue, TEXTURE_SIZE).expect("Failed to creat a blue texture");


    

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump.");
    'running: loop{
        for event in event_pump.poll_iter(){
            match event {
                Event::Quit { .. } | 
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _=> {}
            }
        }
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        sleep(Duration::new(1, 0));
        match _now.elapsed() {
            Ok(elapsed) =>{
                if elapsed.as_secs().div_ceil(1) % 2 == 0 {
            
                    canvas.copy(&green_square, None, Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE))
        .expect("Couldn't copy texture into window.");
                }
                else {
                    canvas.copy(&blue_square, None, Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE))
        .expect("Couldn't copy texture into window.");
                }
            }
            Err(e)=>{
                println!("Error: {e:?}");
            }
        }
        
        canvas.copy(&image_texture, None, None).expect("Render failed");
        canvas.present();
        
        sleep(Duration::new(0, 1_000_000_000u32 /60));
    }
}