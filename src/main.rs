use dialoguer::{theme::ColorfulTheme, Select};
use cpal::traits::HostTrait;
use cpal::traits::DeviceTrait;
use cpal::platform::Host;


fn select_device(option: usize, host: Host){
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

            println!("You selected {}", devices[selection]);
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

            println!("You selected {}", devices[selection]);
        }
        _ => {
            println!("We are the heroes");
        }
    }
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

    let host: Host = cpal::default_host();
    let selection = main_menu();
    select_device(selection, host);
    // Initialize Audio API
    // Query the API - List Interfaces
    // Display Interface Options (for the user)
    // User Interface Choice (save that choice and initiliaze accordingly)
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

