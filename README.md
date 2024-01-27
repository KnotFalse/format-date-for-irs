[![Build and Release](https://github.com/GavinSiver/format-date-for-irs/actions/workflows/rust.yml/badge.svg)](https://github.com/GavinSiver/format-date-for-irs/actions/workflows/rust.yml)

# Format Date for IRS

The `format-date-for-irs` Rust binary is a nimble and efficient tool designed to streamline the process of formatting dates on your clipboard to comply with IRS standards. In the realm of financial documentation and tax preparation, ensuring that dates are consistently formatted is crucial. This utility listens for any changes to your clipboard content, identifies date strings, and automatically reformats them into the IRS-approved format (mm/dd/yyyy), saving you time and reducing the risk of manual entry errors. Whether you're a professional in the finance sector or just someone who frequently deals with dates in a format-sensitive context, `format-date-for-irs` offers a seamless background service to enhance your workflow.


## Features

- **Automatic Clipboard Monitoring**: Continuously monitors the clipboard for changes, ensuring that any date copied is instantly detected.
- **Intelligent Date Detection**: Utilizes robust parsing to identify dates within the clipboard text, regardless of the initial format.
- **Seamless Date Formatting**: Reformats recognized dates to the mm/dd/yyyy format, aligning with IRS requirements, and updates the clipboard with the formatted date.
- **Background Operation**: Runs quietly in the background, allowing you to proceed with your tasks without interruption.
- **Error Handling**: Gracefully handles parsing errors, ensuring the application's stability and reliability.


## Installation

Before installing `format-date-for-irs`, ensure you have the Rust toolchain installed on your system, as it's necessary for compiling Rust projects. You can download and install Rust, including Cargo (Rust's package manager and build tool), from the [official Rust website](https://www.rust-lang.org/tools/install).

### Compiling from Source

To compile `format-date-for-irs` from source:

1. Clone the repository to your local machine:
    ```
    git clone https://github.com/yourusername/format-date-for-irs.git
    ```
2. Navigate into the project directory:
    ```
    cd format-date-for-irs
    ```
3. Compile the project using Cargo:
    ```
    cargo build --release
    ```
4. The compiled binary will be located in `./target/release/`. You can run it directly or move it to a location in your system's PATH for easier access.


## Usage

Once `format-date-for-irs` is installed and running, it will automatically monitor your clipboard for any date strings. When it detects a date, it will format it to the mm/dd/yyyy format and update the clipboard with the formatted date.

To start `format-date-for-irs`, simply run the binary from the command line:

```
./format-date-for-irs
```

The program will run in the background. Ensure it remains active for continuous clipboard monitoring.
