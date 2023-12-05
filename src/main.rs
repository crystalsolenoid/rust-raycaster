use raycast_utils::{Camera};
use std::f32::consts::PI;
use std::fs;

use std::num::NonZeroU32;
use std::rc::Rc;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::WindowBuilder;

fn write_image(img: &image::RgbImage, fname: &str) {
    // TODO: handle errors
    // make the output directory if it doesn't already exist
    let _ = fs::create_dir("output");
    // write the image
    let kind = image::ColorType::Rgb8;
    let dims = img.dimensions();
    let _ = image::save_buffer("output/".to_string() + fname, img, dims.0, dims.1, kind);
}

fn main() {
    let w: u32 = 512;
    let h: u32 = 512;
    let mut img = image::RgbImage::new(w, h);
    let mut render = image::RgbImage::new(w, h);

    let map = map::gen_map(w, h);
    let camera = Camera {
        x: 380,
        y: 340,
        radians: 0.3 * PI,
        ..Camera::default()
    };

    draw::draw_map(&mut img, &map);

    draw::draw_camera(&mut img, &camera);

    let view = raycast_utils::cast_fov(&map, &camera);
    draw::draw_fov(&mut img, &view, &camera);

    write_image(&img, "map.png");

    draw::draw_view(&mut render, &view, &camera);

    write_image(&render, "render.png");

    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(WindowBuilder::new().build(&event_loop).unwrap());

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;

        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(&window.canvas().unwrap())
            .unwrap();
    }

    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    event_loop
        .run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Wait);

            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::RedrawRequested,
                } if window_id == window.id() => {
                    if let (Some(width), Some(height)) = {
                        let size = window.inner_size();
                        (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                    } {
                        surface.resize(width, height).unwrap();

                        let mut buffer = surface.buffer_mut().unwrap();
                        for y in 0..height.get() {
                            for x in 0..width.get() {
                                let red = x % 255;
                                let green = y % 255;
                                let blue = (x * y) % 255;
                                let index = y as usize * width.get() as usize + x as usize;
                                buffer[index] = blue | (green << 8) | (red << 16);
                            }
                        }

                        buffer.present().unwrap();
                    }
                }
                Event::WindowEvent {
                    event:
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    logical_key: Key::Named(NamedKey::Escape),
                                    ..
                                },
                            ..
                        },
                    window_id,
                } if window_id == window.id() => {
                    elwt.exit();
                }
                _ => {}
            }
        })
        .unwrap();
}
