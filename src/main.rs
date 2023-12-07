use raycast_utils::{Camera};
use std::f32::consts::PI;
use std::fs;

use std::num::NonZeroU32;
use std::rc::Rc;
use winit::event::{Event, KeyEvent, WindowEvent, ElementState};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{PhysicalKey, KeyCode, Key, NamedKey};
use winit::window::WindowBuilder;
use winit::dpi::PhysicalSize;

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

//    let map = map::gen_map(512, 512);
    let map = map::spooky_map();
    let mut camera = Camera {
        x: 240,
        y: 464,
        radians: 0.5 * PI,
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
    let window = Rc::new(WindowBuilder::new()
                         .with_inner_size(PhysicalSize::new(1028, 512))
                         .with_title("Raycaster")
                         .build(&event_loop)
                         .unwrap());

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
                        for y in 0..std::cmp::min(512, height.get()) {
                            for x in 0..std::cmp::min(512, width.get()) {
                                // TODO this is terrible
                                let red = render.get_pixel(x, y)[0] as u32;
                                let green = render.get_pixel(x, y)[1] as u32;
                                let blue = render.get_pixel(x, y)[2] as u32;
                                let index = y as usize * width.get() as usize + x as usize;
                                buffer[index] = blue | (green << 8) | (red << 16);
                            }
                            for x in std::cmp::min(512, width.get())..std::cmp::min(512 * 2, width.get()) {
                                let red = img.get_pixel(x - 512, y)[0] as u32;
                                let green = img.get_pixel(x - 512, y)[1] as u32;
                                let blue = img.get_pixel(x - 512, y)[2] as u32;
                                let index = y as usize * width.get() as usize + x as usize;
                                buffer[index] = blue | (green << 8) | (red << 16);
                            }
                        }

                        buffer.present().unwrap();
                    }
                }

                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    physical_key: PhysicalKey::Code(KeyCode::KeyA),
                                    repeat: false,
                                    state: ElementState::Pressed,
                                    ..
                                },
                                ..
                        },
                    window_id,
                } if window_id == window.id() => {
                    camera.radians += 0.125 * PI;
                    draw::draw_map(&mut img, &map);
                    draw::draw_camera(&mut img, &camera);
                    let view = raycast_utils::cast_fov(&map, &camera);
                    draw::draw_fov(&mut img, &view, &camera);
                    draw::draw_view(&mut render, &view, &camera);
                    window.request_redraw();
                }

                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    physical_key: PhysicalKey::Code(KeyCode::KeyD),
                                    repeat: false,
                                    state: ElementState::Pressed,
                                    ..
                                },
                                ..
                        },
                    window_id,
                } if window_id == window.id() => {
                    camera.radians -= 0.125 * PI;
                    draw::draw_map(&mut img, &map);
                    draw::draw_camera(&mut img, &camera);
                    let view = raycast_utils::cast_fov(&map, &camera);
                    draw::draw_fov(&mut img, &view, &camera);
                    draw::draw_view(&mut render, &view, &camera);
                    window.request_redraw();
                }

                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    physical_key: PhysicalKey::Code(KeyCode::KeyW),
                                    repeat: false,
                                    state: ElementState::Pressed,
                                    ..
                                },
                                ..
                        },
                    window_id,
                } if window_id == window.id() => {
                    camera.x += (10.0 * camera.radians.cos()) as i32;
                    camera.y -= (10.0 * camera.radians.sin()) as i32;
                    draw::draw_map(&mut img, &map);
                    draw::draw_camera(&mut img, &camera);
                    let view = raycast_utils::cast_fov(&map, &camera);
                    draw::draw_fov(&mut img, &view, &camera);
                    draw::draw_view(&mut render, &view, &camera);
                    window.request_redraw();
                }

                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    physical_key: PhysicalKey::Code(KeyCode::KeyS),
                                    repeat: false,
                                    state: ElementState::Pressed,
                                    ..
                                },
                                ..
                        },
                    window_id,
                } if window_id == window.id() => {
                    camera.x -= (10.0 * camera.radians.cos()) as i32;
                    camera.y += (10.0 * camera.radians.sin()) as i32;
                    draw::draw_map(&mut img, &map);
                    draw::draw_camera(&mut img, &camera);
                    let view = raycast_utils::cast_fov(&map, &camera);
                    draw::draw_fov(&mut img, &view, &camera);
                    draw::draw_view(&mut render, &view, &camera);
                    window.request_redraw();
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
