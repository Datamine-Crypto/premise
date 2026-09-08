use patterns::Named;

pub enum Rarity {
    Common,
    Rare,
    Epic,
}

impl Named for Rarity {
    fn text(&self) -> &'static str {
        match self {
            Rarity::Common => "common",
            Rarity::Rare => "rare",
            Rarity::Epic => "epic",
        }
    }
}
