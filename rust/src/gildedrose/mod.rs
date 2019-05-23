use std::string;
use std::vec;

pub struct Item {
    pub name: string::String,
    pub sell_in: i32,
    pub quality: i32
}

impl Item {
    pub fn new(name: String, sell_in: i32, quality: i32) -> Item {
        Item {name: name, sell_in: sell_in, quality: quality}
    }
}

pub struct GildedRose {
    pub items: vec::Vec<Item>
}

trait AdjustmentStrategy {
    fn calculate_adjustment(&self, item:&mut Item) -> i32;

    fn adjust_quality(&self, item:&mut Item) {        
        let adjustment = self.calculate_adjustment(item);
        item.quality += adjustment;
        if item.quality < 0 { item.quality = 0};
        if item.quality > 50 { item.quality = 50};
    }
    
    fn decrement_sell_in(&self, item:&mut Item) {
        item.sell_in -= 1;
    }
}

struct Sulfuras;
impl AdjustmentStrategy for Sulfuras {
    fn adjust_quality(&self, item:&mut Item) {
        // do nothing
    }
    fn calculate_adjustment(&self, item: &mut Item) -> i32 {
        0
    }
    fn decrement_sell_in(&self, item: &mut Item) {
        // do nothing
    }
}


struct AgedBrie;
impl AdjustmentStrategy for AgedBrie {
    fn calculate_adjustment(&self, item: &mut Item) -> i32 {
        if item.sell_in < 0 { 2 } else { 1 }
    }
}

struct BackstagePasses;
impl AdjustmentStrategy for BackstagePasses {
    fn calculate_adjustment(&self, item: &mut Item) -> i32 {
        match item.sell_in {
            s if s < 0 => -1 * item.quality,
            s if s < 5 => 3,
            s if s < 10 => 2,
            _ => 1
        }
    }
}

struct StandardItem;
impl AdjustmentStrategy for StandardItem {
    fn calculate_adjustment(&self, item:&mut Item) -> i32 {
        if item.sell_in < 0  {-2} else {-1}
    }
}


impl GildedRose {
    pub fn new(items: vec::Vec<Item>) -> GildedRose {
        GildedRose {items: items}
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {

            if item.name == "Sulfuras, Hand of Ragnaros" {
                let strategy = Sulfuras;
                strategy.decrement_sell_in(item);
                strategy.adjust_quality(item);
            } else if item.name == "Aged Brie" {
                let strategy = AgedBrie;
                strategy.decrement_sell_in(item);
                strategy.adjust_quality(item);
            } else if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                let strategy = BackstagePasses;
                strategy.decrement_sell_in(item);
                strategy.adjust_quality(item);
            } else {
                let strategy = StandardItem;
                strategy.decrement_sell_in(item);
                strategy.adjust_quality(item);
            }
        }
    }
}

#[cfg(test)]
mod test;
