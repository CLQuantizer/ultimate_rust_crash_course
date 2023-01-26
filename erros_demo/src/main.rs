fn main() {
    let s1 = String::from("Kaggle");
    borrows(&s1);
    println!("I am {}", s1);

}

fn borrows(prey: &String){
    prey.push_str(" is so dead");
}
