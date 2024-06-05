use dialoguer::{theme::ColorfulTheme, Select};
use cpal::traits::{HostTrait, DeviceTrait};
use cpal::*;
use cpal::platform::Host;
use std::sync::{Arc, Mutex};
use std::time::Duration;


fn select_device(option: usize, host: &Host) -> String{
    let mut devices = Vec::new();
    match option {
        0 => {
            println!("Input devices:");
            for device in host.input_devices().unwrap() {
                devices.push(device.name().unwrap());
            }
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select and option")
                .default(0)
                .items(&devices[..])
                .interact()
                .unwrap();

let selected_device = &devices[selection];
            selected_device.to_string()
        }
        1 => {
            println!("Output devices:");
            for device in host.output_devices().unwrap() {
                devices.push(device.name().unwrap());
            }
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select and option")
                .default(0)
                .items(&devices[..])
                .interact()
                .unwrap();

            let selected_device = &devices[selection];
            selected_device.to_string()
        }
        _ => {
            "We are the heroes".to_string()
        }
    }
}

fn select_config(device: &Device) -> StreamConfig {
    let supported_configs_range = device.supported_input_configs()
        .expect("Error querying configs");

    let mut sample_rates = Vec::new();
    let mut sample_formats = Vec::new();
    let mut buffer_sizes = Vec::new();

    for config in supported_configs_range {
        sample_rates.push(config.min_sample_rate().0); //min sample rate
        sample_rates.push(config.max_sample_rate().0); //max sample rate
        sample_formats.push(format!("{:?}", config.sample_format())); // sample format
        match config.buffer_size() {
            SupportedBufferSize::Range { min, max } => {
                for size in (*min..=*max).step_by(256) {
                    buffer_sizes.push(size);
                }
            },
            SupportedBufferSize::Unknown => {
                // Add something if needed
            }
        }
    }

    sample_rates.sort();
    sample_rates.dedup();

    buffer_sizes.sort();
    buffer_sizes.dedup();

    let sample_rate_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Sample Rate")
        .default(0)
        .items(&sample_rates)
        .interact()
        .unwrap();

    let _sample_format_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Sample Format")
        .default(0)
        .items(&sample_formats)
        .interact()
        .unwrap();

    let buffer_size_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Buffer Size")
        .default(0)
        .items(&buffer_sizes)
        .interact()
        .unwrap();

    let config =  StreamConfig {
        channels: 1,
        sample_rate: SampleRate(sample_rates[sample_rate_selection]),
        buffer_size: BufferSize::Fixed(buffer_sizes[buffer_size_selection]),
    };

    config


}

fn main_menu() -> usize {
    let selections = &[
        "Select Input Device",
        "Select Output Device",
        "Record Audio",
        "Play Audio"
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select and option")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    return selection

}


fn main() {

    // Initialize Audio API
    let host: Host = cpal::default_host();
    let mut input_device: Option<cpal::Device> = None;
    let mut output_device: Option<cpal::Device> = None;
    // Query the API - List Interfaces
    // Display Interface Options (for the user)
    loop {
        let selection = main_menu();
        // User Interface Choice (save that choice and initiliaze accordingly)
        match selection {
            0 => {
                let device_name = select_device(selection, &host);
                let device = host.output_devices().unwrap()
                    .find(|d| d.name().unwrap() == device_name)
                    .expect("Failed to find output device");
                input_device = Some(device);
            }
            1 => {
                let device_name = select_device(selection, &host);
                let device = host.output_devices().unwrap()
                    .find(|d| d.name().unwrap() == device_name)
                    .expect("Failed to find output device");
                output_device = Some(device);
            }
            2 => {

                if let Some(ref device) = input_device {

                let config: StreamConfig = select_config(&device.clone());

                let audio_buffer = Arc::new(Mutex::new(Vec::new()));

                let buffer_clone = Arc::clone(&audio_buffer);
                let timeout: Duration = Duration::new(1, 0);

                let _stream = device.build_input_stream(
                    &config,
                    move |data: &[f32], _: &InputCallbackInfo| {
                        let mut buffer = buffer_clone.lock().unwrap();
                        buffer.extend_from_slice(data);
                        },
                    |err| eprintln!("An error occured on the input audio stream: {}", err),
                        Some(timeout)
                        ).unwrap();
                }

            },
            _ => println!("Not available right now")
        }

        if let Some(ref device) = input_device {
            println!("Selected output device: {}", device.name().unwrap());
        }
        if let Some(ref device) = output_device {
            println!("Selected output device: {}", device.name().unwrap());
        }
    }




    // User Format Choice
    // Setup Recording (Selected Interface, Sample Rate, Bit Depth, Channels)
    // Start Recording
    // Create buffer
    // Use While loop to append incoming data to the buffer
    // Get user input to start or stop recording
    // Convert data to the format
    // Write Buffer to File

    // Playback:
    // Setup playback engine
    // Read file to buffer
    // Start Playback (for data chunk in audio data write to hardware)
    // Release Resources

}

