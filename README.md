```console
▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
█▀▄▄▀█ ▄▄▀█ ██ █ █ ██
█ ██ █ ▀▀▄█ ▀▀ █▀▄▀██
██▄▄██▄█▄▄█▀▀▀▄█▄█▄██
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀

oryx 0.1.0
A time tracker.

USAGE:
    oryx [OPTIONS] [SUBCOMMAND]
	
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
			
OPTIONS:
    -l, --labels <Session Labels>    Comma seperated labels (categories)
    -t, --title <Session Title>      The title of the session
					
SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    log       Show session history
    status    Show session status						
```

### Features

- Timer
- Session Labels (Categories)
- History (saved to ./.oryx)
- Log
- Desktop Notifications

### Installing

Requirements
- libdbus (linux), used for desktop notifications

You can install it with the Rust package manager 
[Cargo](https://github.com/rust-lang/cargo) like this:

``` bash
 $ cargo install oryx
```

### Timer

Every session has a title, you can start a session like this:
	
	$ oryx --title "Create website"

A countdown timer will start, it will notify you when it's done:

```bash
Focusing on: Client Side
##-------------------------------------- 00:00:40
```			

You can also label a session by providing it with comma seperated labels:

```bash			
$ oryx --title "Client side" --labels "ui, projects"
```

### Status
	
To show current status use:

```
$ oryx status
TODAY: 0 sessions 0h:0m <23 sessions 9h:35m ALL>
```


### Log
		
To show history of sessions: 

```
$ oryx log

TODAY: 0 sessions 0h:0m <3 sessions 1h:25m ALL>

Title: Client Side
Labels: ui, projects
Date: 2020-09-24 17:24
			
Title: Server Side
Labels: api, projects
Date: 2020-09-24 14:25
			
Title: Docs
Labels: documentation
Date: 2020-09-24 13:38
```
You can also log sessions based on their labels:

```
$ oryx log --labels registers
```

### Credits

Ascii art by https://textfancy.com
