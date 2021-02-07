use ansi_term::{Colour, Style};
use indicatif::{ProgressBar, ProgressStyle};
use notify_rust::Notification;
use pager::Pager;
use serde::{Deserialize, Serialize};
use simpler_timer::Timer;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{BufReader, Read};
use std::io::{BufWriter, Write};
use std::time::Duration;
use time::OffsetDateTime;

/// Represents a Session
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Session {
    /// Session title
    title: String,
    /// Session labels (categories)
    labels: Vec<String>,
    /// Date of session
    date: String,
    /// Time of session
    time: String,
}

/// Session Timer
pub fn timer(title: &str, labels: &str) -> std::io::Result<()> {
    let bar = ProgressBar::new(1500);

    bar.set_style(
        ProgressStyle::default_bar()
            .template(" {bar:40.cyan/blue} {elapsed_precise}")
            .progress_chars("##-"),
    );

    // Periodic ticker every  second
    let periodic = Timer::with_duration(Duration::from_secs(1));

    // Timeout after 1500sec (25min)
    let timeout = Timer::with_duration(Duration::from_secs(1500));

    println!(" Focusing on: {}", Colour::Cyan.paint(title));

    loop {
        if periodic.expired() {
            bar.inc(1);
            periodic.reset();
        }

        if timeout.expired() {
            bar.finish();
            println!("Session ended, take a break.");
            break;
        }
    }

    // generate latest session
    let session = generate_session(title, labels)?;

    // Initialize history ( the `.oryx` file)
    init_history()?;

    // read history
    let mut history = read_history()?;

    // add session to front of history
    history.insert(0, session);

    // Write history to json file
    write_history(&history)?;

    // Desktop notification
    notify(title)?;

    Ok(())
}

/// Create desktop notification
fn notify(title: &str) -> std::io::Result<()> {
    let body = format!("Session Ended: `{}`, take a break.", title);

    Notification::new()
        .summary("Session")
        .body(&body)
        .appname("session")
        .timeout(0)
        .show()
        .unwrap();

    Ok(())
}

/// Generate a Session
fn generate_session(title: &str, labels: &str) -> std::io::Result<Session> {
    let date = format!("{}", OffsetDateTime::now_local().date());
    let time = OffsetDateTime::now_local().time().format("%R");

    let lbls: Vec<String> = labels.split(',').map(|s| s.trim().to_owned()).collect();

    Ok(Session {
        title: String::from(title),
        labels: lbls,
        date,
        time,
    })
}

/// Initialze Session history file (./.oryx)
fn init_history() -> std::io::Result<()> {
    File::open("./.oryx").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            let file = File::create("./.oryx").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            });

            let initial_json = "[]";
            let f = File::create("./.oryx").expect("Unable to create file");
            let mut f = BufWriter::new(f);
            f.write_all(initial_json.as_bytes())
                .expect("Unable to write data");

            file
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    Ok(())
}

/// Read Session history from JSON file (./.oryx)
fn read_history() -> std::io::Result<Vec<Session>> {
    let mut data = String::new();
    let f = File::open("./.oryx").expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");

    let v: Vec<Session> = serde_json::from_str(&data)?;

    Ok(v)
}

/// Write Session history To JSON file (./oryx)
fn write_history(history: &[Session]) -> std::io::Result<()> {
    let output = serde_json::to_string(&history).unwrap();
    let f = File::create("./.oryx").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(output.as_bytes())
        .expect("Unable to write data");

    Ok(())
}

// Filter session's by label
fn filter_sessions(labels: &str) -> Vec<Session> {
    let history = read_history().unwrap();
    let mut filtered: Vec<Session> = vec![];

    for label in labels.split(',') {
        for session in history.iter() {
            // Check if session contains label
            // TODO: Avoid cloning by using `Move Semantics`:
            // session.labels.into_iter()
            if session.labels.iter().any(|p| p == label.trim()) {
                filtered.push(session.clone());
            }
        }
    }

    if labels == "" {
        history
    } else {
        filtered
    }
}

/// Show status
pub fn status(labels: &str) -> std::io::Result<()> {
    let to_show = filter_sessions(labels);

    let total_minutes = to_show.len() * 25;
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;

    let date = format!("{}", OffsetDateTime::now_local().date());
    let today = to_show.iter().filter(|p| p.date == date).count();
    let today_total_minutes = today * 25;
    let today_hours = today_total_minutes / 60;
    let today_minutes = today_total_minutes % 60;
    let today_str = format!(
        "TODAY: {} sessions {}h:{}m ",
        today, today_hours, today_minutes
    );
    let today = to_show.iter().filter(|p| p.date == date).count();

    match today {
        0..=2 => {
            print!("{}", Colour::Red.paint(today_str));
        }
        3..=7 => {
            print!("{}", Colour::Yellow.paint(today_str));
        }
        _ => {
            print!("{}", Colour::Green.paint(today_str));
        }
    }

    println!(
        "{}",
        format!(
            "<{} sessions {}h:{}m (all time)>",
            to_show.len(),
            hours,
            minutes
        )
    );

    Ok(())
}

/// Log session history
pub fn log(labels: &str) -> std::io::Result<()> {
    let to_show = filter_sessions(labels);
    let label_style = Style::new().bold().on(Colour::Cyan).fg(Colour::Black);

    Pager::with_pager("less -r").setup();

    status(labels)?;

    for session in to_show {
        println!(
            "\n{}",
            Colour::Cyan.paint(format!("Title: {}", session.title))
        );
        println!("Labels: {}", label_style.paint(session.labels.join(",")));
        println!("Date: {} {}", session.date, session.time);
    }

    Ok(())
}
