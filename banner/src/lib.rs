use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Self {
            short_hand: format!("-{}", &name[0..1]),
            long_hand: format!("--{}", name),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        // Store the callback for both short_hand and long_hand
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        if let Some(callback) = self.flags.get(input) {
            if argv.len() < 2 {
                return Err("Not enough arguments".to_string());
            }
            return callback(argv[0], argv[1]).map_err(|e| e.to_string());
        }
        Err(format!("Flag {} not found", input))
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_float = a.parse::<f64>()?;
    let b_float = b.parse::<f64>()?;
    Ok((a_float / b_float).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_float = a.parse::<f64>()?;
    let b_float = b.parse::<f64>()?;
    Ok((a_float % b_float).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_creation() {
        let flag = Flag::opt_flag("test", "test description");
        assert_eq!(flag.short_hand, "-t");
        assert_eq!(flag.long_hand, "--test");
        assert_eq!(flag.desc, "test description");
    }

    #[test]
    fn test_div_function() {
        assert_eq!(div("10", "2").unwrap(), "5");
        assert!(div("a", "2").is_err());
        assert!(div("10", "b").is_err());
    }

    #[test]
    fn test_rem_function() {
        assert_eq!(rem("10", "3").unwrap(), "1");
        assert!(rem("a", "3").is_err());
        assert!(rem("10", "b").is_err());
    }

    #[test]
    fn test_flags_handler() {
        let mut handler = FlagsHandler { flags: HashMap::new() };
        let d = Flag::opt_flag("division", "divides the values");
        let r = Flag::opt_flag("remainder", "remainder of division");

        handler.add_flag(d, div);
        handler.add_flag(r, rem);

        assert_eq!(handler.exec_func("-d", &["10", "2"]).unwrap(), "5");
        assert_eq!(handler.exec_func("--division", &["10", "2"]).unwrap(), "5");
        assert_eq!(handler.exec_func("-r", &["10", "3"]).unwrap(), "1");
        assert_eq!(handler.exec_func("--remainder", &["10", "3"]).unwrap(), "1");
        assert!(handler.exec_func("-d", &["a", "2"]).is_err());
        assert!(handler.exec_func("-r", &["10", "b"]).is_err());
        assert!(handler.exec_func("-x", &["10", "2"]).is_err());
    }

    #[test]
    fn test_example_from_instructions() {
        let mut handler = FlagsHandler { flags: HashMap::new() };

        let d = Flag::opt_flag("division", "divides the values, formula (a / b)");
        let r = Flag::opt_flag(
            "remainder",
            "remainder of the division between two values, formula (a % b)",
        );

        handler.add_flag(d, div);
        handler.add_flag(r, rem);

        assert_eq!(handler.exec_func("-d", &["1.0", "2.0"]).unwrap(), "0.5");
        assert_eq!(handler.exec_func("-r", &["2.0", "2.0"]).unwrap(), "0");
        assert!(handler.exec_func("--division", &["a", "2.0"]).is_err());
        assert!(handler.exec_func("--remainder", &["2.0", "fd"]).is_err());
    }
}
