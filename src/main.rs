mod password;
mod guid;
mod secret;
mod clipboard;

use std::env;
use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Select, Input, MultiSelect};
use password::{PasswordConfig, generate_password};
use guid::{GuidVersion, generate_guid};
use secret::{SecretEncoding, generate_secret};
use clipboard::{copy_text, ClipboardResult};
use zxcvbn::zxcvbn;

#[derive(Parser)]
#[command(name = "fortify-cli")]
#[command(about = "A CLI tool for generating passwords, GUIDs, and secrets", long_about = None)]
struct Cli {
    #[arg(global = true, long, action = clap::ArgAction::SetTrue)]
    no_copy: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Password {
        #[arg(short, long, default_value_t = 16)]
        length: usize,
        
        #[arg(short, long)]
        uppercase: bool,
        
        #[arg(short = 'w', long)]
        lowercase: bool,
        
        #[arg(short, long)]
        numbers: bool,
        
        #[arg(short, long)]
        symbols: bool,
    },
    Guid {
        #[arg(long, conflicts_with = "v7")]
        v4: bool,

        #[arg(long, conflicts_with = "v4")]
        v7: bool,
    },
    Secret {
        #[arg(short, long, default_value_t = 32)]
        length: usize,
        
        #[arg(long, conflicts_with = "base64")]
        hex: bool,

        #[arg(long, conflicts_with = "hex")]
        base64: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Password { length, uppercase, lowercase, numbers, symbols }) => {
             let (u, l, n, s) = if !*uppercase && !*lowercase && !*numbers && !*symbols {
                 (true, true, true, true)
             } else {
                 (*uppercase, *lowercase, *numbers, *symbols)
             };

             let config = PasswordConfig {
                 length: *length,
                 uppercase: u,
                 lowercase: l,
                 numbers: n,
                 symbols: s,
             };
             let result = generate_password(&config);
             finalize_output(result, true, cli.no_copy);
        },
        Some(Commands::Guid { v4: _, v7 }) => {
            let v = if *v7 {
                GuidVersion::V7
            } else {
                GuidVersion::V4
            };
            let result = generate_guid(v);
            finalize_output(result, false, cli.no_copy);
        },
        Some(Commands::Secret { length, hex: _, base64 }) => {
             let enc = if *base64 {
                 SecretEncoding::Base64
             } else {
                 SecretEncoding::Hex
             };
             let result = generate_secret(*length, enc);
             finalize_output(result, false, cli.no_copy);
        },
        None => {
            interactive_mode();
        }
    }
}

fn finalize_output(text: String, is_password: bool, no_copy: bool) {
    println!("{}", text);
    if is_password {
        let estimate = zxcvbn(&text, &[]);
        println!("Strength: {}/4", estimate.score());
        if let Some(feedback) = estimate.feedback() {
             if let Some(warning) = feedback.warning() {
                 println!("Warning: {}", warning);
             }
        }
    }

    if !no_copy {
        match copy_text(&text) {
            ClipboardResult::Success => println!("(Copied to clipboard)"),
            ClipboardResult::Error(e) => {
                 if env::var("RUST_LOG").is_ok() {
                     eprintln!("Clipboard error: {}", e);
                 }
            },
            ClipboardResult::Unavailable => {
                // Headless or unsupported, silent ignore
            }
        }
    }
}

fn interactive_mode() {
    let selections = &["Password", "GUID", "Secret", "Exit"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to generate?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selection {
        0 => interactive_password(),
        1 => interactive_guid(),
        2 => interactive_secret(),
        _ => std::process::exit(0),
    }
}

fn interactive_password() {
    let length: usize = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Password length")
        .default(16)
        .interact_text()
        .unwrap();

    let options = &["Uppercase", "Lowercase", "Numbers", "Symbols"];
    let defaults = &[true, true, true, true];
    
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select character sets (Space to toggle, Enter to confirm)")
        .items(options)
        .defaults(defaults)
        .interact()
        .unwrap();

    let config = PasswordConfig {
        length,
        uppercase: selections.contains(&0),
        lowercase: selections.contains(&1),
        numbers: selections.contains(&2),
        symbols: selections.contains(&3),
    };

    let result = generate_password(&config);
    finalize_output(result, true, false);
}

fn interactive_guid() {
    let selections = &["UUID v4 (Random)", "UUID v7 (Time-ordered)"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select UUID version")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let version = match selection {
        0 => GuidVersion::V4,
        _ => GuidVersion::V7,
    };

    let result = generate_guid(version);
    finalize_output(result, false, false);
}

fn interactive_secret() {
    let length: usize = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Number of bytes")
        .default(32)
        .interact_text()
        .unwrap();

    let selections = &["Hex", "Base64"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select encoding")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let encoding = match selection {
        0 => SecretEncoding::Hex,
        _ => SecretEncoding::Base64,
    };

    let result = generate_secret(length, encoding);
    finalize_output(result, false, false);
}
