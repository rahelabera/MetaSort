# MetaSort: Smart Google Photos Organizer 📸

[![Download MetaSort](https://img.shields.io/badge/Download_MetaSort-Release-brightgreen)](https://github.com/rahelabera/MetaSort/releases)

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Supported Formats](#supported-formats)
- [Installation](#installation)
- [Usage](#usage)
- [Commands](#commands)
- [Configuration](#configuration)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

---

## Overview

MetaSort is a powerful tool designed to help you organize your Google Photos library efficiently. It cleans files, extracts dates, embeds metadata, sorts photos by date, and generates detailed reports. With support for over 50 file formats, including RAW, MetaSort makes managing your photo collection a breeze.

Whether you're a professional photographer or a casual user, MetaSort automates the tedious tasks of photo management, allowing you to focus on what matters most: your memories.

---

## Features

- **Smart Organization**: Automatically sort photos by date.
- **Metadata Management**: Embed relevant metadata into your files.
- **File Cleaning**: Remove duplicates and unnecessary files.
- **Report Generation**: Create comprehensive reports of your photo collection.
- **Cross-Platform Support**: Works on macOS, Windows, and Linux.
- **Automation**: Streamline your workflow with command-line interface (CLI) capabilities.

---

## Supported Formats

MetaSort supports a wide range of file formats, ensuring compatibility with various photo types. Here are some of the key formats:

- JPEG
- PNG
- GIF
- TIFF
- RAW formats (CR2, NEF, ARW, etc.)
- And many more...

---

## Installation

To install MetaSort, follow these steps:

1. **Download the latest release** from the [Releases page](https://github.com/rahelabera/MetaSort/releases). 
2. Extract the downloaded file.
3. Follow the installation instructions specific to your operating system.

For example, on macOS or Windows, you may need to run an installer or execute a script to set up MetaSort.

---

## Usage

After installation, you can start using MetaSort from the command line. Here’s a simple guide to get you started:

1. Open your terminal or command prompt.
2. Navigate to the directory where MetaSort is installed.
3. Use the command `metasort` followed by your desired options.

For example:

```bash
metasort --sort-by-date
```

---

## Commands

MetaSort offers a variety of commands to enhance your photo management experience. Here are some of the most commonly used commands:

- `--sort-by-date`: Sorts photos based on their capture date.
- `--clean-files`: Removes duplicate and unnecessary files.
- `--generate-report`: Creates a report of your photo collection.
- `--embed-metadata`: Embeds metadata into selected files.

You can combine these commands for more complex operations. For example:

```bash
metasort --sort-by-date --clean-files --generate-report
```

---

## Configuration

MetaSort allows for customization through a configuration file. You can specify default settings such as file paths, sorting preferences, and metadata options. 

To create or edit your configuration file:

1. Locate the `config.toml` file in the MetaSort directory.
2. Modify the settings according to your preferences.

Here’s an example of a simple configuration:

```toml
[settings]
default_path = "/path/to/your/photos"
sort_by = "date"
embed_metadata = true
```

---

## Contributing

Contributions are welcome! If you would like to help improve MetaSort, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them.
4. Push your changes to your forked repository.
5. Submit a pull request.

Please ensure that your code adheres to the existing style and includes appropriate tests.

---

## License

MetaSort is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

---

## Contact

For any questions or feedback, please reach out to the project maintainers. You can find contact information in the repository or open an issue for support.

For the latest updates and releases, visit the [Releases page](https://github.com/rahelabera/MetaSort/releases).

--- 

Feel free to explore the project and enhance your photo management experience with MetaSort!