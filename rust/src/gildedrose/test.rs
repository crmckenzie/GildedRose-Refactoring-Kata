use super::{Item, GildedRose};

fn update_quality(name: &str, sell_in: i32, quality: i32 ) -> Item{
    let item = Item::new(String::from(name), sell_in, quality);
    let items = vec![item];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    let result = rose.items.pop().unwrap();
    result
}

#[test]
pub fn standard_item() {
    let result = update_quality("foo", 10, 20);

    assert_eq!("foo", result.name);
    assert_eq!(9, result.sell_in);
    assert_eq!(19, result.quality);
}

#[test]
pub fn after_sell_in_has_passed_degrade_quality_faster() {
    let result = update_quality("foo", 0, 20);

    assert_eq!("foo", result.name);
    assert_eq!(-1, result.sell_in);
    assert_eq!(18, result.quality);
}


#[test]
pub fn quality_cannot_be_negative() {
    let result = update_quality("foo", 0, 0);

    assert_eq!("foo", result.name);
    assert_eq!(-1, result.sell_in);
    assert_eq!(0, result.quality);
}


#[test]
pub fn aged_brie_increases_in_quality_with_age() {
    let result = update_quality("Aged Brie", 10, 20);

    assert_eq!("Aged Brie", result.name);
    assert_eq!(9, result.sell_in);
    assert_eq!(21, result.quality);
}


#[test]
pub fn aged_brie_increases_in_quality_faster_after_sell_by() {
    let result = update_quality("Aged Brie", 0, 20);

    assert_eq!(-1, result.sell_in);
    assert_eq!(22, result.quality);
}

#[test]
pub fn quality_cannot_exceed_50() {
    let result = update_quality("Aged Brie", 0, 50);

    assert_eq!(-1, result.sell_in);
    assert_eq!(50, result.quality);
}

#[test]
pub fn sulfuras_is_immutable() {
    let result = update_quality("Sulfuras, Hand of Ragnaros", 0, 80);

    assert_eq!(0, result.sell_in);
    assert_eq!(80, result.quality);
}


#[test]
pub fn backstage_passes_increase_in_quality_with_age() {
    let result = update_quality("Backstage passes to a TAFKAL80ETC concert", 20, 30);

    assert_eq!(19, result.sell_in);
    assert_eq!(31, result.quality);
}


