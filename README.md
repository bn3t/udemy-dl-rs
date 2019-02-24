
# Download udemy courses in batch. [![Build Status](https://travis-ci.org/bn3t/udemy-dl-rs.svg?branch=develop)](https://travis-ci.org/bn3t/udemy-dl-rs)

# Usage

A cross-platform utility written in Rust to download courses from udemy for personal offline use.

## Features

- Save course information to a text file (JSON) (option: `info -s, --save <save>`).
- List down course contents and video resolution (option: `info`).
- Download specific chapter in a course (option: `-c / --chapter`).
- Download specific lecture in a chapter (option: `-l / --lecture`).
- Automatically pickup the best resolution for video download.
- Download lecture(s) requested resolution (option: `-q / --quality`).
- Download course to user requested path (option: `-o / --output`).
- Mark complte chapters or individual lectures as complete. 
- Authentication token (option: `-t / --access-token`).

## Authentication Details

You can either connect and authenticate with your username / password or use an *Access Token*. The following paragraph details how to obtain such a token.

### Extracting your Access Token

 - Open developer tools on your browser and access the **Network Tab**.
 - Login to your udemy account.
 - Check the network tab, you can filter on XHR requests to make the following easier.
 - Right click on request links to **udemy.com/api-2.0/**. Check the request cookies and find one named *access_token*. Copy its value. This is your access token.

## Example Usage

### Obtain information from a course

    udemy-dl-rs -u COURSE_URL -t YourAccessToken info

### Download a course to current diretory

    udemy-dl-rs -u COURSE_URL -t YourAccessToken download

### Download a course to a specific directory

    udemy-dl-rs -u COURSE_URL -t YourAccessToken download -o ~/Downloads

### Download a course to a specific directory with a specific quality

    udemy-dl-rs -u COURSE_URL -t YourAccessToken download -o ~/Downloads -q 720

### Download a specific chapter

    udemy-dl-rs -u COURSE_URL -t YourAccessToken download -o ~/Downloads -c 1

### Download a specific lecture from a chapter

    udemy-dl-rs -u COURSE_URL -t YourAccessToken download -o ~/Downloads -c 8 -l 77

Note: The lecture number is it's index in the overall course. Use info to know more.

## Command Line Usage

### General Usage

```
$ udemy-dl-rs
Udemy Downloader 0.9.2
Bernard Niset


USAGE:
    udemy-dl-rs [FLAGS] [OPTIONS] --password <PASSWORD> --url <URL> --username <USERNAME> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Sets the level of verbosity

OPTIONS:
    -t, --access-token <TOKEN>    Access token to authenticate to udemy
    -p, --password <PASSWORD>     Password to authenticate to udemy
    -u, --url <URL>               URL of the course to download
    -U, --username <USERNAME>     Username to authenticate to udemy

SUBCOMMANDS:
    complete    Mark courses as completed
    download    Download course content
    help        Prints this message or the help of the given subcommand(s)
    info        Query course information

Build: unknown - 2019-02-21
```

### Subcommand Usage - info

```
$ udemy-dl-rs info --help
udemy-dl-rs-info 
Query course information

USAGE:
    udemy-dl-rs --password <PASSWORD> --url <URL> --username <USERNAME> info

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```


### Subcommand Usage - download

```
$ udemy-dl-rs download --help
udemy-dl-rs-download 
Download course content

USAGE:
    udemy-dl-rs --password <PASSWORD> --url <URL> --username <USERNAME> download [FLAGS] [OPTIONS]

FLAGS:
    -d, --dry-run    Dry run, show what's would be done but don't download anything.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --chapter <CHAPTER>      Restrict downloads to a specific chapter.
    -l, --lecture <LECTURE>      Restrict download to a specific lecture.
    -o, --output <OUTPUT_DIR>    Directory where to output downloaded files (default to .). [default: .]
    -q, --quality <QUALITY>      Download specific video quality.
```

### Subcommand Usage - complete

```
$ udemy-dl-rs complte --help
udemy-dl-rs-complete 
Mark courses as completed

USAGE:
    udemy-dl-rs --password <PASSWORD> --url <URL> --username <USERNAME> complete [OPTIONS] --chapter <CHAPTER>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --chapter <CHAPTER>    Restrict marking a specific chapter.
    -l, --lecture <LECTURE>    Restrict marking a specific lecture.
```

## To do

- Resume capability for a course video.
- Supports organization and individual udemy users both.
- Download subtitles for a video.
- Download chapter(s) by providing range in a course.
- Download lecture(s) by providing range in a chapter.

# Development Guidelines

## Unit tests

Install cargo watch

    cargo install cargo-watch

Run unit tests. Unit tests need to run single threaded.

    cargo test -- --test-threads=1

Run unit tests with watch

    cargo watch -w src -x "test -- --test-threads=1"

## Run info command

    cargo run -- -u https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass -t YourAccessToken -c YourClientId info

## Run download command

    cargo run -- -u https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass -t YourAccessToken -c YourClientId download -c 1

## Alternative login / password access

    cargo run -- -u https://www.udemy.com/css-the-complete-guide-incl-flexbox-grid-sass -U Email -p YourPassword  download -c 1 -o ~/Downloads

