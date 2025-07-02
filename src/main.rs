// main.rs
// Entry point for MetaSort_v1.0.0 – Google Photos Takeout Organizer

mod media_cleaning;
mod metadata_extraction;
mod metadata_embed;
mod sort_to_folders;
mod html_report;
mod utils;
mod csv_report;
mod filename_date_guess;

use std::io;
use std::path::PathBuf;
use std::fs;
use walkdir;
use fs_extra;

fn get_folder_size(path: &str) -> u64 {
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .map(|e| e.metadata().map(|m| m.len()).unwrap_or(0))
        .sum()
}

fn human_readable_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    match size {
        s if s >= GB => format!("{:.2} GB", s as f64 / GB as f64),
        s if s >= MB => format!("{:.2} MB", s as f64 / MB as f64),
        s if s >= KB => format!("{:.2} KB", s as f64 / KB as f64),
        _ => format!("{} B", size),
    }
}

fn main() {
    println!(r#"
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃                                                                       ┃
┃ ███╗   ███╗███████╗████████╗ █████╗ ███████╗ ██████╗ ██████╗ ████████╗┃
┃ ████╗ ████║██╔════╝╚══██╔══╝██╔══██╗██╔════╝██╔═══██╗██╔══██╗╚══██╔══╝┃
┃ ██╔████╔██║█████╗     ██║   ███████║███████╗██║   ██║██████╔╝   ██║   ┃
┃ ██║╚██╔╝██║██╔══╝     ██║   ██╔══██║╚════██║██║   ██║██╔══██╗   ██║   ┃
┃ ██║ ╚═╝ ██║███████╗   ██║   ██║  ██║███████║╚██████╔╝██║  ██║   ██║   ┃
┃ ╚═╝     ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝╚══════╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ┃
┃                                                                       ┃
┃              MetaSort - Google Photos Takeout Organizer!              ┃
┃                                                                       ┃
┃                           Version 1.0                                 ┃
┃                      Developed by Sanmith S                           ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛



"#);
    println!("\n📂 Please drag and drop your Google Photos Takeout folder here, or specify the folder path:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input_dir = input.trim();

    // Calculate input folder size and prompt for required space
    let folder_size = get_folder_size(input_dir);
    let required_space = folder_size * 3;
    println!("\nThe input folder is {}.", human_readable_size(folder_size));
    println!("MetaSort recommends at least {} of free space in the output location for safe operation.", human_readable_size(required_space));
    println!("Continue? (y/n)");
    let mut cont = String::new();
    io::stdin().read_line(&mut cont).expect("Failed to read line");
    if !matches!(cont.trim().to_lowercase().as_str(), "y" | "yes") {
        println!("Aborted by user.");
        return;
    }

    // Prompt for output folder
    println!("\n📁 Please specify the output folder where MetaSort should work (originals will be untouched):");
    let mut output = String::new();
    io::stdin().read_line(&mut output).expect("Failed to read line");
    let output_dir = PathBuf::from(output.trim());
    let temp_dir = output_dir.join("MetaSort_temp");

    // Copy input folder to MetaSort_temp in output directory
    println!("\n⏳ Copying input folder to working directory (MetaSort_temp)...");
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.copy_inside = true;
    fs_extra::dir::copy(input_dir, &temp_dir, &copy_options).expect("Failed to copy input folder to output working directory");
    println!("\n✅ Copy complete! All processing will happen in: {}", temp_dir.display());

    // 1. Clean and pair media files with their JSONs (fix weird JSON names)
    println!("\n🧹 Step 1: Cleaning and pairing media files with JSONs...");
    media_cleaning::clean_json_filenames(temp_dir.to_str().unwrap());
    println!("\n✅ JSON filename cleaning and pairing complete!");

    // 1b. Ask if WhatsApp/Screenshots should be separated
    println!("\nDo you want to separate WhatsApp and Screenshot images? (y/n)");
    let mut wa_sc_input = String::new();
    io::stdin().read_line(&mut wa_sc_input).expect("Failed to read line");
    let separate_wa_sc = matches!(wa_sc_input.trim().to_lowercase().as_str(), "y" | "yes");
    if separate_wa_sc {
        println!("✅ WhatsApp and Screenshot images will be sorted into their own folders by year/month.");
    } else {
        println!("ℹ️  WhatsApp and Screenshot images will be treated as regular photos.");
    }
    media_cleaning::ask_and_separate_whatsapp_screenshots(temp_dir.to_str().unwrap(), separate_wa_sc);

    // 2. Extract metadata from JSON and embed into media files
    println!("\n🔍 Step 2: Extracting metadata from JSON and embedding into media files...");
    let (metadata, failed_guess_paths) = metadata_extraction::extract_metadata(temp_dir.to_str().unwrap());
    metadata_embed::embed_metadata_all(&metadata, &temp_dir);
    println!("\n✅ Metadata extraction and embedding complete!");

    // 3. Sort files using the embedded metadata (DateTimeOriginal)
    println!("\n📦 Step 3: Sorting files using embedded metadata...");
    let final_output_dir = output_dir.join("MetaSort_Output");
    sort_to_folders::sort_files_to_folders(&temp_dir, &final_output_dir, &failed_guess_paths, separate_wa_sc);
    println!("\n🎉 All done! Check your output and logs for details.");

    // 4. Move technical folders into Technical Files
    let technical_dir = final_output_dir.join("Technical Files");
    let _ = fs::create_dir_all(&technical_dir);
    for folder in &["CSV Report", "Json Files", "logs"] {
        let src = final_output_dir.join(folder);
        let dst = technical_dir.join(folder);
        if src.exists() {
            let _ = fs_extra::dir::move_dir(&src, &dst, &fs_extra::dir::CopyOptions::new());
        }
    }

    // 5. HTML summary report
    let photos = count_files(&final_output_dir.join("Media Files/Photos"));
    let videos = count_files(&final_output_dir.join("Media Files/Videos"));
    let whatsapp = count_files(&final_output_dir.join("Media Files/Whatsapp"));
    let screenshots = count_files(&final_output_dir.join("Media Files/Screenshots"));
    let unknown = count_files(&final_output_dir.join("Media Files/Unknown Time"));
    let mkv = count_files(&final_output_dir.join("Media Files/mkv_files"));
    let total = photos + videos + whatsapp + screenshots + unknown + mkv;
    let errors = count_log_errors(&final_output_dir.join("Technical Files/logs"));
    let csv_files = vec!["photos.csv", "videos.csv", "unknown_time.csv", "mkv_files.csv"];
    let log_files = vec!["media_cleaning.log", "metadata_extraction.log", "metadata_embedding.log", "sorting.log"];
    let metadata_fields: Vec<&str> = if let Some(meta) = metadata.first() {
        let mut fields = vec!["media_path", "json_path"];
        if meta.exif_date.is_some() { fields.push("exif_date"); }
        if meta.gps_latitude.is_some() { fields.push("gps_latitude"); }
        if meta.gps_longitude.is_some() { fields.push("gps_longitude"); }
        if meta.gps_altitude.is_some() { fields.push("gps_altitude"); }
        if meta.camera_make.is_some() { fields.push("camera_make"); }
        if meta.camera_model.is_some() { fields.push("camera_model"); }
        fields
    } else {
        vec!["media_path", "json_path", "exif_date", "gps_latitude", "gps_longitude", "gps_altitude", "camera_make", "camera_model"]
    };
    html_report::generate_html_report(
        &final_output_dir,
        total, photos, videos, whatsapp, screenshots, unknown, mkv, errors,
        &csv_files, &log_files, &metadata_fields,
    );

    // Delete MetaSort_temp folder after all processing
    if temp_dir.exists() {
        match fs_extra::dir::remove(&temp_dir) {
            Ok(_) => println!("\n🗑️  Deleted temporary folder: {}", temp_dir.display()),
            Err(e) => println!("\n⚠️  Could not delete temporary folder: {} (Error: {})", temp_dir.display(), e),
        }
    }
    println!("\n💖 Like my work? Please consider donating: https://upier.vercel.app/pay/sanmith@superyes\n");
}

fn count_files(dir: &PathBuf) -> usize {
    walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .count()
}

fn count_log_errors(logs_dir: &PathBuf) -> usize {
    let mut errors = 0;
    if let Ok(entries) = fs::read_dir(logs_dir) {
        for entry in entries.flatten() {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                errors += content.matches("❌").count();
            }
        }
    }
    errors
} 