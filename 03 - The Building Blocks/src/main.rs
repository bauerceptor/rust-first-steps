#[derive(Debug)]
struct Deck {
  cards: Vec<String>,
}

fn main() {
  // suits - Hearts, Clubs, Diamonds, Spades
  let suits = ["Hearts", "Clubs", "Diamonds", "Spades"];
  
  // values - Ace, Two, Three, Four
  let values = ["Ace", "Two", "Three"];
  
  let mut cards: Vec<String> = vec![];
  
  for suit in suits {
    for val in values {
      let card = format!("{} of {}", val, suit);
      cards.push(card);
    }
  }
  
  let my_deck = Deck {
    cards: cards,
  };

  

  println!("my deck is: {:#?}", my_deck);
}

