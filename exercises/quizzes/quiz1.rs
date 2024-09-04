// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
// fn calculate_price_of_apples(???) -> ??? { ??? }

use std::cmp::Ordering;

fn main() {
    // You can optionally experiment here.
}

fn calculate_price_of_apples(apples_quantity: u16) -> u16 {
    const MINIMUM_APPLES_QUANTITY_TO_DISCOUNT: u16 = 41;
    const REGULAR_PRICE_OF_APPLES: u16 = 2;
    const PRICE_OF_APPLES_WITH_DISCOUNT: u16 = 1;

    let price_of_apples = match apples_quantity.cmp(&MINIMUM_APPLES_QUANTITY_TO_DISCOUNT) {
        Ordering::Less => REGULAR_PRICE_OF_APPLES,
        _ => PRICE_OF_APPLES_WITH_DISCOUNT,
    };

    apples_quantity * price_of_apples
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
