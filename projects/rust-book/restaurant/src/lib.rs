mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

        mod back_of_house {
            pub enum Appetizer {
                Soup,
                Salad,
            }

            pub struct Breakfast {
                pub toast: String,
                seasonal_fruit: String,
            }

            impl Breakfast {
                pub fn summer(toast: &str) -> Breakfast {
                    Breakfast {
                        toast: String::from(toast),
                        seasonal_fruit: String::from("peaches"),
                    }
                }
            }

            fn fix_incorrect_order() {
                cook_over();
                super::serve_order();
            }

            fn cook_over() {}
        }

        //use crate::front_of_house::hosting;
        use super::hosting; // same with above

        pub fn eat_at_restaurant() {
            let mut meal = back_of_house::Breakfast::summer("Rye");

            meal.toast = String::from("Wheat");
            println!("I'd like {} toast please", meal.toast);

            let order1 = back_of_house::Appetizer::Soup;
            let order2 = back_of_house::Appetizer::Salad;

            // with use
            hosting::add_to_waitlist();
            hosting::add_to_waitlist();
        }
    }
}
