extern crate gfx_texture;
extern crate image;
extern crate piston;
extern crate piston_window;
extern crate texture;

use gfx_texture::Texture;
use image::ImageBuffer;
use piston::input::*;
use piston_window::*;
use texture::TextureSettings;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;

fn main() {
	let mut window : PistonWindow = WindowSettings::new("conway", [WIDTH, HEIGHT])
		.exit_on_esc(true)
		.build()
		.expect("error making window");

	let buf = vec![0u8; (WIDTH * HEIGHT * 4) as usize];
	let mut img = ImageBuffer::from_raw(WIDTH, HEIGHT, buf).unwrap();
	let tset = TextureSettings::new();
	let mut tex = Texture::from_image(&mut window.factory, &img, &tset).unwrap();

	while let Some(e) = window.next() {
		if let Some(Button::Keyboard(key)) = e.press_args() {
			if key == Key::Space {
				for p in img.enumerate_pixels_mut() {
					p.2.data = [0xFF, 0xFF, 0xFF, 0xFF];
				}
				tex = Texture::from_image(&mut window.factory, &img, &tset).unwrap();
			}
		}
		window.draw_2d(&e, |c, g| {
			clear([0.0, 0.0, 0.0, 1.0], g);
			image(&tex, c.transform, g);
		});
	}
}
