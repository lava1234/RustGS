use crate::models;
use byte::*;
use crate::utils;
pub fn parse_fighters(buffer_data: &[u8]){
    let p1 = buffer_to_player(buffer_data);

    let p2 = buffer_to_player(&buffer_data[26..]);

    println!("{:?}", p1);
    println!("{:?}", p2);
}

fn buffer_to_player(buffer_data: &[u8]) -> models::player::Player{
    return models::player::Player {
        hp: buffer_data[..4].read_with::<i32>(&mut 0, LE).unwrap(),
        max_hp: buffer_data[4..8].read_with::<i32>(&mut 0, LE).unwrap(),
        mp: buffer_data[8..12].read_with::<i32>(&mut 0, LE).unwrap(),
        max_mp: buffer_data[12..16].read_with::<i32>(&mut 0, LE).unwrap(),
        min_atk: buffer_data[16..18].read_with::<i16>(&mut 0, LE).unwrap(),
        max_atk: buffer_data[18..20].read_with::<i16>(&mut 0, LE).unwrap(),
        def: buffer_data[20..24].read_with::<i32>(&mut 0, LE).unwrap(),
        atk_time: buffer_data[24..26].read_with::<i16>(&mut 0, LE).unwrap(),
    };

}

pub fn create_fight(p1: models::player::Player, p2: models::player::Player){
    
}
