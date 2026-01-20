mod password;
mod guid;
mod secret;

use clap::{Parser, Subcommand, ValueEnum};
use dialoguer::{theme::ColorfulTheme, Select, Input, MultiSelect};
use password::{PasswordConfig, generate_password};
use guid::{GuidVersion, generate_guid};
use secret::{SecretEncoding, generate_secret};
use arboard::Clipboard;
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
        
        #[arg(long, default_value_t = false)]
        no_uppercase: bool,
        
        #[arg(long, default_value_t = false)]
        no_lowercase: bool,
        
        #[arg(long, default_value_t = false)]
        no_numbers: bool,
        
        #[arg(long, default_value_t = false)]
        no_symbols: bool,
    },
    Guid {
        #[arg(short, long, value_enum, default_value_t = CliGuidVersion::V4)]
        version: CliGuidVersion,
    },
    Secret {
        #[arg(short, long, default_value_t = 32)]
        length: usize,
        #[arg(short, long, value_enum, default_value_t = CliSecretEncoding::Hex)]
        encoding: CliSecretEncoding,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CliGuidVersion {
    V4,
    V7,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CliSecretEncoding {
    Hex,
    Base64,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Password { length, no_uppercase, no_lowercase, no_numbers, no_symbols }) => {
             let config = PasswordConfig {
                 length: *length,
                 uppercase: !no_uppercase,
                 lowercase: !no_lowercase,
                 numbers: !no_numbers,
                 symbols: !no_symbols,
             };
             let result = generate_password(&config);
             finalize_output(result, true, cli.no_copy);
        },
        Some(Commands::Guid { version }) => {
            let v = match version {
                CliGuidVersion::V4 => GuidVersion::V4,
                CliGuidVersion::V7 => GuidVersion::V7,
            };
            let result = generate_guid(v);
            finalize_output(result, false, cli.no_copy);
        },
        Some(Commands::Secret { length, encoding }) => {
             let enc = match encoding {
                 CliSecretEncoding::Hex => SecretEncoding::Hex,
                 CliSecretEncoding::Base64 => SecretEncoding::Base64,
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
        // Simple feedback if available
        if let Some(feedback) = estimate.feedback() {
             if let Some(warning) = feedback.warning() {
                 println!("Warning: {}", warning);
             }
        }
    }

    if !no_copy {
        // Attempt to copy to clipboard
        // Note: In some headless environments this might fail, we catch it.
        match Clipboard::new() {
            Ok(mut clipboard) => {
                if let Err(_) = clipboard.set_text(text) {
                    // Silently fail or minimal error if desired, but user might expect copy
                } else {
                     println!("(Copied to clipboard)");
                }
            },
            Err(_) => {
                // Clipboard not available (e.g. server without X11)
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
    finalize_output(result, true, false); // Interactive mode defaults to auto-copy
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
