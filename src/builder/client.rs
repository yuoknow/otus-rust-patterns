#[derive(Debug)]
pub struct Client {
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub email: String,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }
}

#[derive(Default)]
pub struct ClientBuilder {
    first_name: String,
    last_name: String,
    address: String,
    email: String,
}

impl ClientBuilder {
    pub fn first_name(mut self, first_name: String) -> ClientBuilder {
        self.first_name = first_name;
        self
    }

    pub fn last_name(mut self, last_name: String) -> ClientBuilder {
        self.last_name = last_name;
        self
    }

    pub fn address(mut self, address: String) -> ClientBuilder {
        self.address = address;
        self
    }

    pub fn email(mut self, email: String) -> ClientBuilder {
        self.email = email;
        self
    }

    pub fn build(self) -> Client {
        Client {
            first_name: self.first_name,
            last_name: self.last_name,
            address: self.address,
            email: self.email,
        }
    }
}
