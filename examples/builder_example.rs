use patterns::builder::client::Client;

fn main() {
    let client = Client::builder()
        .first_name("Harry".to_string())
        .last_name("Potter".to_string())
        .address("4 Privet Drive, Little Whinging, Surrey".to_string())
        .email("harry@mage.com".to_string())
        .build();

    println!("{:?}", client)
}
