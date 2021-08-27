use std;

use crate::id;
use rand::Rng;

const INVENTORY_MAX_LENGHT: usize = 10;

pub enum ObjectDrop {
    Ok(Weapon),
    None,
}

pub struct WeaponInventory {
    pub weapon_list: [Weapon; INVENTORY_MAX_LENGHT],
    pub selected_index: usize,
}

#[derive(Clone, Copy)]
pub enum Weapon {
    Pistol(Pistol),
    None,
}

#[derive(Clone, Copy)]
pub struct Pistol {
    pub id: i32,
    pub damage: i32,
    pub attack_speed: i32, // delay between two shots
    pub last_shot_time: std::time::SystemTime,
    pub range: f32,
}

pub fn generate_drop(id_manager: &mut id::IdManager) -> ObjectDrop {
    let prcentage = rand::thread_rng().gen_range(0..100);

    let unique_weapon = 1; // the number of weapons

    if prcentage < 90 {
        if unique_weapon > 1 {
            let index = rand::thread_rng().gen_range(0..unique_weapon);
            match index {
                1 => ObjectDrop::Ok(Weapon::Pistol(Pistol::new(id_manager))),
                _ => ObjectDrop::None,
            }
        } else {
            ObjectDrop::Ok(Weapon::Pistol(Pistol::new(id_manager)))
        }
    } else {
        ObjectDrop::None
    }
}

impl WeaponInventory {
    pub fn new(id_manager: &mut id::IdManager) -> Self {
        let mut weapon_list = [Weapon::None; INVENTORY_MAX_LENGHT];
        weapon_list[0] = Weapon::Pistol(Pistol::new(id_manager));
        WeaponInventory {
            weapon_list: weapon_list,
            selected_index: 0,
        }
    }
    pub fn index_is_weapon(&self) -> bool {
        match self.weapon_list[self.selected_index] {
            Weapon::None => false,
            _ => true,
        }
    }
}

impl Pistol {
    pub fn new(id_manager: &mut id::IdManager) -> Self {
        Pistol {
            id: id_manager.get_new_id(),
            damage: 6,
            attack_speed: 200,
            last_shot_time: std::time::SystemTime::UNIX_EPOCH,
            range: 1000.,
        }
    }
    pub fn can_shoot(&mut self) -> bool {
        match self.last_shot_time.elapsed() {
            Ok(elapsed) => {
                if elapsed.as_millis() > self.attack_speed as u128 {
                    self.last_shot_time = std::time::SystemTime::now();
                    // println!("elapsed: {}", elapsed.as_millis());
                    true
                } else {
                    // println!("Can't shoot yet");
                    false
                }
            }
            Err(e) => {
                eprintln!(
                    "There has been an error with the system clock, err: {:?}",
                    e
                );
                false
            }
        }
    }
}
