use super::{Item, GildedRose};

#[test]
pub fn standard_item() {
    let sell_in = 10;
    let quality = 20;
    let name = "foo";
    let item = Item::new(String::from(name), sell_in, quality);
    let items = vec![item];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    let result = rose.items.pop().unwrap();

    assert_eq!("foo", result.name);
    assert_eq!(9, result.sell_in);
    assert_eq!(19, result.quality);
}
