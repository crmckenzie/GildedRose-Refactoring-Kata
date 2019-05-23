use super::item::Item;

pub trait AdjustmentStrategy {
    fn calculate_adjustment(&self, item:&mut Item) -> i32;

    fn adjust_quality(&self, item:&mut Item) {        
        self.decrement_sell_in(item);
        let adjustment = self.calculate_adjustment(item);
        item.quality += adjustment;
        self.enforce_quality_range_requirements(item);
    }
    
    fn enforce_quality_range_requirements(&self, item:&mut Item) {
        if item.quality < 0 { item.quality = 0};
        if item.quality > 50 { item.quality = 50};
    }

    fn decrement_sell_in(&self, item:&mut Item) {
        item.sell_in -= 1;
    }
}

struct Sulfuras;
impl AdjustmentStrategy for Sulfuras {
    fn enforce_quality_range_requirements(&self, _item:&mut Item){
        // do nothing    
    }

    fn calculate_adjustment(&self, _item: &mut Item) -> i32 {
        0
    }

    fn decrement_sell_in(&self, _item: &mut Item) {
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

struct ConjuredItem;
impl AdjustmentStrategy for ConjuredItem {
    fn calculate_adjustment(&self, item:&mut Item) -> i32 {
        if item.sell_in < 0  {-4} else {-2}
    }
}

pub fn get_strategy(name:&str) -> Box<AdjustmentStrategy> {
    match name {
        _ if name.starts_with("Conjured ") => Box::new(ConjuredItem),
        "Sulfuras, Hand of Ragnaros" => Box::new(Sulfuras),
        "Aged Brie" => Box::new(AgedBrie),
        "Backstage passes to a TAFKAL80ETC concert" => Box::new(BackstagePasses),
        _ => Box::new(StandardItem)
    }
}