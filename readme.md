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

<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
<link type="text/css" rel="stylesheet" href="resources/sheet.css" >
<style type="text/css">.ritz .waffle a { color: inherit; }.ritz .waffle .s31{border-bottom:1px SOLID #000000;background-color:#ffff00;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s10{border-bottom:1px SOLID #000000;background-color:#7f7f7f;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s43{border-bottom:1px SOLID #000000;background-color:#142b00;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s21{border-bottom:1px SOLID #000000;background-color:#ffbd6c;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s68{border-bottom:1px SOLID #000000;background-color:#436400;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s5{border-bottom:1px SOLID #000000;border-right:1px SOLID #000000;background-color:#ffffff;text-align:right;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s12{border-bottom:1px SOLID #000000;background-color:#001912;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s8{border-bottom:1px SOLID #000000;background-color:#1e1e1e;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s38{border-bottom:1px SOLID #000000;background-color:#874cff;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s52{border-bottom:1px SOLID #000000;background-color:#190019;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s61{border-bottom:1px SOLID #000000;background-color:#4cff88;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s47{border-bottom:1px SOLID #000000;background-color:#00ff00;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s53{border-bottom:1px SOLID #000000;background-color:#4cff5e;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s2{border-bottom:1px SOLID #000000;background-color:#ffffff;text-align:right;color:#000000;font-family:'docs-sans-serif',Arial;font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s24{border-bottom:1px SOLID #000000;background-color:#0055ff;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s66{border-bottom:1px SOLID #000000;background-color:#795100;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s54{border-bottom:1px SOLID #000000;background-color:#ff4c87;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s23{border-bottom:1px SOLID #000000;background-color:#ff5400;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s62{border-bottom:1px SOLID #000000;background-color:#ff1500;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s20{border-bottom:1px SOLID #000000;background-color:#001019;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s30{border-bottom:1px SOLID #000000;background-color:#4c4cff;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s41{border-bottom:1px SOLID #000000;background-color:#1d5900;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s26{border-bottom:1px SOLID #000000;background-color:#001d59;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s58{border-bottom:1px SOLID #000000;background-color:#59001d;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s3{border-bottom:1px SOLID #000000;background-color:#000000;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s6{border-bottom:1px SOLID #000000;background-color:#ffffff;text-align:left;color:#000000;font-family:'docs-sans-serif',Arial;font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s36{border-bottom:1px SOLID #000000;background-color:#000019;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s28{border-bottom:1px SOLID #000000;background-color:#000819;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s18{border-bottom:1px SOLID #000000;background-color:#004152;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s56{border-bottom:1px SOLID #000000;background-color:#ff0054;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s1{border-bottom:1px SOLID #000000;border-right:1px SOLID #000000;background-color:#ffffff;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s37{border-bottom:1px SOLID #000000;background-color:#88ff4c;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s40{border-bottom:1px SOLID #000000;background-color:#5400ff;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s65{border-bottom:1px SOLID #000000;background-color:#00591d;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s7{border-bottom:1px SOLID #000000;background-color:#4cffb7;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s42{border-bottom:1px SOLID #000000;background-color:#190064;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s46{border-bottom:1px SOLID #000000;background-color:#ff4cff;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s51{border-bottom:1px SOLID #000000;background-color:#001900;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s44{border-bottom:1px SOLID #000000;background-color:#0f0030;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s19{border-bottom:1px SOLID #000000;background-color:#190000;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s32{border-bottom:1px SOLID #000000;background-color:#0000ff;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s25{border-bottom:1px SOLID #000000;background-color:#591d00;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s49{border-bottom:1px SOLID #000000;background-color:#005900;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s67{border-bottom:1px SOLID #000000;background-color:#001f12;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s15{border-bottom:1px SOLID #000000;background-color:#ff0000;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s14{border-bottom:1px SOLID #000000;background-color:#4cc3ff;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s50{border-bottom:1px SOLID #000000;background-color:#590059;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s22{border-bottom:1px SOLID #000000;background-color:#4c88ff;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s55{border-bottom:1px SOLID #000000;background-color:#00ff19;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s60{border-bottom:1px SOLID #000000;background-color:#220013;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s17{border-bottom:1px SOLID #000000;background-color:#590000;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s0{border-bottom:1px SOLID #000000;background-color:#ffffff;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s63{border-bottom:1px SOLID #000000;background-color:#00ff55;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s13{border-bottom:1px SOLID #000000;background-color:#ff4c4c;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s29{border-bottom:1px SOLID #000000;background-color:#ffff4c;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s9{border-bottom:1px SOLID #000000;background-color:#00ff99;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s39{border-bottom:1px SOLID #000000;background-color:#54ff00;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s11{border-bottom:1px SOLID #000000;background-color:#005935;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s45{border-bottom:1px SOLID #000000;background-color:#4cff4c;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s59{border-bottom:1px SOLID #000000;background-color:#001902;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s57{border-bottom:1px SOLID #000000;background-color:#00590d;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s27{border-bottom:1px SOLID #000000;background-color:#271b00;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s48{border-bottom:1px SOLID #000000;background-color:#ff00ff;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s16{border-bottom:1px SOLID #000000;background-color:#00a9ff;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s34{border-bottom:1px SOLID #000000;background-color:#000059;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s35{border-bottom:1px SOLID #000000;background-color:#191900;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s4{border-bottom:1px SOLID #000000;background-color:#ffffff;text-align:right;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s64{border-bottom:1px SOLID #000000;background-color:#993500;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}.ritz .waffle .s33{border-bottom:1px SOLID #000000;background-color:#595900;text-align:left;color:#000000;font-family:'Arial';font-size:10pt;vertical-align:bottom;white-space:nowrap;direction:ltr;padding:2px 3px 2px 3px;}
.column-headers-background {
    display: none;
}
.row-headers-background {
    display: none;
}
</style>

<div class="ritz grid-container" dir="ltr">
   <table class="waffle" cellspacing="0" cellpadding="0">
      <thead>
         <tr>
            <th class="row-header freezebar-origin-ltr"></th>
            <th id="0C0" style="width:82px;" class="column-headers-background">A</th>
            <th id="0C1" style="width:64px;" class="column-headers-background">B</th>
            <th id="0C2" style="width:75px;" class="column-headers-background">C</th>
            <th id="0C3" style="width:100px;" class="column-headers-background">D</th>
            <th id="0C4" style="width:100px;" class="column-headers-background">E</th>
            <th id="0C5" style="width:100px;" class="column-headers-background">F</th>
            <th id="0C6" style="width:100px;" class="column-headers-background">G</th>
            <th id="0C7" style="width:100px;" class="column-headers-background">H</th>
            <th id="0C8" style="width:100px;" class="column-headers-background">I</th>
            <th id="0C9" style="width:100px;" class="column-headers-background">J</th>
         </tr>
      </thead>
      <tbody>
         <tr style="height: 31px">
            <th id="0R0" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">1</div>
            </th>
            <td class="s0" dir="ltr">color Hex</td>
            <td class="s0" dir="ltr">velocity</td>
            <td class="s0" dir="ltr">color</td>
            <td class="s0" dir="ltr">qlcplus Off</td>
            <td class="s1" dir="ltr">qlc_plus ON</td>
            <td class="s0" dir="ltr">color Hex</td>
            <td class="s0" dir="ltr">velocity</td>
            <td class="s0" dir="ltr">color</td>
            <td class="s0" dir="ltr">qlcplus Off</td>
            <td class="s0" dir="ltr">qlc_plus ON</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R1" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">2</div>
            </th>
            <td class="s0" dir="ltr">#000000</td>
            <td class="s2" dir="ltr">0</td>
            <td class="s3"></td>
            <td class="s4">0</td>
            <td class="s5">128</td>
            <td class="s6" dir="ltr">#4CFFB7</td>
            <td class="s2" dir="ltr">32</td>
            <td class="s7"></td>
            <td class="s4">64</td>
            <td class="s4">192</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R2" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">3</div>
            </th>
            <td class="s0" dir="ltr">#1E1E1E</td>
            <td class="s2" dir="ltr">1</td>
            <td class="s8"></td>
            <td class="s4">2</td>
            <td class="s5">130</td>
            <td class="s6" dir="ltr">#00FF99</td>
            <td class="s2" dir="ltr">33</td>
            <td class="s9"></td>
            <td class="s4">66</td>
            <td class="s4">194</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R3" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">4</div>
            </th>
            <td class="s0" dir="ltr">#7F7F7F</td>
            <td class="s2" dir="ltr">2</td>
            <td class="s10"></td>
            <td class="s4">4</td>
            <td class="s5">132</td>
            <td class="s6" dir="ltr">#005935</td>
            <td class="s2" dir="ltr">34</td>
            <td class="s11"></td>
            <td class="s4">68</td>
            <td class="s4">196</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R4" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">5</div>
            </th>
            <td class="s0" dir="ltr">#FFFFFF</td>
            <td class="s2" dir="ltr">3</td>
            <td class="s0"></td>
            <td class="s4">6</td>
            <td class="s5">134</td>
            <td class="s6" dir="ltr">#001912</td>
            <td class="s2" dir="ltr">35</td>
            <td class="s12"></td>
            <td class="s4">70</td>
            <td class="s4">198</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R5" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">6</div>
            </th>
            <td class="s0" dir="ltr">#FF4C4C</td>
            <td class="s2" dir="ltr">4</td>
            <td class="s13"></td>
            <td class="s4">8</td>
            <td class="s5">136</td>
            <td class="s6" dir="ltr">#4CC3FF</td>
            <td class="s2" dir="ltr">36</td>
            <td class="s14"></td>
            <td class="s4">72</td>
            <td class="s4">200</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R6" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">7</div>
            </th>
            <td class="s0" dir="ltr">#FF0000</td>
            <td class="s2" dir="ltr">5</td>
            <td class="s15"></td>
            <td class="s4">10</td>
            <td class="s5">138</td>
            <td class="s6" dir="ltr">#00A9FF</td>
            <td class="s2" dir="ltr">37</td>
            <td class="s16"></td>
            <td class="s4">74</td>
            <td class="s4">202</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R7" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">8</div>
            </th>
            <td class="s6" dir="ltr">#590000</td>
            <td class="s2" dir="ltr">6</td>
            <td class="s17"></td>
            <td class="s4">12</td>
            <td class="s5">140</td>
            <td class="s6" dir="ltr">#004152</td>
            <td class="s2" dir="ltr">38</td>
            <td class="s18"></td>
            <td class="s4">76</td>
            <td class="s4">204</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R8" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">9</div>
            </th>
            <td class="s6" dir="ltr">#190000</td>
            <td class="s2" dir="ltr">7</td>
            <td class="s19"></td>
            <td class="s4">14</td>
            <td class="s5">142</td>
            <td class="s6" dir="ltr">#001019</td>
            <td class="s2" dir="ltr">39</td>
            <td class="s20"></td>
            <td class="s4">78</td>
            <td class="s4">206</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R9" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">10</div>
            </th>
            <td class="s6" dir="ltr">#FFBD6C</td>
            <td class="s2" dir="ltr">8</td>
            <td class="s21"></td>
            <td class="s4">16</td>
            <td class="s5">144</td>
            <td class="s6" dir="ltr">#4C88FF</td>
            <td class="s2" dir="ltr">40</td>
            <td class="s22"></td>
            <td class="s4">80</td>
            <td class="s4">208</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R10" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">11</div>
            </th>
            <td class="s6" dir="ltr">#FF5400</td>
            <td class="s2" dir="ltr">9</td>
            <td class="s23"></td>
            <td class="s4">18</td>
            <td class="s5">146</td>
            <td class="s6" dir="ltr">#0055FF</td>
            <td class="s2" dir="ltr">41</td>
            <td class="s24"></td>
            <td class="s4">82</td>
            <td class="s4">210</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R11" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">12</div>
            </th>
            <td class="s6" dir="ltr">#591D00</td>
            <td class="s2" dir="ltr">10</td>
            <td class="s25"></td>
            <td class="s4">20</td>
            <td class="s5">148</td>
            <td class="s6" dir="ltr">#001D59</td>
            <td class="s2" dir="ltr">42</td>
            <td class="s26"></td>
            <td class="s4">84</td>
            <td class="s4">212</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R12" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">13</div>
            </th>
            <td class="s6" dir="ltr">#271B00</td>
            <td class="s2" dir="ltr">11</td>
            <td class="s27"></td>
            <td class="s4">22</td>
            <td class="s5">150</td>
            <td class="s6" dir="ltr">#000819</td>
            <td class="s2" dir="ltr">43</td>
            <td class="s28"></td>
            <td class="s4">86</td>
            <td class="s4">214</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R13" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">14</div>
            </th>
            <td class="s6" dir="ltr">#FFFF4C</td>
            <td class="s2" dir="ltr">12</td>
            <td class="s29"></td>
            <td class="s4">24</td>
            <td class="s5">152</td>
            <td class="s6" dir="ltr">#4C4CFF</td>
            <td class="s2" dir="ltr">44</td>
            <td class="s30"></td>
            <td class="s4">88</td>
            <td class="s4">216</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R14" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">15</div>
            </th>
            <td class="s6" dir="ltr">#FFFF00</td>
            <td class="s2" dir="ltr">13</td>
            <td class="s31"></td>
            <td class="s4">26</td>
            <td class="s5">154</td>
            <td class="s6" dir="ltr">#0000FF</td>
            <td class="s2" dir="ltr">45</td>
            <td class="s32"></td>
            <td class="s4">90</td>
            <td class="s4">218</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R15" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">16</div>
            </th>
            <td class="s6" dir="ltr">#595900</td>
            <td class="s2" dir="ltr">14</td>
            <td class="s33"></td>
            <td class="s4">28</td>
            <td class="s5">156</td>
            <td class="s6" dir="ltr">#000059</td>
            <td class="s2" dir="ltr">46</td>
            <td class="s34"></td>
            <td class="s4">92</td>
            <td class="s4">220</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R16" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">17</div>
            </th>
            <td class="s6" dir="ltr">#191900</td>
            <td class="s2" dir="ltr">15</td>
            <td class="s35"></td>
            <td class="s4">30</td>
            <td class="s5">158</td>
            <td class="s6" dir="ltr">#000019</td>
            <td class="s2" dir="ltr">47</td>
            <td class="s36"></td>
            <td class="s4">94</td>
            <td class="s4">222</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R17" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">18</div>
            </th>
            <td class="s6" dir="ltr">#88FF4C</td>
            <td class="s2" dir="ltr">16</td>
            <td class="s37"></td>
            <td class="s4">32</td>
            <td class="s5">160</td>
            <td class="s6" dir="ltr">#874CFF</td>
            <td class="s2" dir="ltr">48</td>
            <td class="s38"></td>
            <td class="s4">96</td>
            <td class="s4">224</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R18" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">19</div>
            </th>
            <td class="s6" dir="ltr">#54FF00</td>
            <td class="s2" dir="ltr">17</td>
            <td class="s39"></td>
            <td class="s4">34</td>
            <td class="s5">162</td>
            <td class="s6" dir="ltr">#5400FF</td>
            <td class="s2" dir="ltr">49</td>
            <td class="s40"></td>
            <td class="s4">98</td>
            <td class="s4">226</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R19" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">20</div>
            </th>
            <td class="s6" dir="ltr">#1D5900</td>
            <td class="s2" dir="ltr">18</td>
            <td class="s41"></td>
            <td class="s4">36</td>
            <td class="s5">164</td>
            <td class="s6" dir="ltr">#190064</td>
            <td class="s2" dir="ltr">50</td>
            <td class="s42"></td>
            <td class="s4">100</td>
            <td class="s4">228</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R20" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">21</div>
            </th>
            <td class="s6" dir="ltr">#142B00</td>
            <td class="s2" dir="ltr">19</td>
            <td class="s43"></td>
            <td class="s4">38</td>
            <td class="s5">166</td>
            <td class="s6" dir="ltr">#0F0030</td>
            <td class="s2" dir="ltr">51</td>
            <td class="s44"></td>
            <td class="s4">102</td>
            <td class="s4">230</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R21" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">22</div>
            </th>
            <td class="s6" dir="ltr">#4CFF4C</td>
            <td class="s2" dir="ltr">20</td>
            <td class="s45"></td>
            <td class="s4">40</td>
            <td class="s5">168</td>
            <td class="s6" dir="ltr">#FF4CFF</td>
            <td class="s2" dir="ltr">52</td>
            <td class="s46"></td>
            <td class="s4">104</td>
            <td class="s4">232</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R22" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">23</div>
            </th>
            <td class="s6" dir="ltr">#00FF00</td>
            <td class="s2" dir="ltr">21</td>
            <td class="s47"></td>
            <td class="s4">42</td>
            <td class="s5">170</td>
            <td class="s6" dir="ltr">#FF00FF</td>
            <td class="s2" dir="ltr">53</td>
            <td class="s48"></td>
            <td class="s4">106</td>
            <td class="s4">234</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R23" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">24</div>
            </th>
            <td class="s6" dir="ltr">#005900</td>
            <td class="s2" dir="ltr">22</td>
            <td class="s49"></td>
            <td class="s4">44</td>
            <td class="s5">172</td>
            <td class="s6" dir="ltr">#590059</td>
            <td class="s2" dir="ltr">54</td>
            <td class="s50"></td>
            <td class="s4">108</td>
            <td class="s4">236</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R24" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">25</div>
            </th>
            <td class="s6" dir="ltr">#001900</td>
            <td class="s2" dir="ltr">23</td>
            <td class="s51"></td>
            <td class="s4">46</td>
            <td class="s5">174</td>
            <td class="s6" dir="ltr">#190019</td>
            <td class="s2" dir="ltr">55</td>
            <td class="s52"></td>
            <td class="s4">110</td>
            <td class="s4">238</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R25" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">26</div>
            </th>
            <td class="s6" dir="ltr">#4CFF5E</td>
            <td class="s2" dir="ltr">24</td>
            <td class="s53"></td>
            <td class="s4">48</td>
            <td class="s5">176</td>
            <td class="s6" dir="ltr">#FF4C87</td>
            <td class="s2" dir="ltr">56</td>
            <td class="s54"></td>
            <td class="s4">112</td>
            <td class="s4">240</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R26" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">27</div>
            </th>
            <td class="s6" dir="ltr">#00FF19</td>
            <td class="s2" dir="ltr">25</td>
            <td class="s55"></td>
            <td class="s4">50</td>
            <td class="s5">178</td>
            <td class="s6" dir="ltr">#FF0054</td>
            <td class="s2" dir="ltr">57</td>
            <td class="s56"></td>
            <td class="s4">114</td>
            <td class="s4">242</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R27" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">28</div>
            </th>
            <td class="s6" dir="ltr">#00590D</td>
            <td class="s2" dir="ltr">26</td>
            <td class="s57"></td>
            <td class="s4">52</td>
            <td class="s5">180</td>
            <td class="s6" dir="ltr">#59001D</td>
            <td class="s2" dir="ltr">58</td>
            <td class="s58"></td>
            <td class="s4">116</td>
            <td class="s4">244</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R28" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">29</div>
            </th>
            <td class="s6" dir="ltr">#001902</td>
            <td class="s2" dir="ltr">27</td>
            <td class="s59"></td>
            <td class="s4">54</td>
            <td class="s5">182</td>
            <td class="s6" dir="ltr">#220013</td>
            <td class="s2" dir="ltr">59</td>
            <td class="s60"></td>
            <td class="s4">118</td>
            <td class="s4">246</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R29" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">30</div>
            </th>
            <td class="s6" dir="ltr">#4CFF88</td>
            <td class="s2" dir="ltr">28</td>
            <td class="s61"></td>
            <td class="s4">56</td>
            <td class="s5">184</td>
            <td class="s6" dir="ltr">#FF1500</td>
            <td class="s2" dir="ltr">60</td>
            <td class="s62"></td>
            <td class="s4">120</td>
            <td class="s4">248</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R30" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">31</div>
            </th>
            <td class="s6" dir="ltr">#00FF55</td>
            <td class="s2" dir="ltr">29</td>
            <td class="s63"></td>
            <td class="s4">58</td>
            <td class="s5">186</td>
            <td class="s6" dir="ltr">#993500</td>
            <td class="s2" dir="ltr">61</td>
            <td class="s64"></td>
            <td class="s4">122</td>
            <td class="s4">250</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R31" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">32</div>
            </th>
            <td class="s6" dir="ltr">#00591D</td>
            <td class="s2" dir="ltr">30</td>
            <td class="s65"></td>
            <td class="s4">60</td>
            <td class="s5">188</td>
            <td class="s6" dir="ltr">#795100</td>
            <td class="s2" dir="ltr">62</td>
            <td class="s66"></td>
            <td class="s4">124</td>
            <td class="s4">252</td>
         </tr>
         <tr style="height: 31px">
            <th id="0R32" style="height: 31px;" class="row-headers-background">
               <div class="row-header-wrapper" style="line-height: 31px">33</div>
            </th>
            <td class="s6" dir="ltr">#001F12</td>
            <td class="s2" dir="ltr">31</td>
            <td class="s67"></td>
            <td class="s4">62</td>
            <td class="s5">190</td>
            <td class="s6" dir="ltr">#436400</td>
            <td class="s2" dir="ltr">63</td>
            <td class="s68"></td>
            <td class="s4">126</td>
            <td class="s4">254</td>
         </tr>
      </tbody>
   </table>
</div>
