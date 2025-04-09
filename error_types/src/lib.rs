use chrono::Local;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        FormError {
            form_values: (field_name, field_value),
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn new(name: String, password: String) -> Self {
        Form { name, password }
    }

    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new(
                "name",
                self.name.clone(),
                "Username is empty",
            ));
        }

        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be at least 8 characters long",
            ));
        }

        let has_letter = self.password.chars().any(|c| c.is_ascii_alphabetic());
        let has_number = self.password.chars().any(|c| c.is_ascii_digit());
        let has_symbol = self.password.chars().any(|c| {
            c.is_ascii() && !c.is_ascii_alphanumeric()
        });

        if !has_letter || !has_number || !has_symbol {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_form() {
        let form = Form {
            name: "Lee".to_owned(),
            password: "qwqwsa1dty_".to_owned(),
        };
        assert_eq!(form.validate(), Ok(()));
    }

    #[test]
    fn test_empty_name() {
        let form = Form {
            name: "".to_owned(),
            password: "qwqwsa1dty_".to_owned(),
        };
        assert!(form.validate().is_err());
    }

    #[test]
    fn test_short_password() {
        let form = Form {
            name: "Lee".to_owned(),
            password: "dty_1".to_owned(),
        };
        assert!(form.validate().is_err());
    }

    #[test]
    fn test_password_missing_number() {
        let form = Form {
            name: "Lee".to_owned(),
            password: "asdasASd(_".to_owned(),
        };
        assert!(form.validate().is_err());
    }

    #[test]
    fn test_password_missing_symbol() {
        let form = Form {
            name: "Lee".to_owned(),
            password: "asdasASd123SA".to_owned(),
        };
        assert!(form.validate().is_err());
    }
}
