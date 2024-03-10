#[derive(Eq, Hash, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    fn from(input: char) -> Self {
        match input {
            'H' => Suit::Hearts,
            'D' => Suit::Diamonds,
            'C' => Suit::Clubs,
            'S' => Suit::Spades,
            _ => panic!("invalid suit value"),
        }
    }
}

pub struct Card {
    pub value: u8,
    pub suit: Suit
}

impl Card {
    pub fn new(data: &str) -> Self {
        let (value, suit) = Self::get_components(data);

        let mut card = Card {value: 0 , suit: Suit::Hearts};

        match value {
            Some(v) => {
                match v {
                    '0' => card.value = 10,
                    'J' => card.value = 11,
                    'Q' => card.value = 12,
                    'K' => card.value = 13,
                    _ => card.value = v as u8
                }
            },
            None => panic!("invalid card value")
        }

        match suit {
            Some(v) => {
                card.suit = Suit::from(v)
            },
            None => panic!("invalid card suit")
        }

        return card;
    }

    // Function `get_components` returns two optional values of a char. These chars represent the value and the suit respectively.
    // It's important to note that a card with a value of `10H` would be translated to a value of `0` and a suit of `H`.
    fn get_components(data: &str) -> (Option<char>, Option<char>) {
        let mut chars = data.chars();
        let value: Option<char>;
        let suit: Option<char>;
        
        match data.len() {
            2 => {
                value = chars.next();
                suit = chars.next();
            },
            // 2 digits value or invalid card
            3 => {
                 if data[0..2].eq("10") {
                    value = Some('0')
                 } else {
                    panic!("invalid card value")
                 }
                 suit = chars.nth(2);
            },
            _ => panic!("invalid card value")
        }

        return (value, suit);
    }
}