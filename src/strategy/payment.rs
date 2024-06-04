pub trait PaymentStrategy {
    fn pay(&self, amount: u32) -> String;
}

pub struct Bonus;

impl PaymentStrategy for Bonus {
    fn pay(&self, amount: u32) -> String {
        format!("Paid using bonuses {}", amount)
    }
}

pub struct DebitCard;

impl PaymentStrategy for DebitCard {
    fn pay(&self, amount: u32) -> String {
        format!("Paid using debit card {}", amount)
    }
}

pub struct ShopItem {
    pub count: u32,
    pub price: u32,
}

pub struct Checkout;
impl Checkout {
    pub fn checkout<T: PaymentStrategy>(payment_method: T, items: Vec<ShopItem>) -> String {
        let mut amount = 0;
        for item in items {
            amount += item.count * item.price
        }
        payment_method.pay(amount)
    }
}
