use colored::Colorize;

pub fn print_banner() {
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║                                                              ║".bright_cyan());
    println!("{}", "║    🦉  OWLSOL - Solana Account Storage Compression  🦉      ║".bright_cyan().bold());
    println!("{}", "║                                                              ║".bright_cyan());
    println!("{}", "║         Compress • Save Costs • Scale Efficiently           ║".bright_white());
    println!("{}", "║                                                              ║".bright_cyan());
    println!("{}", "╚══════════════════════════════════════════════════════════════╝".bright_cyan());
    println!();
}

pub fn print_section_header(title: &str) {
    println!();
    println!("{} {}", "━".repeat(3).bright_black(), title.bright_yellow().bold());
    println!();
}

pub fn print_success(message: &str) {
    println!("  {} {}", "✓".bright_green().bold(), message);
}

pub fn print_info(message: &str) {
    println!("  {} {}", "ℹ".bright_blue(), message.bright_white());
}

pub fn print_warning(message: &str) {
    println!("  {} {}", "⚠".bright_yellow(), message.bright_yellow());
}

pub fn print_error(message: &str) {
    println!("  {} {}", "✗".bright_red().bold(), message.bright_red());
}

pub fn print_metric(label: &str, value: &str, unit: &str) {
    println!(
        "  {} {} {}",
        format!("{}:", label).bright_black(),
        value.bright_white().bold(),
        unit.bright_black()
    );
}

pub fn print_separator() {
    println!("{}", "  ─".repeat(30).bright_black());
}
