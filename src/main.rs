mod map_generator;
mod cell;
mod cell_type;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Window", 1280, 720)
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut tick = 0;

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    //let mut red_square = create_texture_rect(&texture_creator, Color::RGBA(255, 0, 0, 255), 2, 2)?;
    let width = 1280;
    let height = 720;
    let mut map_texture = create_texture_rect(&texture_creator, Color::BLACK, width, height)?;
    let map = map_generator::generate_map(width as usize,height as usize);
    println!("finished generating the map");

    for i in 0..map.len() {
        for j in 0..map[0].len(){
            set_pixel(&mut map_texture, i, j, map[i][j].get_cell_color())?;
        }
    }
    let mut mouse_pos = (0,0);
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion { x, y, .. } => {
                    mouse_pos = (x, y);
                }
                _ => {}
            }
        }

        {
            // Update the window title.
            let window = canvas.window_mut();

            let position = window.position();
            let size = window.size();
            let title = format!(
                "pos({}x{}), size({}x{}), mouse_pos({}x{}): {}",
                position.0, position.1, size.0, size.1,mouse_pos.0,mouse_pos.1, tick
            );
            window.set_title(&title).map_err(|e| e.to_string())?;

            tick += 1;
        }
        let scale_x = 2.0;
        let scale_y = 2.0;
        canvas.set_scale(scale_x, scale_y)?;

        let translate_x = 0;
        let translate_y = 0;
        let translated_rect = Rect::new(translate_x, translate_y, width, height);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.copy(&map_texture, None, translated_rect)?;

        // Present the result
        canvas.present();
    }

    Ok(())
}
fn create_texture_rect<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    color: Color,
    width: u32,
    height: u32,
) -> Result<Texture<'a>, String> {
    let pixel_format_enum = sdl2::pixels::PixelFormatEnum::ABGR8888;
    let mut texture = texture_creator
        .create_texture_streaming(pixel_format_enum, width, height)
        .map_err(|e| e.to_string())?;
    texture.set_blend_mode(sdl2::render::BlendMode::Blend);
    texture
        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..height {
                for x in 0..width {
                    let offset = y as usize * pitch + x as usize * 4;
                    buffer[offset] = color.a;
                    buffer[offset + 1] = color.b;
                    buffer[offset + 2] = color.g;
                    buffer[offset + 3] = color.r;
                }
            }
        })
        .map_err(|e| e.to_string())?;
    Ok(texture)
}
fn set_pixel(
    texture: &mut sdl2::render::Texture,
    x: usize,
    y: usize,
    color: Color,
) -> Result<(), String> {
    let texture_info = texture.query();
    let width = texture_info.width as usize;
    let height = texture_info.height as usize;

    if x >= width || y >= height {
        return Err("Pixel coordinates out of bounds".to_string());
    }
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        let offset = y * pitch + x * 4;
        buffer[offset] = color.r;
        buffer[offset + 1] = color.g;
        buffer[offset + 2] = color.b;
        buffer[offset + 3] = color.a;
    })?;

    Ok(())
}
