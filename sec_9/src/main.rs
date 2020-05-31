use self::lib;

fn main() {
    let mut guess = Guess::new(1);
    guess.value = -1;
    println!("{}", guess.value);
}
