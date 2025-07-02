use regex::Regex;

/// Attempts to extract a date/time from a filename using common patterns.
/// Returns the date in EXIF format (YYYY:MM:DD HH:MM:SS) if found.
pub fn extract_date_from_filename(filename: &str) -> Option<String> {
    // WhatsApp: IMG-20220101-WA0001.jpg, WhatsApp Image 2022-01-01 at 12.34.56.jpg
    let wa1 = Regex::new(r"IMG-(\d{8})-WA\d+").unwrap();
    let wa2 = Regex::new(r"WhatsApp Image (\d{4})-(\d{2})-(\d{2}) at (\d{2})\.(\d{2})\.(\d{2})").unwrap();
    // Screenshot: Screenshot_2023-01-01-12-00-00.png, Screen Shot 2023-01-01 at 12.00.00.png
    let sc1 = Regex::new(r"Screenshot_(\d{4})-(\d{2})-(\d{2})-(\d{2})-(\d{2})-(\d{2})").unwrap();
    let sc2 = Regex::new(r"Screen Shot (\d{4})-(\d{2})-(\d{2}) at (\d{2})\.(\d{2})\.(\d{2})").unwrap();
    let sc3 = Regex::new(r"Screenshot_(\d{8})-(\d{6})").unwrap();
    let tg1 = Regex::new(r"photo_(\d{4})-(\d{2})-(\d{2}) (\d{2})\.(\d{2})\.(\d{2})").unwrap();
    let mi1 = Regex::new(r"IMG_(\d{8})_(\d{6})").unwrap();
    let dt1 = Regex::new(r"(\d{4})-(\d{2})-(\d{2})-(\d{6})").unwrap();
    let custom1 = Regex::new(r"[._-](\d{8})-(\d{4})").unwrap();
    // Samsung, Google, Canon, Sony, etc.
    let samsung1 = Regex::new(r"(\\d{8})_(\\d{6})").unwrap(); // 20230101_123456
    let samsung2 = Regex::new(r"(\\d{4})-(\\d{2})-(\\d{2}) (\\d{2})\\.(\\d{2})\\.(\\d{2})").unwrap(); // 2023-01-01 12.34.56
    let samsung3 = Regex::new(r"(\\d{8})-(\\d{6})").unwrap(); // 20230101-123456
    let samsung4 = Regex::new(r"(\\d{4})-(\\d{2})-(\\d{2})_(\\d{2})-(\\d{2})-(\\d{2})").unwrap(); // 2023-01-01_12-34-56
    let samsung5 = Regex::new(r"(\\d{4})\\.(\\d{2})\\.(\\d{2})_(\\d{2})\\.(\\d{2})\\.(\\d{2})").unwrap(); // 2023.01.01_12.34.56
    let samsung6 = Regex::new(r"(\\d{4})_(\\d{2})_(\\d{2})_(\\d{2})_(\\d{2})_(\\d{2})").unwrap(); // 2023_01_01_12_34_56
    let pxl = Regex::new(r"PXL_(\\d{8})_(\\d{6,})").unwrap(); // PXL_20230101_123456789
    let ms1 = Regex::new(r"(\\d{4})-(\\d{2})-(\\d{2})_(\\d{2})-(\\d{2})-(\\d{2})-\\d+").unwrap(); // 2023-01-01_12-34-56-123
    let ms2 = Regex::new(r"Screenshot_(\\d{4})-(\\d{2})-(\\d{2})-(\\d{2})-(\\d{2})-(\\d{2})-\\d+").unwrap(); // Screenshot_2023-01-01-12-34-56-123
    let vid = Regex::new(r"VID_(\\d{8})_(\\d{6})").unwrap(); // VID_20230101_123456
    // Sony: DSC01234_20230101_123456.JPG, DSC_20230101_123456.JPG
    let sony1 = Regex::new(r"DSC\\d+_(\\d{8})_(\\d{6})").unwrap();
    let sony2 = Regex::new(r"DSC_(\\d{8})_(\\d{6})").unwrap();
    // Custom: RMLmc20250531_115820_RMlmc.7
    let rmlmc = Regex::new(r"RMLmc(\\d{8})_(\\d{6})").unwrap();
    // Custom: wallpaper - IMG_20240113_143213Jan 13 2024
    let wallpaper = Regex::new(r"IMG_(\\d{8})_(\\d{6})Jan \\d{2} \\d{4}").unwrap();
    // Custom: San-1 Oct 2024.jxl
    let san1 = Regex::new(r"(\\d{1,2}) (Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec) (\\d{4})").unwrap();
    // WhatsApp
    if let Some(caps) = wa1.captures(filename) {
        let date = &caps[1];
        return Some(format!("{}:{}:{} 00:00:00", &date[0..4], &date[4..6], &date[6..8]));
    }
    if let Some(caps) = wa2.captures(filename) {
        return Some(format!("{}:{}:{} {}:{}:{}", &caps[1], &caps[2], &caps[3], &caps[4], &caps[5], &caps[6]));
    }
    // Screenshot
    if let Some(caps) = sc1.captures(filename) {
        return Some(format!("{}:{}:{} {}:{}:{}", &caps[1], &caps[2], &caps[3], &caps[4], &caps[5], &caps[6]));
    }
    if let Some(caps) = sc2.captures(filename) {
        return Some(format!("{}:{}:{} {}:{}:{}", &caps[1], &caps[2], &caps[3], &caps[4], &caps[5], &caps[6]));
    }
    if let Some(caps) = sc3.captures(filename) {
        let date = &caps[1];
        let time = &caps[2];
        return Some(format!("{}:{}:{} {}:{}:{}", &date[0..4], &date[4..6], &date[6..8], &time[0..2], &time[2..4], &time[4..6]));
    }
    // Telegram
    if let Some(caps) = tg1.captures(filename) {
        return Some(format!("{}:{}:{} {}:{}:{}", &caps[1], &caps[2], &caps[3], &caps[4], &caps[5], &caps[6]));
    }
    // MIUI
    if let Some(caps) = mi1.captures(filename) {
        let date = &caps[1];
        let time = &caps[2];
        return Some(format!("{}:{}:{} {}:{}:{}", &date[0..4], &date[4..6], &date[6..8], &time[0..2], &time[2..4], &time[4..6]));
    }
    // Generic date-time
    if let Some(caps) = dt1.captures(filename) {
        let date = format!("{}{}{}", &caps[1], &caps[2], &caps[3]);
        let time = &caps[4];
        return Some(format!("{}:{}:{} {}:{}:{}", &date[0..4], &date[4..6], &date[6..8], &time[0..2], &time[2..4], &time[4..6]));
    }
    // Custom
    if let Some(caps) = custom1.captures(filename) {
        let date = &caps[1];
        let time = &caps[2];
        return Some(format!("{}:{}:{} {}:{}:00", &date[0..4], &date[4..6], &date[6..8], &time[0..2], &time[2..4]));
    }
    // Try new patterns before fallback
    if let Some(caps) = samsung1.captures(filename) {
        let date = &caps[1];
        let time = &caps[2];
        return Some(format!("{}:{}:{} {}:{}:{}", &date[0..4], &date[4..6], &date[6..8], &time[0..2], &time[2..4], &time[4..6]));
    }
    if let Some(caps) = samsung2.captures(filename) {
        return Some(format!("{}:{}:{} {}:{}:{}", &caps[1], &caps[2], &caps[3], &caps[4], &caps[5], &caps[6]));
    }
    if let Some(caps) = samsung3.captures(filename) {
        let date = &caps[1];
        let time = &caps[2];
        return Some(format!("{}:{}:{} {}:{}:{}", &date[0..4], &date[4..6], &date[6..8], &time[0..2], &time[2..4], &time[4..6]));
    }
    if let Some(caps) = samsung4.captures(filename) {
        return Some(format!("{}:{}:{} {}:{}:{}", &caps[1], &caps[2], &caps[3], &caps[4], &caps[5], &caps[6]));
    }
    if let Some(caps) = samsung5.captures(filename) {
        return Some(format!("{}:{}:{} {}:{}:{}", &caps[1], &caps[2], &caps[3], &caps[4], &caps[5], &caps[6]));
    }
    if let Some(caps) = samsung6.captures(filename) {
        return Some(format!("{}:{}:{} {}:{}:{}", &caps[1], &caps[2], &caps[3], &caps[4], &caps[5], &caps[6]));
    }
    if let Some(caps) = pxl.captures(filename) {
        let date = &caps[1];
        let time = &caps[2][..6];
        return Some(format!("{}:{}:{} {}:{}:{}", &date[0..4], &date[4..6], &date[6..8], &time[0..2], &time[2..4], &time[4..6]));
    }
    if let Some(caps) = ms1.captures(filename) {
        return Some(format!("{}:{}:{} {}:{}:{}", &caps[1], &caps[2], &caps[3], &caps[4], &caps[5], &caps[6]));
    }
    if let Some(caps) = ms2.captures(filename) {
        return Some(format!("{}:{}:{} {}:{}:{}", &caps[1], &caps[2], &caps[3], &caps[4], &caps[5], &caps[6]));
    }
    if let Some(caps) = vid.captures(filename) {
        let date = &caps[1];
        let time = &caps[2];
        return Some(format!("{}:{}:{} {}:{}:{}", &date[0..4], &date[4..6], &date[6..8], &time[0..2], &time[2..4], &time[4..6]));
    }
    if let Some(caps) = sony1.captures(filename) {
        let date = &caps[1];
        let time = &caps[2];
        return Some(format!("{}:{}:{} {}:{}:{}", &date[0..4], &date[4..6], &date[6..8], &time[0..2], &time[2..4], &time[4..6]));
    }
    if let Some(caps) = sony2.captures(filename) {
        let date = &caps[1];
        let time = &caps[2];
        return Some(format!("{}:{}:{} {}:{}:{}", &date[0..4], &date[4..6], &date[6..8], &time[0..2], &time[2..4], &time[4..6]));
    }
    if let Some(caps) = rmlmc.captures(filename) {
        let date = &caps[1];
        let time = &caps[2];
        return Some(format!("{}:{}:{} {}:{}:{}", &date[0..4], &date[4..6], &date[6..8], &time[0..2], &time[2..4], &time[4..6]));
    }
    if let Some(caps) = wallpaper.captures(filename) {
        let date = &caps[1];
        let time = &caps[2];
        return Some(format!("{}:{}:{} {}:{}:{}", &date[0..4], &date[4..6], &date[6..8], &time[0..2], &time[2..4], &time[4..6]));
    }
    if let Some(caps) = san1.captures(filename) {
        // e.g. San-1 Oct 2024.jxl
        let day = caps[1].parse::<u32>().unwrap_or(1);
        let month = match &caps[2] {
            "Jan" => 1, "Feb" => 2, "Mar" => 3, "Apr" => 4, "May" => 5, "Jun" => 6,
            "Jul" => 7, "Aug" => 8, "Sep" => 9, "Oct" => 10, "Nov" => 11, "Dec" => 12, _ => 1
        };
        let year = caps[3].parse::<u32>().unwrap_or(1970);
        return Some(format!("{:04}:{:02}:{:02} 00:00:00", year, month, day));
    }
    None
} 