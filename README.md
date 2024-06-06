# Audio Device Selector and Recorder

This Rust program allows you to select audio input and output devices, record audio, and play audio files using the CPAL library for cross-platform audio. It also leverages other libraries such as Dialoguer for CLI interaction, Hound for WAV file handling, and RFD for file dialog management.

## Features

- **Select Input and Output Devices**: Choose your desired audio input and output devices.
- **Record Audio**: Record audio from the selected input device and save it as a WAV file.
- **Play Audio**: Play audio from a selected WAV file through the chosen output device.

## Dependencies

- [cpal](https://crates.io/crates/cpal) - A low-level cross-platform audio I/O library in Rust.
- [dialoguer](https://crates.io/crates/dialoguer) - CLI library for Rust that provides user input prompts.
- [hound](https://crates.io/crates/hound) - A WAV encoding/decoding library for Rust.
- [rfd](https://crates.io/crates/rfd) - A Rust file dialog library.

## Usage

### Prerequisites

1. Make sure you have Rust installed. If not, you can install it from [rustup.rs](https://rustup.rs).
2. Clone this repository or copy the provided code into your project.

### Build and Run

1. Add the required dependencies to your `Cargo.toml`:

    ```toml
    [dependencies]
    cpal = "0.11.0"
    dialoguer = "0.9.0"
    hound = "3.4.0"
    rfd = "0.9.0"
    ```

2. Build and run the project:

    ```sh
    cargo build
    cargo run
    ```

### How to Use

1. **Select Input Device**: Choose this option to list and select the audio input device.
2. **Select Output Device**: Choose this option to list and select the audio output device.
3. **Record Audio**: After selecting the input device, use this option to start recording. Press `Q` to stop recording. The program will prompt you to save the recorded audio as a WAV file.
4. **Play Audio**: After selecting the output device, use this option to play a WAV file. Press `Q` to stop playback.
5. **Exit**: Quit the application.

### Main Menu Options

```rust
let selections = &[
    "Select Input Device",
    "Select Output Device",
    "Record Audio",
    "Play Audio",
    "Exit"
];
```

## Code Overview

### Select Device

This function lists the available input or output devices and allows the user to select one:

```rust
fn select_device(option: usize, host: &Host) -> String
```

### Select Configuration

This function lets you choose the sample rate and buffer size for the selected device:

```rust
fn select_config(device: &Device) -> StreamConfig
```

### Main Menu

This function displays the main menu and handles user input:

```rust
fn main_menu() -> usize
```

### Save to WAV

This function saves recorded audio samples to a WAV file:

```rust
fn save_to_wav(file_name: &str, specs: StreamConfig, samples: &[f32])
```

### Select Location

This function prompts the user to select a directory and file name for saving recordings:

```rust
fn select_location() -> String
```

### Select File

This function prompts the user to select a WAV file for playback:

```rust
fn select_file() -> Option<PathBuf>
```

### Main Function

The `main` function initializes the audio API, handles user input from the main menu, and performs the corresponding actions:

```rust
fn main()
```

## Acknowledgements

- [CPAL](https://github.com/RustAudio/cpal) library for cross-platform audio.
- [Dialoguer](https://github.com/mitsuhiko/dialoguer) for the CLI prompts.
- [Hound](https://github.com/ruuda/hound) for WAV file handling.
- [RFD](https://github.com/PolyMeilex/rfd) for file dialog management.

Feel free to contribute by submitting issues or pull requests.

