use crate::utils::exit_with_message;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Card {
  A = 14,
  K = 13,
  Q = 12,
  // J = 11,
  T = 10,
  Nine = 9,
  Eighth = 8,
  Seven = 7,
  Six = 6,
  Five = 5,
  Four = 4,
  Three = 3,
  Two = 2,
  J = 1,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Type {
  FiveOfAKind = 6,
  FourOfAKind = 5,
  FullHouse = 4,
  ThreeOfAKind = 3,
  TwoPair = 2,
  OnePair = 1,
  HighCard = 0,
}

#[derive(Debug)]
pub struct Hand {
  pub cards: Vec<Card>,
  pub bid: i32,
  pub _type: Type,
}

impl Hand {
  pub fn new(line: &str, is_jokers_enabled: bool) -> Self {
    let hand: Vec<&str> = line.split_whitespace().collect();
    let bid = hand[1].parse::<i32>().unwrap();
    let cards_str = hand[0];

    let mut cards: Vec<Card> = Vec::with_capacity(5);
    for card in cards_str.chars() {
      match card {
        'A' => cards.push(Card::A),
        'K' => cards.push(Card::K),
        'Q' => cards.push(Card::Q),
        'J' => cards.push(Card::J),
        'T' => cards.push(Card::T),
        '9' => cards.push(Card::Nine),
        '8' => cards.push(Card::Eighth),
        '7' => cards.push(Card::Seven),
        '6' => cards.push(Card::Six),
        '5' => cards.push(Card::Five),
        '4' => cards.push(Card::Four),
        '3' => cards.push(Card::Three),
        '2' => cards.push(Card::Two),
        _ => exit_with_message(format!("Unknown card {}", card).as_str()),
      }
    }

    let mut _type = calculate_hand_type(&cards, is_jokers_enabled);

    Hand { cards, bid, _type }
  }
}

fn calculate_hand_type(cards: &Vec<Card>, is_jokers_enabled: bool) -> Type {
  let mut processed_cards: Vec<&Card> = Vec::with_capacity(5);
  let mut _type = Type::HighCard;

  for card in cards {
    if processed_cards.contains(&card) {
      continue;
    }

    let same_cards = cards.iter().filter(|&c| c == card);
    let count = same_cards.count();
    processed_cards.push(card);

    // Skip jokers if they are enabled and handle them in the end instead
    if is_jokers_enabled && card == &Card::J {
      continue;
    }

    // Do nothing if we have a single card
    if count == 1 {
      continue;
    }

    if count == 5 {
      return Type::FiveOfAKind;
    } else if count == 4 {
      _type = Type::FourOfAKind;
    } else if count == 3 {
      if _type == Type::OnePair {
        _type = Type::FullHouse;
      } else {
        _type = Type::ThreeOfAKind;
      }
    } else if count == 2 {
      if _type == Type::ThreeOfAKind {
        _type = Type::FullHouse;
      } else if _type == Type::OnePair {
        _type = Type::TwoPair;
      } else {
        _type = Type::OnePair;
      }
    }
  }

  if is_jokers_enabled && cards.contains(&Card::J) {
    let joker_count = cards.iter().filter(|&c| c == &Card::J).count();
    _type = handle_jokers(_type, joker_count);
  }

  _type
}

fn handle_jokers(_type: Type, joker_count: usize) -> Type {
  if joker_count == 1 {
    // 4 cars to play with
    if _type == Type::HighCard {
      return Type::OnePair;
    }
    if _type == Type::OnePair {
      return Type::ThreeOfAKind;
    }
    if _type == Type::TwoPair {
      return Type::FullHouse;
    }
    if _type == Type::ThreeOfAKind || _type == Type::FullHouse {
      return Type::FourOfAKind;
    }
    if _type == Type::FourOfAKind {
      return Type::FiveOfAKind;
    }
  }

  if joker_count == 2 {
    // 3 cars to play with
    if _type == Type::HighCard {
      return Type::ThreeOfAKind;
    }
    if _type == Type::OnePair {
      return Type::FourOfAKind;
    }
    if _type == Type::ThreeOfAKind {
      return Type::FiveOfAKind;
    }
  }
  if joker_count == 3 {
    // 2 cars to play with
    if _type == Type::HighCard {
      return Type::FourOfAKind;
    }
    if _type == Type::OnePair {
      return Type::FiveOfAKind;
    }
  }

  if joker_count == 4 || joker_count == 5 {
    return Type::FiveOfAKind;
  }

  exit_with_message(
    format!(
      "Failed to handle jokers, not all cases have been handled: {:?} with {} jokers",
      _type, joker_count
    )
    .as_str(),
  );
}
