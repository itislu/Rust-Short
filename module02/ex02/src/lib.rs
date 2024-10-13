#![allow(dead_code)]

/*
  Once a pizza has been ordered, it takes two days before the cook start working on it.
  Making a pizza takes roughly 5 days.
  Once the pizza is ready, the only delivery man must pick it up. It takes 3 days on average.
  Delivering the pizza always takes a whole week.
*/
#[derive(PartialEq, Debug)]
enum PizzaStatus {
    Ordered,
    Cooking,
    Cooked,
    Delivering,
    Delivered,
}

impl PizzaStatus {
    fn from_delivery_time(ordered_days_ago: u32) -> Self {
        match ordered_days_ago {
            0..2 => PizzaStatus::Ordered,
            2..7 => PizzaStatus::Cooking,
            7..10 => PizzaStatus::Cooked,
            10..17 => PizzaStatus::Delivering,
            17.. => PizzaStatus::Delivered,
        }
    }

    fn get_delivery_time_in_days(&self) -> u32 {
        match self {
            PizzaStatus::Ordered => 17,
            PizzaStatus::Cooking => 15,
            PizzaStatus::Cooked => 10,
            PizzaStatus::Delivering => 7,
            PizzaStatus::Delivered => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(PizzaStatus::from_delivery_time(0), PizzaStatus::Ordered);
        assert_eq!(PizzaStatus::from_delivery_time(100), PizzaStatus::Delivered);
    }

    #[test]
    fn in_days() {
        assert_eq!(PizzaStatus::from_delivery_time(15).get_delivery_time_in_days(), 7);
    }
}
