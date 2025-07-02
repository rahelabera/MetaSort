// metadata_embed.rs
// Embedding metadata logic for MetaSort_v1.0.0 – Google Photos Takeout Organizer 

use std::process::Command;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use crate::metadata_extraction::MediaMetadata;
use crate::filename_date_guess::extract_date_from_filename;
use crate::utils::log_to_file;

pub fn embed_metadata_all(metadata_list: &[MediaMetadata], log_dir: &Path) {
    let logs_dir = log_dir.join("logs");
    let log_path = logs_dir.join("metadata_embedding.log");
    let _ = fs::create_dir_all(&logs_dir);
    let _log_file = File::create(&log_path).expect("Failed to create log file");
    println!("\n🧐Do you want to embed date/time for WhatsApp & Screenshot images based on their  \n1. Metadata\n2. Filename\n");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let use_filename = matches!(input.trim(), "2");
    let total = metadata_list.len();
    let mut processed = 0;
    for meta in metadata_list {
        let mut args = Vec::new();
        let filename = meta.media_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let parent = meta.media_path.parent().and_then(|p| p.file_name()).and_then(|n| n.to_str()).unwrap_or("");
        let is_wa = parent.eq_ignore_ascii_case("Whatsapp");
        let is_sc = parent.eq_ignore_ascii_case("Screenshots");
        let mut used = "metadata";
        let mut date_to_embed = meta.exif_date.clone();
        if use_filename && (is_wa || is_sc) {
            if let Some(date) = extract_date_from_filename(filename) {
                date_to_embed = Some(date);
                used = "filename";
            }
        }
        if date_to_embed.is_none() {
            used = "metadata (fallback)";
        }
        if let Some(ref date) = date_to_embed {
            if meta.media_path.extension().map(|e| e.to_ascii_lowercase()) == Some("png".into()) {
                args.push(format!("-XMP:DateTimeOriginal={}", date));
            } else {
                args.push(format!("-DateTimeOriginal={}", date));
            }
        }
        if let (Some(lat), Some(lon)) = (meta.gps_latitude, meta.gps_longitude) {
            args.push(format!("-GPSLatitude={}", lat));
            args.push(format!("-GPSLongitude={}", lon));
        }
        if let Some(alt) = meta.gps_altitude {
            args.push(format!("-GPSAltitude={}", alt));
        }
        if let Some(ref make) = meta.camera_make {
            args.push(format!("-Make={}", make));
        }
        if let Some(ref model) = meta.camera_model {
            args.push(format!("-Model={}", model));
        }
        // Add more fields as needed
        args.push("-overwrite_original".to_string());
        args.push(meta.media_path.to_string_lossy().to_string());
        let log_msg = format!(
            "File: {:?}, Used: {}, Date: {:?}, Lat: {:?}, Lon: {:?}, Alt: {:?}, Make: {:?}, Model: {:?}",
            meta.media_path.file_name().unwrap_or_default(), used, date_to_embed, meta.gps_latitude, meta.gps_longitude, meta.gps_altitude, meta.camera_make, meta.camera_model
        );
        let status = Command::new("exiftool")
            .args(&args)
            .status();
        if let Ok(status) = status {
            if status.success() {
                log_to_file(&logs_dir, "metadata_embedding.log", &format!("✅ Embedded metadata. {}", log_msg));
            } else {
                log_to_file(&logs_dir, "metadata_embedding.log", &format!("❌ Failed to embed metadata. {}", log_msg));
            }
        } else {
            log_to_file(&logs_dir, "metadata_embedding.log", &format!("❌ Error running exiftool. {}", log_msg));
        }
        processed += 1;
        print_progress(processed, total);
    }
    println!("\n✅ Metadata embedding complete! Embedded metadata for {} files. Log: {:?}", processed, log_path);
}

fn print_progress(done: usize, total: usize) {
    let percent = if total > 0 { (done * 100) / total } else { 100 };
    let bar = format!("{}{}", "🟦".repeat(percent / 4), "⬜".repeat(25 - percent / 4));
    print!("\r✍️  Embedding metadata: [{}] {}% ({} / {})", bar, percent, done, total);
    let _ = std::io::stdout().flush();
    if done == total {
        println!();
    }
} 