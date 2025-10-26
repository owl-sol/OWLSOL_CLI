use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, FeeSpeed, InputField};

pub fn draw(f: &mut Frame, app: &App) {
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(3),  // Wallet info
            Constraint::Length(8),  // Swap config
            Constraint::Length(10), // Optimization analysis
            Constraint::Length(3),  // Buttons
            Constraint::Min(2),     // Status
        ])
        .split(f.area());

    // Render each section
    draw_header(f, chunks[0]);
    draw_wallet_info(f, chunks[1], app);
    draw_swap_config(f, chunks[2], app);
    draw_optimization(f, chunks[3], app);
    draw_buttons(f, chunks[4], app);
    draw_status(f, chunks[5], app);
}

fn draw_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new("🦉 OWLSOL - Smart Solana Swap Optimizer")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );

    f.render_widget(header, area);
}

fn draw_wallet_info(f: &mut Frame, area: Rect, app: &App) {
    let short_pk = if app.wallet_pubkey.len() > 12 {
        format!(
            "{}...{}",
            &app.wallet_pubkey[..8],
            &app.wallet_pubkey[app.wallet_pubkey.len() - 4..]
        )
    } else {
        app.wallet_pubkey.clone()
    };

    let text = format!("Wallet: {}  |  Balance: {:.4} SOL", short_pk, app.balance);

    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Wallet Info")
                .border_style(Style::default().fg(Color::White)),
        );

    f.render_widget(paragraph, area);
}

fn draw_swap_config(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Swap Configuration")
        .border_style(Style::default().fg(Color::White));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Styles for active/inactive fields
    let from_style = if app.active_field == InputField::FromToken {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let to_style = if app.active_field == InputField::ToToken {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let amount_style = if app.active_field == InputField::Amount {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let speed_style = if app.active_field == InputField::Speed {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    // Build lines
    let lines = vec![
        Line::from(vec![
            Span::styled("From:  ", Style::default().fg(Color::Gray)),
            Span::styled(format!("[{}]", app.from_token), from_style),
            Span::raw("  |  "),
            Span::styled("Amount: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("[{}]", app.amount), amount_style),
        ]),
        Line::from(vec![
            Span::styled("To:    ", Style::default().fg(Color::Gray)),
            Span::styled(format!("[{}]", app.to_token), to_style),
            Span::raw("  |  "),
            Span::styled("You get: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("~{:.4} {}", app.estimated_output, app.to_token),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Speed: ", Style::default().fg(Color::Gray)),
            Span::styled(
                if app.speed == FeeSpeed::Economy { "◉" } else { "○" },
                speed_style,
            ),
            Span::styled(" Economy  ", speed_style),
            Span::styled(
                if app.speed == FeeSpeed::Standard { "◉" } else { "○" },
                speed_style,
            ),
            Span::styled(" Standard  ", speed_style),
            Span::styled(
                if app.speed == FeeSpeed::Fast { "◉" } else { "○" },
                speed_style,
            ),
            Span::styled(" Fast", speed_style),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Tip: ", Style::default().fg(Color::Cyan)),
            Span::raw("Use Tab to navigate, Up/Down to change options"),
        ]),
    ];

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}

fn draw_optimization(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Optimization Analysis")
        .border_style(Style::default().fg(Color::White));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let lines = vec![
        Line::from(vec![
            Span::styled("✅ ", Style::default().fg(Color::Green)),
            Span::raw(format!(
                "Optimal priority fee: {} micro-lamports/CU",
                app.optimal_fee
            )),
        ]),
        Line::from(vec![
            Span::styled("✅ ", Style::default().fg(Color::Green)),
            Span::raw(format!(
                "Compute units optimized: {} CU (vs 200k)",
                app.estimated_cu
            )),
        ]),
        Line::from(vec![
            Span::styled("✅ ", Style::default().fg(Color::Green)),
            Span::raw("Using Jupiter ALT (46% smaller TX)"),
        ]),
        Line::from(vec![
            Span::styled("✅ ", Style::default().fg(Color::Green)),
            Span::raw(format!("Route comparison: {} routes analyzed", app.routes_analyzed)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("💰 ", Style::default()),
            Span::styled(
                format!(
                    "Estimated fee: {:.6} SOL (${:.2})",
                    app.owlsol_fee,
                    app.owlsol_fee * 190.0
                ),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("📉 ", Style::default()),
            Span::raw(format!("vs Normal wallet: {:.6} SOL ", app.normal_fee)),
            Span::styled(
                format!("({:.1}% savings!)", app.savings_pct),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}

fn draw_buttons(f: &mut Frame, area: Rect, app: &App) {
    let button_text = if app.is_loading {
        "⏳ Loading... Please wait"
    } else if app.can_execute() {
        "[Enter] Execute Swap  |  [Q] Quit"
    } else {
        "[Tab] Navigate  |  [Up/Down] Change  |  [Q] Quit"
    };

    let button_color = if app.can_execute() && !app.is_loading {
        Color::Green
    } else {
        Color::Cyan
    };

    let paragraph = Paragraph::new(button_text)
        .style(Style::default().fg(button_color).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(button_color)),
        );

    f.render_widget(paragraph, area);
}

fn draw_status(f: &mut Frame, area: Rect, app: &App) {
    let (status_text, status_color) = if let Some(ref error) = app.error {
        (format!("❌ Error: {}", error), Color::Red)
    } else if app.is_loading {
        (format!("⏳ {}", app.status), Color::Yellow)
    } else if app.is_ready {
        (format!("✅ {}", app.status), Color::Green)
    } else {
        (app.status.clone(), Color::White)
    };

    let paragraph = Paragraph::new(status_text)
        .style(Style::default().fg(status_color))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Status")
                .border_style(Style::default().fg(status_color)),
        );

    f.render_widget(paragraph, area);
}
