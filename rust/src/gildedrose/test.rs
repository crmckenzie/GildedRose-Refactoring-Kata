use gildedrose::{Item, GildedRose};

fn update_quality(name: &str, sell_in: i32, quality: i32 ) -> Item{
    let item = Item::new(String::from(name), sell_in, quality);
    let items = vec![item];
    let mut rose = GildedRose::new(items);

    rose.update_quality();

    let result = rose.items.pop().unwrap();
    /*
        A csharp dev would be tempted to simply return `item`.
        This is illegal in rust as the value is now owned by the vector.
    */
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

#[test]
pub fn backstage_passes_increase_in_quality_twice_as_fast_within_10_days() {
    let result = update_quality("Backstage passes to a TAFKAL80ETC concert", 10, 30);

    assert_eq!(9, result.sell_in);
    assert_eq!(32, result.quality);
}

#[test]
pub fn backstage_passes_increase_in_quality_thrice_as_fast_within_5_days() {
    let result = update_quality("Backstage passes to a TAFKAL80ETC concert", 5, 30);

    assert_eq!(4, result.sell_in);
    assert_eq!(33, result.quality);
}

#[test]
pub fn backstage_passes_quality_drops_to_zero_after_the_concert() {
    let result = update_quality("Backstage passes to a TAFKAL80ETC concert", 0, 30);

    assert_eq!(-1, result.sell_in);
    assert_eq!(0, result.quality);
}

#[test]
pub fn conjured_item_degrades_twice_as_fast() {
    let result = update_quality("Conjured foo", 20, 30);

    assert_eq!(19, result.sell_in);
    assert_eq!(28, result.quality);
}

#[test]
pub fn conjured_item_degrades_twice_as_fast_after_sell_by() {
    let result = update_quality("Conjured foo", 0, 30);

    assert_eq!(-1, result.sell_in);
    assert_eq!(26, result.quality);
}