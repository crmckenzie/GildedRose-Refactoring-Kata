use std::vec;
use super::Item as Item;
use super::adjustment_strategy::get_strategy;

pub struct GildedRose {
    pub items: vec::Vec<Item>
}

impl GildedRose {
    pub fn new(items: vec::Vec<Item>) -> GildedRose {
        GildedRose {items: items}
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            let strategy = get_strategy(&item.name);
            strategy.adjust_quality(item);
        }
    }
}