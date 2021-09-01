pub mod card;
pub mod deck;
pub mod player;
pub mod packet;

pub mod prelude {
    pub use crate::{
        card::{ self, Card },
        player::*,
        deck::*,
        packet,
    };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn full_deck_size() {
        assert_eq!(Deck::full().size(), 108);
    }
}
