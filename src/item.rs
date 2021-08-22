pub enum Items {
	Sword,
}

pub enum Armors {
	Helmet,
	Chesplate,
	Leggings,
	Boots,
}

pub struct Item{
	durability: u32,
	stack_size: u32,
}

pub struct Sword {
	item: Item,
	attack_speed: i32, // delay between attacks (in millis)
}

pub struct ArmorPiece {
	item: Item,
	protection: i32,
}
