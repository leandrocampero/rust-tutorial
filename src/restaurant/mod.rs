mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }

    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            return Pizza {
                dough: String::from("Regular dough"),
                cheese: String::from("Mozzarella"),
                topping: String::from(topping),
            };
        }
    }

    pub mod help_customer {
        fn seat_at_table() {
            println!("Customer seated at table");
        }

        pub fn take_order() {
            seat_at_table();
            let customer_pizza: super::Pizza = super::Pizza::lunch("veggies");
            serve_customer(customer_pizza);
        }

        fn serve_customer(pizza: super::Pizza) {
            println!(
                "The customer is served a regular pizza with {}",
                pizza.topping
            )
        }
    }
}

pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order();
}
