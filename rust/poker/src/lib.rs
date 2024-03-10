mod hand;
mod card;

use hand::Hand;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut sorted_hands = hands.iter()
        .map(|str| Hand::new(str))
        .collect::<Vec<Hand>>();

    sorted_hands.sort_by(|lhs, rhs| lhs.hand_rating().cmp(&rhs.hand_rating()));

    let mut winning_hands: Vec<&'a str> = vec![];
    let mut max_score: u32 = 0;
    sorted_hands.iter().rev().for_each(|h| {
        let score = h.hand_rating();
        if score >= max_score {
            max_score = score;
            winning_hands.push(h.str_hand);
        }
    });

    return winning_hands;
}