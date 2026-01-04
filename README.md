# GM48Scraper

A command line interface for scraping GameMaker data files from the
[GM48 Game Jam](https://gm48.net).

## Installation Guide

1. Go to "Releases" on the right side
2. Download the latest release for your platform

If there is no binary for your platform / architecture, you need to build it
from source (see `Compilation Guide`).

## Compilation Guide

1. Install the [Rust toolchain](https://rustup.rs)
2. Open a terminal and navigate to some temporary directory
3. Clone this Git repository
   (`git clone https://github.com/BioTomateDE/GM48Scraper`)
4. Build the binary (`cargo build -r`)
5. Execute the binary (`./target/release/gm48-scraper.exe`)

## Usage

Open a terminal and execute the binary, for example: `~/Downloads/gm48-scraper`.

By default, the program creates a directory called `gm48_datafiles` where it
will dump all GameMaker data files. However, you can pass a command line
argument to the CLI in order to customize your output directory. Example:
`./gm48-scraper ~/Documents/gamemaker_datafiles/`

## What's the purpose of this

The reason I made this program is to get lots of GameMaker data files
(`data.win` files) in order to test my GameMaker asset unpacker tool
[LibGM](https://github.com/BioTomateDE/LibGM).

## Does this violate ToS

I read (skimmed) GM48's Terms of Service and didn't find any sentence explicitly
disallowing data scrapers. However, I don't study law.

This program uses the standard [reqwest](https://crates.io/crates/reqwest) user
agent. If the website wanted to block scrapers like this, they could easily
detect a non-browser user agent.

Also, their [`robots.txt`](https://dotnet.48/robots.txt) file allows scraping
for all user-agents anyway.

That being said: **Use it on your own risk.** I take no responsibility for
lawsuits, IP-bans or any other punishment as a result of using this program.

## Contributing

All contributions are welcome! Just open up a GitHub Issue or Pull request.
