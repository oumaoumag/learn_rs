pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
    pub fn new(alias: &str) -> Self {
        Light {
            alias: String::from(alias), // Convert &str to String
            brightness: 0, // Start at 0 brightness
        }
    }
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    for light in lights.iter_mut() {
        if light.alias == alias {
            light.brightness = value;
            break; // Stop once we find and update the light
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_creation_and_brightness_change() {
        let mut lights = vec![Light::new("kitchen")];
        assert_eq!(lights[0].brightness, 0); // check initial brightness
        change_brightness(&mut lights, "kitchen", 150);
        assert_eq!(lights[0].brightness, 150); // Check updated brightness
    }
}
