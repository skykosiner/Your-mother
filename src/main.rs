struct YourMom {
    weight: f64
}

impl YourMom {
    fn get_weight(&self) {
        println!("Your mother's weight is {}lb", self.weight)
    }

    fn deez_nuts(&self) {
        println!("Your mother seems to be so fat, that the only thing that can get into her is deez nuts")
    }
}

fn main() {
    let your_mom = YourMom { weight: 69.420 };
    your_mom.get_weight();
    your_mom.deez_nuts();
}
