pub struct One {
    pub first_layer: Option<Two>,
}

pub struct Two {
    pub second_layer: Option<Three>,
}

pub struct Three {
    pub third_layer: Option<Four>,
}

pub struct Four {
    pub fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        let two = self.first_layer?;
        let three = two.second_layer?;
        let four = three.third_layer?;
        four.fourth_layer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fourth_layer_some() {
        let a = One {
            first_layer: Some(Two {
                second_layer: Some(Three {
                    third_layer: Some(Four {
                        fourth_layer: Some(1000)
                    })
                })
            })
        };

        assert_eq!(a.get_fourth_layer(), Some(1000));
    }

    #[test]
    fn test_get_fourth_layer_none_at_first() {
        let a = One {
            first_layer: None,
        };

        assert_eq!(a.get_fourth_layer(), None);
    }

    #[test]
    fn test_get_fourth_layer_none_at_second() {
        let a = One {
            first_layer: Some(Two {
                second_layer: None,
            })
        };

        assert_eq!(a.get_fourth_layer(), None);
    }

    #[test]
    fn test_get_fourth_layer_none_at_third() {
        let a = One {
            first_layer: Some(Two {
                second_layer: Some(Three {
                    third_layer: None,
                })
            })
        };

        assert_eq!(a.get_fourth_layer(), None);
    }

    #[test]
    fn test_get_fourth_layer_none_at_fourth() {
        let a = One {
            first_layer: Some(Two {
                second_layer: Some(Three {
                    third_layer: Some(Four {
                        fourth_layer: None,
                    })
                })
            })
        };

        assert_eq!(a.get_fourth_layer(), None);
    }
}
