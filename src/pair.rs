use std::fmt;


pub struct Pair {
    name: String,
    reserve_0: u128,
    reserve_1: u128,
    price: f64,
}

impl Pair {
    pub fn new(name: &str, reserve_0: u128, reserve_1: u128, price: f64) -> Pair {
        Pair {
            name:name.to_string(),
            reserve_0,
            reserve_1,
            price,
        }
    }


}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tokens:Vec<&str> = self.name.split("-").collect();
        write!(
            f,
            "Name: {}\n{}: {}\n{}: {}\nPrice: {}\n",
            self.name, tokens[0], self.reserve_0, tokens[1], self.reserve_1, self.price
        )
    }
}