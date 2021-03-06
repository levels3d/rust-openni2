extern crate openni2;
use openni2::{
    Device,
    Status,
    SensorType,
    Stream,
    OniRGB888Pixel,
    OniDepthPixel,
    OniGrayscale16Pixel,
    Pixel,
    // LogLevel,
};

fn interrogate_stream<PixelType: Pixel>(device: &Device, sensor_type: SensorType) {
    if let Some(sensor_info) = device.get_sensor_info(sensor_type) {
        println!("{:#?}", sensor_info);
        if let Ok(mut stream) = device.create_stream(sensor_type) {
            dump_stream_data(&stream);
            println!("Starting stream: {:?}", stream.start());
            {
                for _ in 0..5 {
                    match stream.read_frame::<PixelType>() {
                        Ok(frame) => println!("Got frame: {:?}", frame),
                        Err(status) => println!("Couldn't read frame! {}", status),
                    }
                }
            }
            stream.stop();
            println!("Stopping stream.");
        }
    } else {
        println!("Couldn't open {:?} stream", sensor_type);
    }
}

fn dump_stream_data(stream: &Stream) {
    println!("Cropping: {:?}", stream.get_cropping().ok());
    println!("Horizontal FOV: {:?}", stream.get_horizontal_fov().ok());
    println!("Vertical FOV: {:?}", stream.get_vertical_fov().ok());
    println!("Video Mode: {:?}", stream.get_video_mode().ok());
    println!("Max Value: {:?}", stream.get_max_value().ok());
    println!("Min Value: {:?}", stream.get_min_value().ok());
    println!("Stride: {:?}", stream.get_stride().ok());
    println!("Mirroring: {:?}", stream.get_mirroring().ok());
    println!("Number of frames: {:?}", stream.get_number_of_frames().ok());
    println!("Auto White Balance: {:?}", stream.get_auto_white_balance().ok());
    println!("Auto Exposure: {:?}", stream.get_auto_exposure().ok());
    println!("Auto Exposure: {:?}", stream.get_auto_exposure().ok());
    println!("Exposure: {:?}", stream.get_exposure().ok());
    println!("Gain: {:?}", stream.get_gain().ok());
}

fn main() -> Result<(), Status> {
    openni2::init()?;

    // openni2::set_console_log(true);
    // openni2::set_log_level(LogLevel::Verbose);

    // Try openni2::Device::open_uri("uri") with a uri string returned
    // from openni2::get_device_list()
    let device = openni2::Device::open_default()?;
    println!("{:?}", device.info());

    println!("Firmware Version: {:?}", device.get_firmware_version());
    println!("Driver Version: {:?}", device.get_driver_version());
    println!("Hardware Version: {:?}", device.get_hardware_version());
    println!("Serial No: {:?}", device.get_serial_number());

    interrogate_stream::<OniRGB888Pixel>(&device, SensorType::COLOR);

    interrogate_stream::<OniDepthPixel>(&device, SensorType::DEPTH);

    interrogate_stream::<OniGrayscale16Pixel>(&device, SensorType::IR);

    openni2::shutdown();
    Ok(())
}
