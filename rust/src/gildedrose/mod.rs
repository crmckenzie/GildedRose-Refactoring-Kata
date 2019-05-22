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

fn adjust_quality(item:&mut Item, adjustment: i32) {

    item.quality += adjustment;

    if item.quality < 0 { item.quality = 0};
    if item.quality > 50 { item.quality = 50};
}

impl GildedRose {
    pub fn new(items: vec::Vec<Item>) -> GildedRose {
        GildedRose {items: items}
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            if item.name == "Sulfuras, Hand of Ragnaros" {
                break;
            }

            item.sell_in = item.sell_in - 1;
            let mut adjustment = -1;

            if item.name == "Aged Brie" {
                adjustment = if item.sell_in < 0 { 2 } else {1};
            } else if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                adjustment = if item.sell_in < 0 {
                    -1 * item.quality
                } else if item.sell_in < 5 {
                    3
                } else if item.sell_in < 10 {
                    2
                } else {
                    1
                };
            } else {
                adjustment = if item.sell_in < 0 {-2} else {-1};
            }

            adjust_quality(item, adjustment);
        }
    }
}

#[cfg(test)]
mod test;
