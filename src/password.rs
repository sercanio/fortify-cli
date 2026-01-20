use rand::Rng;

pub struct PasswordConfig {
    pub length: usize,
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub symbols: bool,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            length: 16,
            uppercase: true,
            lowercase: true,
            numbers: true,
            symbols: true,
        }
    }
}

pub fn generate_password(config: &PasswordConfig) -> String {
    let mut charset = String::new();
    if config.uppercase { charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); }
    if config.lowercase { charset.push_str("abcdefghijklmnopqrstuvwxyz"); }
    if config.numbers { charset.push_str("0123456789"); }
    if config.symbols { charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?"); }

    if charset.is_empty() {
        return String::from("Error: No character set selected.");
    }

    let mut rng = rand::rng();
    (0..config.length)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_length() {
        let config = PasswordConfig {
            length: 20,
            ..Default::default()
        };
        let password = generate_password(&config);
        assert_eq!(password.len(), 20);
    }

    #[test]
    fn test_password_charset() {
        let config = PasswordConfig {
            length: 100,
            uppercase: false,
            lowercase: true,
            numbers: false,
            symbols: false,
        };
        let password = generate_password(&config);
        assert!(password.chars().all(|c| c.is_lowercase()));
    }
}
