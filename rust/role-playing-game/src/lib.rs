// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            return Some(Player {
                health: 100,
                mana: if self.level >= 10 { Some(100) } else { None },
                level: self.level
            })
        } else {
            return None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                let temp_hp = self.health as i32 - mana_cost as i32;
                self.health = std::cmp::max(temp_hp, 0_i32) as u32;
                return 0;
            }
            Some (u32)=> {
                if self.mana.unwrap() >= mana_cost {
                    let temp_mana = self.mana.unwrap() - mana_cost;
                    self.mana = if temp_mana <= 0 { None } else { Some(self.mana.unwrap() - mana_cost) };
                    return mana_cost * 2;
                } else {
                    return 0;
                }
            }
        }

    }
}
