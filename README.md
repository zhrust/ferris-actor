# Ferris Actor 
> ASCII-art Animation and snap as ASCII-art logo for Ferris
> 蟹老板终端动画片和随机 pose

![ferris-actor](https://ipic.zoomquiet.top/2023-03-02-230302-ferris-actor.gif)

.. effect animation snap under in [Swordfish90/cool-retro-term: A good looking terminal emulator which mimics the old cathode display...](https://github.com/Swordfish90/cool-retro-term)

------
## Command-line options

```
a CLI tool for show and snap ASCII-art animation for Ferris

Usage: ferris-actor <COMMAND>

Commands:
  go    Ferris action!
  snap  snap from animation
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```



------
## Installation

### Cargo
If you already have a Rust environment set up, you can use the cargo install command:

> $ cargo install ferris-actor

Cargo will build the `ferris-actor` binary and place it in $HOME/.cargo.


### Manual installation from GitHub
Compiled binary versions of `ferris-actor` are uploaded to GitHub when a release is made. You can install `ferris-actor` manually by downloading a release, extracting it, and copying the binary to a directory in your `$PATH`, such as `/usr/local/bin`.

For more information, ...TBD

### Homebrew

..TBD

## background
[Animated Ferris - JSFiddle](https://jsfiddle.net/Diggsey/3pdgh52r/embedded/result/)

## goal

as crate, can:

- easy install
- usage at local
- look animation in terminal
    - or snap randomly ASCII-art for ferris usage Markdown

as one Rust learnning result...

## logging

- 230302 ZQ v0.1.42 can work
- 230301 ZQ init.

### refer.


- [ferris-says - crates.io: Rust Package Registry](https://crates.io/crates/ferris-says)
- [ferris-say - crates.io: Rust Package Registry](https://crates.io/crates/ferris-say)
    - [spaghettidev 🦀 — I'm a Software engineering student and Typescript & Rust developer.](https://spaghettidev.tech/posts/creating-a-cli-with-rust/)
- [Rustacean.net: Home of Ferris the Crab](https://rustacean.net/)
    - [Animated Ferris - JSFiddle](https://jsfiddle.net/Diggsey/3pdgh52r/embedded/result/)
- [Tokens - The Rust Reference](https://doc.rust-lang.org/reference/tokens.html#raw-string-literals)
    - ...

Snap a randomly Ferris ASCII-art pose as Markdown:

```
         _~`+~~_
     () /  ^ o  \ \/
       '_   V   _'
       | '--.--' <

...act by ferris-actor v0.1.42
```