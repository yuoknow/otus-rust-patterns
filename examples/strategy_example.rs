use patterns::strategy::payment::{Checkout, DebitCard, ShopItem};

fn main() {
    let result = Checkout::checkout(
        DebitCard,
        vec![
            ShopItem {
                count: 1,
                price: 10,
            },
            ShopItem {
                count: 2,
                price: 15,
            },
        ],
    );

    println!("{}", result)
}
