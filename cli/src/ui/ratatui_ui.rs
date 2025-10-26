use ratatui::{prelude::*, widgets::*};
use std::io;

pub fn show_compression_stats(stats: &[(String, String)]) -> io::Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;
    let rows: Vec<Row> = stats
        .iter()
        .map(|(k, v)| Row::new(vec![k.clone(), v.clone()]))
        .collect();
    let widths = [Constraint::Length(20), Constraint::Length(30)];
    let table = Table::new(rows, widths)
        .header(
            Row::new(vec!["Field", "Value"]).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .block(
            Block::default()
                .title("Compression Results")
                .borders(Borders::ALL),
        );
    terminal.draw(|f| {
        let size = f.size();
        f.render_widget(table, size);
    })?;
    Ok(())
}
