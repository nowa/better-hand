use rs_poker::core::*;

#[derive(PartialEq, Hash, Copy, Clone)]
pub enum Suited {
    On,
    Off,
}

#[derive(PartialEq, Hash, Copy, Clone)]
pub struct PokerHand {
    a: Value,
    b: Value,
    s: Suited,
}

#[derive(Copy, Clone)]
pub struct Beats {
    pub hand: (Card, Card),
    pub we_beat: bool,
}

impl Eq for Suited {}

impl Eq for PokerHand {}

impl Default for PokerHand {
    fn default() -> Self {
        PokerHand {
            a: Value::Ace,
            b: Value::Ace,
            s: Suited::On,
        }
    }
}

impl ToString for PokerHand {
    fn to_string(&self) -> String {
        match self.s {
            Suited::On => format!("{}{}{}", self.a.to_char(), self.b.to_char(), 's'),
            Suited::Off => format!("{}{}{}", self.a.to_char(), self.b.to_char(), 'o'),
        }
    }
}

impl PokerHand {
    pub fn new(hand: (Card, Card)) -> Self {
        let a: Value = hand.0.value;
        let b: Value = hand.1.value;
        let s: Suited = if hand.0.suit == hand.1.suit {
            Suited::On
        } else {
            Suited::Off
        };

        if a > b {
            Self { a: a, b: b, s: s }
        } else {
            Self { a: b, b: a, s: s }
        }
    }

    pub fn grid_pos(&self) -> (u8, u8) {
        match self.s {
            Suited::On => (12 - (self.b as u8), 12 - (self.a as u8)),
            Suited::Off => (12 - (self.a as u8), 12 - (self.b as u8)),
        }
    }
}
