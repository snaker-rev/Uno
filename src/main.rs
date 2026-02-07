
//use rand:Rng

#[derive(Debug)]
enum Color {Red,Blue,Green}
#[derive(Debug)]
struct Card {color:Color,value:i32}
fn main() {
    // let no=(1,2,3,4,5,6,7,8,9);
    // let color=("red","green","blue","yellow");
    // let power=("reverse","block","c_color");
    let mut mycard = Card{color:Color::Red,value:7};
    let mut deck = Vec::new();
    deck.push(mycard);
    //let card = (1, "red", 0);
    println!("{:?}", deck);
}
