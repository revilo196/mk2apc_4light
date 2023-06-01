# mk2apc_4light

mk2apc_4light is a simple mapper designed to facilitate providing feedback to an AKAI APC mini mk2 MIDI controller when used in conjunction with QLC+. It receives feedback messages from QLC+ and adapts them to control the button LEDs on the AKAI APC mini mk2 controller.


## Prerequisites

- Linux operating system for midi `os::unix::VirtualInput`
- [Rust](https://www.rust-lang.org/) programming language
- [QLC+](https://qlcplus.org/) lighting control software

## Usage

1. Clone the repository:

   ```shell
   git clone https://github.com/revilo196/mk2apc_4light.git
   ```

2. Navigate to the project directory:

   ```shell
   cd mk2apc_4light
   ```

3. Configure the program:

   - Create a configuration file named `config.toml` in the project directory. Example content:

     ```toml
     midi_idx = 0
     ```

     This configuration file specifies the MIDI index for the AKAI APC mini mk2. Adjust the `midi_idx` value according to the desired MIDI input device. You can check the console log to identify the correct index.

4. Build and run the program:

   ```shell
   cargo run --release
   ```

   The program will start and wait for MIDI messages from QLC+.

5. Configure QLC+ to send feedback messages:

   - Open QLC+ and go to the "Input/Output Manager."
   - Under the "MIDI Input" tab, select a MIDI plugin and enable it.
   - Configure the plugin to send messages to the virtual MIDI input device created by `mk2apc_4light` (usually named "mk2apc_4light_port").

   Now, when QLC+ sends feedback messages, `mk2apc_4light` will receive and adapt them to control the button LEDs on the AKAI APC mini mk2 controller.

## Features

- Receives MIDI feedback messages from QLC+ and adapts them for the AKAI APC mini mk2 controller.
- Maps different types of feedback to specific MIDI channels for controlling button LEDs.
- Converts Note Off messages to Note On messages for consistent handling.
- Provides console output for debugging purposes.


## Default Mapping

The `apc_mk2_mapper` program provides a default mapping for the AKAI APC mini mk2 controller, which controls the button LEDs based on the feedback messages received from QLC+. The default mapping is as follows:

- **Outer Buttons**: The outer buttons of the controller, which are notes beyond the range of 88, are mapped to Channel 1. This mapping ensures that these buttons remain on Channel 1.

- **Button Blinking**: Buttons with a color value greater than 64 are mapped to Channel 9. The program subtracts 64 from the color value and maps it to Channel 9, making the button LEDs blink.

- **Normal Buttons**: All other buttons are mapped to Channel 7 to make them as bright as possible.

The program handles Note Off messages by converting them to Note On messages with the same parameters. This conversion ensures consistent handling of button states.

These default mappings can be modified and customized according to specific requirements by editing the `midi_callback` function in the source code.

## Customization

You can customize the mappings and behavior of mk2apc_4light by modifying the code in the midi_callback function within the `main.rs` file. The comments in the code provide explanations and guidance on how to modify the mappings for different MIDI messages.


## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgments

- This program utilizes the [midir](https://github.com/Boddlnagg/midir) and [wmidi](https://github.com/RustAudio/wmidi) Rust libraries for MIDI input and output.
- The [config](https://github.com/mehcode/config-rs) Rust library is used for reading the configuration file.


## Color Table

here follows a color table of usable color for refernce
[PDF](APC_MINI_mk2_qlcplus_colors.pdf) or [HTML](APC_MINI_mk2_qlcplus_colors.html) 