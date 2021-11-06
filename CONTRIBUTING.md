# Contributing to Homecloud Photos

## Backend

Homecloud Photos is written in rust so the rust toolchain is needed for
development. Follow the [instructions on the official rust website](https://www.rust-lang.org/tools/install)
to install rust and cargo for your environment.

Additionally, sqlite is used as the database for the photo library so both
sqlite3 and its development headers will need to be installed to compile:

- **macOS:** `brew install sqlite`
- **Debian**, **Ubuntu:** `apt-get install libsqlite3-dev`
- **Arch:** `pacman -S sqlite`

## Fontend

The frontend for now will be very simple server side rendered templates with
minimal front end javascript. The only reason for this is my lack of experience
with front end frameworks. If someone more familiar with front end systems wants
to make a single page app version of the UI in react or whatever, I would love
to work with you!
