use colored::*;

pub fn print_success(message: &str) {
    println!("{} {}", "✅".bright_green(), message.bright_white());
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", "❌".bright_red(), message.bright_red());
}

pub fn print_warning(message: &str) {
    println!("{} {}", "⚠️".bright_yellow(), message.bright_yellow());
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ️".bright_blue(), message.bright_cyan());
}

pub fn print_step(step: &str, message: &str) {
    println!("{} {}", step.bright_blue(), message.bright_white());
}

pub fn print_header(title: &str) {
    println!();
    println!("{}", title.bright_cyan().bold());
    println!("{}", "=".repeat(title.len()).bright_black());
    println!();
}

pub fn print_subheader(title: &str) {
    println!("{}", title.bright_blue().bold());
    println!("{}", "-".repeat(title.len()).bright_black());
}

pub struct ProgressBar {
    current: usize,
    total: usize,
    width: usize,
}

impl ProgressBar {
    pub fn new(total: usize) -> Self {
        Self {
            current: 0,
            total,
            width: 40,
        }
    }

    pub fn update(&mut self, current: usize) {
        self.current = current;
        self.render();
    }

    pub fn increment(&mut self) {
        self.current += 1;
        self.render();
    }

    pub fn finish(&self) {
        println!();
    }

    fn render(&self) {
        let percentage = if self.total > 0 {
            (self.current as f32 / self.total as f32) * 100.0
        } else {
            0.0
        };

        let filled = ((self.current as f32 / self.total as f32) * self.width as f32) as usize;
        let empty = self.width - filled;

        let progress_bar = format!(
            "[{}{}] {:.1}% ({}/{})",
            "█".repeat(filled).bright_green(),
            "░".repeat(empty).bright_black(),
            percentage,
            self.current,
            self.total
        );

        print!("\r{}", progress_bar);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1023), "1023 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
        assert_eq!(format_bytes(1073741824), "1.0 GB");
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("hello", 10), "hello");
        assert_eq!(truncate_string("hello world", 8), "hello...");
        assert_eq!(truncate_string("hi", 5), "hi");
        assert_eq!(truncate_string("", 5), "");
    }

    #[test]
    fn test_progress_bar() {
        let mut pb = ProgressBar::new(100);
        pb.update(25);
        pb.update(50);
        pb.update(100);
        pb.finish();
    }
}
