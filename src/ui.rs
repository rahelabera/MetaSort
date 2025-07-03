// ui.rs
// Polished terminal output for MetaSort

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub struct MetaSortUI {
    main_progress: Option<ProgressBar>,
}

impl MetaSortUI {
    pub fn new() -> Self {
        Self {
            main_progress: None,
        }
    }

    pub fn print_header() {
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
┃                    Cross-platform (macOS & Windows)                   ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
"#);
    }

    pub fn print_section_header(title: &str) {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("  {}", title);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }

    pub fn print_success(message: &str) {
        println!("✅ {}", message);
    }

    pub fn print_warning(message: &str) {
        println!("⚠️  {}", message);
    }

    pub fn print_error(message: &str) {
        println!("❌ {}", message);
    }

    pub fn print_info(message: &str) {
        println!("ℹ️  {}", message);
    }

    pub fn start_main_progress(&mut self, total: u64, message: &str) {
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("█░")
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(120));
        self.main_progress = Some(pb);
    }

    pub fn finish_progress(&mut self, message: &str) {
        if let Some(pb) = &self.main_progress {
            pb.finish_with_message(message.to_string());
        }
        self.main_progress = None;
    }

    pub fn print_summary(
        photos: usize,
        videos: usize,
        whatsapp: usize,
        screenshots: usize,
        unknown: usize,
        mkv: usize,
        errors: usize,
        output_path: &str,
    ) {
        let total = photos + videos + whatsapp + screenshots + unknown + mkv;
        
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("                           MetaSort Summary");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("  📸 Photos processed:     {}", photos);
        println!("  🎥 Videos processed:     {}", videos);
        println!("  💬 WhatsApp images:      {}", whatsapp);
        println!("  📱 Screenshots:          {}", screenshots);
        println!("  ❓ Unknown time:         {}", unknown);
        println!("  🎬 MKV files:            {}", mkv);
        println!("  📊 Total files:          {}", total);
        println!("  ⚠️  Errors encountered:   {}", errors);
        println!("  📁 Output location:      {}", output_path);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }

    pub fn print_footer() {
        println!("\n");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("  💖 Like my work? Please consider donating! 💖");
        println!("  🌟 Support MetaSort: https://upier.vercel.app/pay/sanmith@superyes");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
} 