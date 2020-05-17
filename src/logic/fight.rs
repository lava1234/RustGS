use crate::models;
use byte::*;
use crate::utils;
use rand::Rng;

pub fn parse_fighters(buffer_data: &[u8]){
    let mut p1 = buffer_to_player(buffer_data);

    let mut p2 = buffer_to_player(&buffer_data[26..]);
    println!("{:?}", p1);
    println!("{:?}", p2);
    
    create_fight(&mut p1, &mut p2);

    
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
        next_atk_time: 0
    };

}

fn create_fight(p1: &mut models::player::Player, p2: &mut models::player::Player){
    let mut min_atk_time = std::cmp::min(p1.atk_time, p2.atk_time);

    p1.next_atk_time = p1.atk_time;
    p2.next_atk_time = p2.atk_time;
    let mut rng = rand::thread_rng();
    let max_bout = 60;
    let mut flag = false;
    for i in 0..max_bout {
        if p1.next_atk_time == p2.next_atk_time {
            if rng.gen_range(0, 101) > rng.gen_range(0, 101) 
            {
                flag = attack(p1, p2,min_atk_time);
            } else {
                flag = attack(p2, p1, min_atk_time);
            }
        } else if min_atk_time == p1.next_atk_time{
            flag = attack(p1, p2, min_atk_time);
        } else if min_atk_time == p2.next_atk_time {
            flag = attack(p2, p1, min_atk_time);
        }

        min_atk_time = std::cmp::min(p1.next_atk_time, p2.next_atk_time);

        if !flag {
            println!("fight end");
            break;
        }
    }

    println!("{:?}", p1);
    println!("{:?}", p2);
}

fn attack(p1: &mut models::player::Player, p2: &mut models::player::Player, min_atk_time: i16) -> bool{

    p2.hp -= std::cmp::min(i32::from(p1.min_atk), i32::from(p1.max_atk)+1);
    if p2.hp < 0{
        return false;
    }

    p1.next_atk_time += p1.atk_time;

    return true;
}
