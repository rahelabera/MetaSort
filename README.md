# <img src="https://i.ibb.co/67J3jp0p/upi.png" alt="Donate UPI" width="120" align="left" />

# MetaSort v1.0.0 – Google Photos Takeout Organizer

[![Donate via UPI](https://img.shields.io/badge/Donate-UPI-blue?logo=googlepay&style=for-the-badge)](https://upier.vercel.app/pay/sanmith@superyes)

---

<p align="center">
  <img src="https://i.ibb.co/67J3jp0p/upi.png" alt="UPI QR" width="140" style="margin-bottom: 0.5em;"/>
</p>

<h3 align="center">
  🚀✨ <b>MetaSort</b> is your all-in-one, lightning-fast, emoji-powered tool for <br> decluttering and organizing your <b>Google Photos Takeout</b> (or any messy media folder) on macOS!<br>
  <br>
  🧹 Cleans, 📦 sorts, 🏷️ embeds metadata, and 📑 generates beautiful reports—all in one go!
</h3>

---

**MetaSort** is a blazing-fast, user-friendly tool for organizing your Google Photos Takeout exports (or any messy media folder). It cleans, sorts, embeds metadata, and generates beautiful reports—all in one go!

---

## 🚀 Installation (macOS)

1. **Install Rust (if not already):**
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   # Follow the prompts, then restart your terminal
   ```
2. **Install exiftool (required):**
   ```sh
   brew install exiftool
   ```
   > MetaSort requires [exiftool](https://exiftool.org/) for metadata extraction and embedding.
3. **Clone this repo:**
   ```sh
   git clone https://github.com/iamsanmith/MetaSort.git
   cd MetaSort_v1.0.0
   ```
4. **Build the project:**
   ```sh
   cargo build --release
   ```
5. **Run MetaSort:**
   ```sh
   cargo run --release
   ```

> **Note:** MetaSort is designed for macOS. Linux may work, but is untested. Windows is not supported. 

---

## 💡 Features

- 🧹 **Cleans .json Filenames and organizes** your Google Photos Takeout or any media folder
- 💬 **Separates WhatsApp and Screenshot images** (optional)
- 🏷️ **Corrects file types and names** (even if Google/your phone got it wrong)
- 🔍 **Extracts all available metadata** from JSON (date, camera, GPS, and more)
- ✍️ **Embeds original date, camera, GPS, etc.** into your photos/videos
- 📦 **Sorts everything into neat folders** by year/month/type
- 📑 **Generates CSV and HTML summary reports**
- 📊 **Emoji-rich logs and progress bars** for every step
- 🛠️ **Modular, maintainable codebase**

---

## 🖥️ Usage

1. **Run MetaSort:**
   ```sh
   cargo run --release
   ```
2. **Follow the prompts:**
   - Drag and drop your Google Photos Takeout folder (or any media folder).
   - Choose whether to separate WhatsApp/Screenshot images.
   - Choose how to embed metadata (from JSON or filename for WhatsApp/Screenshot).
   - Specify an output directory for sorted files and reports.
3. **Check your output:**
   - Sorted media in `MetaSort_Output/Media Files/`
   - Technical files, logs, and CSVs in `MetaSort_Output/Technical Files/`
   - Beautiful HTML summary report at the end

---

## 🗂️ Project Structure

```
MetaSort_v1.0.0/
│
├── src/
│   ├── main.rs                # Orchestrates the workflow
│   ├── media_cleaning.rs      # Cleaning, WhatsApp/Screenshot separation, type correction
│   ├── metadata_extraction.rs # Extracts metadata from JSON
│   ├── metadata_embed.rs      # Embeds metadata into media files
│   ├── sort_to_folders.rs     # Sorts files and generates CSVs
│   ├── csv_report.rs          # CSV report generation
│   ├── html_report.rs         # HTML summary report generation
│   └── utils.rs               # Logging and helpers
├── assets/
│   └── upi.png                # UPI QR for donations
├── Cargo.toml
├── README.md
```

---

## 🗓️ Supported Filename Date Guessing Patterns

MetaSort can extract dates from a huge variety of filename formats, including those from popular phones, cameras, and apps such as:

- **WhatsApp:**
  - `IMG-20220101-WA0001.jpg` → `2022:01:01 00:00:00`
  - `WhatsApp Image 2022-01-01 at 12.34.56.jpg` → `2022:01:01 12:34:56`
- **Screenshots:**
  - `Screenshot_2023-01-01-12-00-00.png` → `2023:01:01 12:00:00`
  - `Screen Shot 2023-01-01 at 12.00.00.png` → `2023:01:01 12:00:00`
  - `Screenshot_20230101-120000.png` → `2023:01:01 12:00:00`
  - `Screenshot_2023-01-01-12-34-56-123.png` → `2023:01:01 12:34:56`
- **Samsung/Google/Android:**
  - `20230101_123456.jpg` → `2023:01:01 12:34:56`
  - `2023-01-01 12.34.56.jpg` → `2023:01:01 12:34:56`
  - `20230101-123456.jpg` → `2023:01:01 12:34:56`
  - `2023-01-01_12-34-56.jpg` → `2023:01:01 12:34:56`
  - `2023.01.01_12.34.56.jpg` → `2023:01:01 12:34:56`
  - `2023_01_01_12_34_56.jpg` → `2023:01:01 12:34:56`
- **Google Photos/PXL:**
  - `PXL_20230101_123456789.jpg` → `2023:01:01 12:34:56`
- **Google/Android Video:**
  - `VID_20230101_123456.mp4` → `2023:01:01 12:34:56`
- **Telegram:**
  - `photo_2023-01-01 12.00.00.jpg` → `2023:01:01 12:00:00`
- **MIUI:**
  - `IMG_20230101_120000.jpg` → `2023:01:01 12:00:00`
- **Sony Camera:**
  - `DSC01234_20230101_123456.JPG` → `2023:01:01 12:34:56`
  - `DSC_20230101_123456.JPG` → `2023:01:01 12:34:56`
- **Custom/Other:**
  - `RMLmc20250531_115820_RMlmc.7` → `2025:05:31 11:58:20`
  - `wallpaper - IMG_20240113_143213Jan 13 2024` → `2024:01:13 14:32:13`
  - `San-1 Oct 2024.jxl` → `2024:10:01 00:00:00`

...and many more! If your filename contains a date, MetaSort will likely find it. 🎯

If no date is found in the filename, MetaSort will fall back to JSON metadata or mark as "Unknown Time".

---

## 🙏 Support & Donations

If MetaSort saved you hours, consider buying me a coffee through donating via UPI! Your support helps keep this project alive and free.

<p align="center">
  <img src="https://i.ibb.co/67J3jp0p/upi.png" alt="UPI QR" width="200" />
  <br/>
  <a href="https://upier.vercel.app/pay/sanmith@superyes"><b>Donate via UPI</b></a>
</p>

---

## 📣 Suggestions & Contributions

- Found a bug? Have a feature request? Open an issue or PR!
- Want to add more metadata fields, analytics, or a GUI? Contributions are welcome.

---

**MetaSort v1** © 2025 Sanmith S. All rights reserved. 