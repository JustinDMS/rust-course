//! # Private by default (solution)
//!
//! `front_of_house`, `hosting` and `back_of_house` needed `pub` so the
//! tests (in the crate root) can walk down to them. `seat_at_table`,
//! `take_order` and `take_payment` are only called from inside their own
//! module, so they stay private.

mod restaurant {
    pub mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() -> &'static str {
                seat_at_table()
            }

            fn seat_at_table() -> &'static str {
                "seated"
            }
        }

        pub mod serving {
            fn take_order() -> &'static str {
                "order taken"
            }

            pub fn serve_order() -> String {
                format!("{}, served", take_order())
            }

            fn take_payment() -> u32 {
                42
            }

            pub fn close_table() -> u32 {
                take_payment()
            }
        }
    }

    pub mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: toast.to_string(),
                    seasonal_fruit: String::from("peaches"),
                }
            }

            pub fn fruit(&self) -> &str {
                &self.seasonal_fruit
            }
        }

        pub enum Appetizer {
            Soup,
            Salad,
        }
    }

    pub fn eat_at_restaurant() -> String {
        let mut meal = back_of_house::Breakfast::summer("rye");
        meal.toast = String::from("wheat");
        format!("{} toast with {}", meal.toast, meal.fruit())
    }
}

#[test]
fn waitlist_is_reachable() {
    assert_eq!(restaurant::front_of_house::hosting::add_to_waitlist(), "seated");
}

#[test]
fn serving_is_reachable() {
    assert_eq!(restaurant::front_of_house::serving::serve_order(), "order taken, served");
    assert_eq!(restaurant::front_of_house::serving::close_table(), 42);
}

#[test]
fn breakfast_has_a_public_constructor_and_toast() {
    let mut meal = restaurant::back_of_house::Breakfast::summer("rye");
    meal.toast = String::from("sourdough");
    assert_eq!(meal.toast, "sourdough");
    assert_eq!(meal.fruit(), "peaches");
}

#[test]
fn appetizers_are_public() {
    let _soup = restaurant::back_of_house::Appetizer::Soup;
    let _salad = restaurant::back_of_house::Appetizer::Salad;
}

#[test]
fn eating_works() {
    assert_eq!(restaurant::eat_at_restaurant(), "wheat toast with peaches");
}
