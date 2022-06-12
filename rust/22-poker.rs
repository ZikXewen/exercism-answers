#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    rank: u8,
    suit: Suit,
}
impl From<&str> for Card {
    fn from(card: &str) -> Self {
        let (c1, c2) = card.split_at(card.len() - 1);
        Self {
            rank: match c1 {
                "J" => 9,
                "Q" => 10,
                "K" => 11,
                "A" => 12,
                _ => c1.parse::<u8>().expect("Unknown Rank") - 2,
            },
            suit: match c2 {
                "C" => Suit::Clubs,
                "D" => Suit::Diamonds,
                "H" => Suit::Hearts,
                "S" => Suit::Spades,
                _ => panic!("Unknown Suit"),
            },
        }
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Hand {
    High(u8, u8, u8, u8, u8),
    Pair(u8, u8, u8, u8),
    TwoPair(u8, u8, u8),
    Trips(u8, u8, u8),
    Straight(u8),
    Flush(u8, u8, u8, u8, u8),
    FullHouse(u8, u8),
    Quads(u8, u8),
    StraightFlush(u8),
}
impl From<Vec<Card>> for Hand {
    fn from(mut cards: Vec<Card>) -> Self {
        cards.sort_unstable();
        cards.reverse();
        let (ranks, suits): (Vec<u8>, Vec<Suit>) = cards.iter().map(|x| (x.rank, x.suit)).unzip();
        if ranks[0] == ranks[3] {
            return Self::Quads(ranks[2], ranks[4]);
        }
        if ranks[1] == ranks[4] {
            return Self::Quads(ranks[2], ranks[0]);
        }
        let mut singles = vec![];
        let mut pairs = vec![];
        let mut trips = vec![];
        {
            let mut i = 0;
            while i < 5 {
                if ranks.get(i + 2).map_or(false, |&x| x == ranks[i]) {
                    trips.push(ranks[i]);
                    i += 3;
                } else if ranks.get(i + 1).map_or(false, |&x| x == ranks[i]) {
                    pairs.push(ranks[i]);
                    i += 2;
                } else {
                    singles.push(ranks[i]);
                    i += 1;
                }
            }
        }
        match (pairs.len(), trips.len()) {
            (1, 1) => Self::FullHouse(trips[0], pairs[0]),
            (0, 1) => Self::Trips(trips[0], singles[0], singles[1]),
            (2, 0) => Self::TwoPair(pairs[0], pairs[1], singles[0]),
            (1, 0) => Self::Pair(pairs[0], singles[0], singles[1], singles[2]),
            _ => {
                let is_flush = suits.iter().all(|&x| x == suits[0]);
                let is_straight = (0..4).all(|i| {
                    (ranks[i] == ranks[i + 1] + 1) || (ranks[i] == 12 && ranks[i + 1] == 3)
                });
                match (is_flush, is_straight) {
                    (true, true) => Self::StraightFlush(if ranks[0] - ranks[1] == 1 { ranks[0] } else {3}),
                    (true, false) => Self::Flush(ranks[0], ranks[1], ranks[2], ranks[3], ranks[4]),
                    (false, true) => Self::Straight(if ranks[0] - ranks[1] == 1 { ranks[0] } else {3}),
                    (false, false) => Hand::High(ranks[0], ranks[1], ranks[2], ranks[3], ranks[4]),
                }
            }
        }
    }
}

fn create_cards(x: &str) -> Vec<Card> {
    x.split_whitespace().map(|s| Card::from(s)).collect()
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut vec = hands
        .iter()
        .map(|&x| (Hand::from(create_cards(x)), x))
        .collect::<Vec<_>>();
    vec.sort();
    let (win, s1) = vec.pop().unwrap();
    let mut ret = vec![s1];
    while let Some((hand, n)) = vec.pop() {
        if hand != win {
            break;
        }
        ret.push(n);
    }
    ret
}

// #[test]
// fn test() {
//     dbg!(&Hand::from(create_cards("4S AH 3S 2D 5H")));
// }
