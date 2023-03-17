struct YourMom {
    weight: f64
}

impl YourMom {
    fn get_weight(&self) {
        println!("Your mother's weight is {}lb", self.weight)
    }
}

fn main() {
    let your_mom = YourMom { weight: 69.420 };
    your_mom.get_weight();
}
