// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const BASE_CAR_PRODUCTION_PER_HOUR: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let theoretical_rate_per_hour = BASE_CAR_PRODUCTION_PER_HOUR * speed as f64;
    return match speed {
        0..=4 => theoretical_rate_per_hour,
        5..=8 => theoretical_rate_per_hour * 0.9,
        9|10 => theoretical_rate_per_hour * 0.77, 
        _ => 0.0,
    };
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return production_rate_per_hour(speed).floor() as u32 / 60 as u32
}
