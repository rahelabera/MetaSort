// csv_report.rs
// CSV report generation logic for MetaSort_v1.0.0 – Google Photos Takeout Organizer 

use std::path::Path;
use csv;

/// Write a CSV report for a given folder and set of files.
pub fn write_csv_report(
    folder: &Path,
    files: &[(String, String, String, String, String, u64)], // (FileName, Filetype, Original Time, Resolution, Human Size, Size)
    csv_name: &str,
) {
    let csv_path = folder.join(csv_name);
    let mut wtr = csv::Writer::from_path(&csv_path).expect("Failed to create CSV file");
    wtr.write_record(&["SL", "FileName", "Filetype", "Original Time", "File Resolution", "File Size", "Bytes"]).unwrap();
    for (i, (filename, filetype, orig_time, resolution, human_size, size)) in files.iter().enumerate() {
        wtr.write_record(&[
            (i + 1).to_string(),
            filename.to_string(),
            filetype.to_string(),
            orig_time.to_string(),
            resolution.to_string(),
            human_size.to_string(),
            size.to_string(),
        ]).unwrap();
    }
    wtr.flush().unwrap();
} 