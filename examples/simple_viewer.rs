// Rust implementation of the OpenNI2 C++ sample
// https://github.com/OpenNI/OpenNI2/blob/master/Samples/SimpleViewer/main.cpp

extern crate minifb;
extern crate openni2;
use minifb::{ Window, Key, KeyRepeat, WindowOptions, Scale };
use std::{mem, process};
use openni2::{
    Status,
    Device,
    SensorType,
    OniDepthPixel,
    OniRGB888Pixel,
    VideoMode,
    PixelFormat,
};

fn main() -> Result<(), Status> {
    openni2::init(2, 2)?;
    let device = Device::open_default()?;
    let depth = device.create_stream::<OniDepthPixel>(SensorType::DEPTH)?;
    depth.set_video_mode(VideoMode {
        pixel_format: PixelFormat::DEPTH_1_MM,
        resolution_x: 320,
        resolution_y: 240,
        fps: 30,
    })?;
    let color = device.create_stream::<OniRGB888Pixel>(SensorType::COLOR)?;
    color.set_video_mode(VideoMode {
        pixel_format: PixelFormat::RGB888,
        resolution_x: 320,
        resolution_y: 240,
        fps: 30,
    })?;
    // let ir = device.create_stream::<OniRGB888Pixel>(SensorType::IR)?;
    // ir.set_video_mode(VideoMode {
    //     pixel_format: PixelFormat::RGB888,
    //     resolution_x: 320,
    //     resolution_y: 240,
    //     fps: 30,
    // })?;

    let mut window = match Window::new("OpenNI2 Simple Viewer", 320, 240, WindowOptions {
        resize: false,
        scale: Scale::X2,
        ..Default::default()
    }) {
        Ok(window) => window,
        Err(_) => process::exit(1),
    };

    color.start();
    depth.start();

    let color_reader = color.reader();
    let depth_reader = depth.reader();

    let mut display_color = true;
    let mut display_depth = false;
    let mut buffer: [u32; 320 * 240] = unsafe { mem::zeroed() };
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let color_frame = color_reader.read();
        let depth_frame = depth_reader.read();
        for (i, (color, depth)) in color_frame.pixels().iter().zip(depth_frame.pixels()).enumerate() {
            if display_depth && *depth > 0 {
                let brightness = (depth / 256) as u32;
                buffer[i] = brightness << 16 | brightness << 8 | brightness;
            } else if display_color {
                buffer[i] = ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32);
            } else {
                buffer[i] = 0;
            }
        }
        window.update_with_buffer(&buffer).expect("Couldn't write to minifb");
        window.get_keys_pressed(KeyRepeat::No).map(|keys| {
            for t in keys {
                match t {
                    Key::Key1 | Key::NumPad1 => {
                        display_color = true;
                        display_depth = true;
                        device.set_image_registration(true).unwrap();
                    },
                    Key::Key2 | Key::NumPad2 => {
                        display_color = true;
                        display_depth = false;
                        device.set_image_registration(false).unwrap();
                    },
                    Key::Key3 | Key::NumPad3 => {
                        display_color = false;
                        display_depth = true;
                        device.set_image_registration(false).unwrap();
                    },
                    _ => (),
                }
            }
        });
    }

    color.stop();
    depth.stop();
    Ok(())
}