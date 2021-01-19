use clap::{App, Arg, SubCommand};

fn main() -> std::io::Result<()> {
    let matches = App::new("oryx")
        .version("0.1.0")
        .about("A time tracker.")
	.before_help("▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄\n█▀▄▄▀█ ▄▄▀█ ██ █ █ ██\n█ ██ █ ▀▀▄█ ▀▀ █▀▄▀██\n██▄▄██▄█▄▄█▀▀▀▄█▄█▄██\n▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀")
        .arg(
            Arg::with_name("title")
                .short("t")
                .long("title")
                .value_name("Session Title")
                .help("The title of the session")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("labels")
                .short("l")
                .long("labels")
                .value_name("Session Labels")
                .help("Comma seperated labels (categories)")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("status")
                .about("Show session status")
                .arg(
                    Arg::with_name("labels")
                        .short("l")
                        .value_name("Session Labels")
                        .help("Filter sessions using comma seperated labels"),
                ),
        )
        .subcommand(
            SubCommand::with_name("log")
                .about("Show session history")
                .arg(
                    Arg::with_name("labels")
                        .short("l")
                        .value_name("Session Labels")
                        .help("Filter session using comma seperated labels"),
                ),
        )
        .get_matches();

    // Session Status
    if let Some(matches) = matches.subcommand_matches("status") {
        let mut labels = "";

        if let Some(l) = matches.value_of("labels") {
            labels = l;
        }

        oryx::status(labels)?;
    }

    // Log history
    if let Some(matches) = matches.subcommand_matches("log") {
        let mut labels = "";

        if let Some(l) = matches.value_of("labels") {
            labels = l;
        }

        oryx::log(labels)?;
    }

    // Start timer
    if let Some(title) = matches.value_of("title") {
        let mut labels = "";

        if let Some(l) = matches.value_of("labels") {
            labels = l;
        }

        // start session timer
        oryx::timer(&title, labels)?;
    }

    Ok(())
}
