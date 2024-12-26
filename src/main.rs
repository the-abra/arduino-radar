use std::env;
use std::io;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tui::{
    backend::{CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Table, List, ListItem},
    Terminal,
};
use crossterm::ExecutableCommand;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, ClearType, Clear};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    clear_screen();
    // Parse the serial port address from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <serial-port-address>", args[0]);
        std::process::exit(1);
    }
    let port_name = &args[1];

    // Open the serial port
    let port = serialport::new(port_name, 9600)
    .timeout(Duration::from_millis(200))
    .open();

    let port = match port {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to open port {}: {}", port_name, e);
            std::process::exit(1);
        }
    };

    // Create a thread-safe channel to send Serial to the UI
    let (tx, rx) = mpsc::channel();

    // Start a thread to read serial data
    thread::spawn(move || {
        let reader = BufReader::new(port);
        for line in reader.lines() {
            if let Ok(data) = line {
                tx.send(data).ok(); // Send data to the channel
            }
        }
    });

    // Set up the TUI
    enable_raw_mode()?;
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Serial Table
    let mut radar_data: Vec<(String, String)> = vec![];

    // Main TUI loop
    loop {
        // Check for keyboard input to exit
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // Receive and process Serial
        if let Ok(data) = rx.try_recv() {
            if let Some((angle, distance)) = parse_radar_data(&data) {
                radar_data.push((angle, distance));

                // Keep only the last 16 readings
                if radar_data.len() > 16 {
                    radar_data.remove(0);
                }
            }
        }

        // Render the TUI
        terminal.draw(|f| {
            let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),  // Heading
                         Constraint::Length(20), // Serial Table height
                         Constraint::Length(7),  // Footer height
            ].as_ref())
            .split(f.size());

            // Heading
            let heading = Paragraph::new("Serial Monitor (Press 'q' to quit)")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("Status"));
            f.render_widget(heading, chunks[0]);

            // Serial Table
            let table = Table::new(
                radar_data.iter().map(|(angle, distance)| {
                    Row::new(vec![angle.clone(), distance.clone()])
                }),
            )
            .header(Row::new(vec!["Angle (°)", "Distance (cm)"]).style(Style::default().fg(Color::Yellow)))
            .block(Block::default().borders(Borders::ALL).title("Serial"))
            .widths(&[Constraint::Length(10), Constraint::Length(15)]);

            f.render_widget(table, chunks[1]);

            // Footer (About Section)
            let footer_text = vec![
                ListItem::new("About: Arduino Radar Project").style(Style::default().fg(Color::Blue)),
                      ListItem::new("GitHub: https://github.com/the-abra/arduino-radar").style(Style::default().fg(Color::Green)),
                      ListItem::new("Requirements:").style(Style::default().fg(Color::Magenta)),
                      ListItem::new("- Arduino Uno (or equivalent)").style(Style::default().fg(Color::White)),
                      ListItem::new("- SG90 Servo Motor").style(Style::default().fg(Color::White)),
                      ListItem::new("- HC-SR04 Ultrasonic Distance Sensor").style(Style::default().fg(Color::White)),
                      ListItem::new("Wiring:").style(Style::default().fg(Color::Magenta)),
                      ListItem::new("HC-SR04: TRIG -> pin 3, ECHO -> pin 2, VCC -> 5V, GND -> GND").style(Style::default().fg(Color::White)),
                      ListItem::new("SG90: Signal -> pin 9, VCC -> 5V, GND -> GND").style(Style::default().fg(Color::White)),
                      ListItem::new("To run:").style(Style::default().fg(Color::Magenta)),
                      ListItem::new("1. Build: cargo build --release").style(Style::default().fg(Color::White)),
                      ListItem::new("2. Run: cargo run -- /dev/ttyUSB0").style(Style::default().fg(Color::White)),
            ];

            let footer = List::new(footer_text)
            .block(Block::default().borders(Borders::ALL).title("About"));

            f.render_widget(footer, chunks[2]);
        })?;
    }

    // Cleanup
    disable_raw_mode()?;
    terminal.clear()?;
    Ok(())
}

// Parse Serial from the serial input
fn parse_radar_data(data: &str) -> Option<(String, String)> {
    // Example data: "Angle: 45°, Distance: 28 cm"
    let parts: Vec<&str> = data.split(',').collect();
    if parts.len() == 2 {
        let angle = parts[0].trim().replace("Angle:", "").trim().to_string();
        let distance = parts[1].trim().replace("Distance:", "").trim().to_string();
        return Some((angle, distance));
    }
    None
}

fn clear_screen() {
    io::stdout().execute(Clear(ClearType::All)).unwrap();
}
