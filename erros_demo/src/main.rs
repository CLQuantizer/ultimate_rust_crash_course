fn main() {
    let s1 = String::from("Kaggle");
    let s1 = takes_and_returns(s1);
    println!("I am {}", s1);
}

fn takes_and_returns(prey: String) -> String{
    println!("I lowkey possess the soul of {}", prey);
    prey
}
