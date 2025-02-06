# CapsWitch

[![CapsWitch logo](./readme_assets/readme_logo.webp)](#capswitch)

If you got used to switching your keyboard layout with `CapsLock` on MacOS and
Linux, welcome **CapsWitch** to make your life easier on Windows.

CapsWitch is a lightweight, background, Rust-built Windows application that
simplifies keyboard layout switching by repurposing the CapsLock key. Default
`CapsLock` key behaviour is preserved with `Shift + CapsLock`.

You won't need to use `Alt + Shift`, `Win + Space` or `Ctrl + Shift` anymore\*.
Give it a try!

> \* Except for the applications run as administrator.

## Features

- **Tray menu**: A system tray icon provides easy access to settings.
- **Pause/Resume**: Temporarily disable CapsWitch and revert to the default
  `CapsLock` behavior.
- **Autoload on startup**: Enable this option to ensure CapsWitch launches
  automatically when you start your computer. With autoload enabled, Capswitch
  will also preserve your chosen switching mode.
- **Switching mode**: Choose between Windows default (circular) and app's
  `Previous` modes:
  - **Default** mode: cycles through all available keyboard layouts in order,
    just like Windows does by default.
  - **Previous** mode: switches only between the **two most recently used
    layouts**. This mode is particularly useful if you frequently switch between
    two layouts out of many available.

## Installation

> Important note
>
> CapsWitch is an open-source, non-commercial project. As such, I have not
> purchased an OV (Organization Validation) certificate to sign the executable.
> Instead, the app is signed with a
> [self-issued certificate](https://github.com/Linkerin/capswitch/blob/main/certificate.crt).
>
> Unfortunately, this means that Windows Defender or other security software may
> flag the app as potentially unsafe. If you encounter this issue, you will need
> to manually allow the app to run. Or build the executable from the source
> code.
>
> Rest assured, CapsWitch is completely safe to use. The source code is open for
> anyone to inspect, and no malicious behavior is present.

You can download the latest version of CapsWitch from the
[Releases](https://github.com/Linkerin/capswitch/releases) page. The following
options are available:

- MSIX installer;
- Portable EXE;
- Source code.

## Building from source

If you prefer to build CapsWitch from source, follow these steps:

1. Install Rust by following the instructions on the
   [official Rust website](https://www.rust-lang.org/tools/install).
2. Clone the CapsWitch repository:

   ```sh
   git clone https://github.com/Linkerin/capswitch.git
   ```

3. Build the App:

   ```sh
   cargo build --release
   ```

   The compiled binary will be located in the target/release directory.

## Contributing

Found a bug or have a feature idea? Feel free to open an
[issue](https://github.com/Linkerin/capswitch/issues) or submit a pull request.
Contributions are always welcome!

## License

**CapsWitch** is licensed under the MIT License. See the
[LICENSE](https://github.com/Linkerin/capswitch/blob/main/LICENSE) file for
details.
