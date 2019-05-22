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
