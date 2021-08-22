// pub struct Item{

// }

pub enum Item {
    Sword(sword),
}

pub struct sword {
    attack_speed: i32, // delay between attacks (in millis)
}
