use crate::to_pig_latin::to_pig_latin;
mod to_pig_latin;

fn main() {
    let input_text: &str = "apple dog";
    println!("{} -> {}", input_text, to_pig_latin(input_text));
}
