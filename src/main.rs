use dialoguer::{theme::ColorfulTheme, Select};
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use cpal::*;
use cpal::platform::Host;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use hound::{WavWriter, WavSpec};
use rfd::FileDialog;
use std::path::{PathBuf, Path};
use std::env;

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
    let mut buffer_sizes = Vec::new();

    for config in supported_configs_range {
        sample_rates.push(config.min_sample_rate().0); //min sample rate
        sample_rates.push(config.max_sample_rate().0); //max sample rate
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
        "Play Audio",
        "Exit"
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select and option")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    return selection

}

fn save_to_wav(file_name: &str, specs: StreamConfig, samples: &[f32]) {
    let spec = WavSpec {
        channels: specs.channels,
        sample_rate: specs.sample_rate.0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(file_name, spec).unwrap();

    for &sample in samples.iter() {
        let amplitude = i16::MAX as f32;
        let sample = (sample * amplitude) as i16;
        writer.write_sample(sample).unwrap();
    }

    writer.finalize().unwrap();

}


fn select_location() -> String {
    let file_path = FileDialog::new()
        .set_title("Select a saving directory")
        .set_directory("/")
        .pick_folder();

    let file_path = match file_path {
        Some(path) => path,
        None => env::current_dir().expect("Failed to get current directory")
    };

    println!("Enter file name: ");
    let mut file_name = String::new();
    std::io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read line");
    let file_name = file_name.trim();

    let full_path = format!("{}/{}.wav", file_path.display(), file_name);

    full_path

}
fn select_file() -> Option<PathBuf> {

    let file = FileDialog::new()
        .set_title("Select a file")
        .pick_file();

    match file {
        Some(path) => {
            return Some(path);
        },
        None => {
            println!("No file selected");
            None
        }
    }
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
                    let timeout: Duration = Duration::from_secs(5);

                    let stream = device.build_input_stream(
                        &config,
                        move |data: &[f32], _: &InputCallbackInfo| {
                            let mut buffer = buffer_clone.lock().unwrap();
                            buffer.extend_from_slice(data);
                            },
                        |err| eprintln!("An error occured on the input audio stream: {}", err),
                            Some(timeout)
                            ).unwrap();

                    stream.play().unwrap();
                    println!("Recording audio...");

                    std::thread::sleep(std::time::Duration::from_secs(1));

                    println!("Press Q to stop recording");

                    loop {
                        let mut selection = String::new();
                        std::io::stdin()
                            .read_line(&mut selection)
                            .expect("Failed to read line");
                        let selection = selection.trim().to_uppercase();
                        if selection == "Q" {
                            stream.pause().unwrap();
                            println!("Stopping recording...");
                            std::thread::sleep(std::time::Duration::from_secs(1));
                            break;
                        } else {
                            println!("Not a valid option");
                        }
                    }
                    // Prompt user for name of file
                    let file_name = select_location();
                    // Convert to wav
                    let recording = audio_buffer.lock().unwrap();
                    save_to_wav(&file_name, config, &recording);
                    // Save to location in computer

                }
            },
            3 => {
                if let Some(ref device) = output_device {
                    let config = device.default_output_config().unwrap();

                    let config =  StreamConfig {
                        channels: config.channels(),
                        sample_rate: config.sample_rate(),
                        buffer_size: BufferSize::Fixed(256),
                    };

                    let file_path = select_file();

                    if let Some(path_buf) = file_path {
                        let path: &Path = path_buf.as_path();
                        let reader = hound::WavReader::open(path).unwrap();
                        let spec = reader.spec();
                        println!("Sample rate: {}", spec.sample_rate);
                        println!("Number of channels: {}", spec.channels);

                        let samples: Vec<f32> = reader.into_samples::<i16>()
                            .map(|s| s.unwrap() as f32 / i16::MAX as f32)
                            .collect();

                        let audio_buffer = Arc::new(Mutex::new(samples));
                        let mut sample_clock = 0usize;

                        let stream = device.build_output_stream(
                            &config,
                            move |data: &mut [f32], _: &OutputCallbackInfo| {
                                let buffer = audio_buffer.lock().unwrap();
                                for frame in data.chunks_mut(config.channels as usize) {
                                    let sample = buffer[sample_clock];
                                    sample_clock = (sample_clock + 1) % buffer.len();
                                    for sample_out in frame.iter_mut() {
                                        *sample_out = sample;
                                    }
                                }
                            },
                            |err| eprintln!("An error occured on the output audio stream: {}", err),
                            None
                        ).unwrap();

                        stream.play().unwrap();
                        println!("Playing audio...");

                        std::thread::sleep(std::time::Duration::from_secs(1));

                        println!("Press Q to stop playback");

                        loop {
                            let mut selection = String::new();
                            std::io::stdin()
                                .read_line(&mut selection)
                                .expect("Failed to read line");
                            let selection = selection.trim().to_uppercase();
                            if selection == "Q" {
                                stream.pause().unwrap();
                                println!("Stopping playback...");
                                std::thread::sleep(std::time::Duration::from_secs(1));
                                break;
                            } else {
                                println!("Not a valid option");
                            }
                        }
                    } else {
                        println!("No path provided");
                    }
                }

            },
            4 => std::process::exit(0),
            _ => println!("Not available right now")
        }

        if let Some(ref device) = output_device {
            println!("Selected output device: {}", device.name().unwrap());
        }
    }
}

