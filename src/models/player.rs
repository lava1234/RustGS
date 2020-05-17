#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub hp: i32,
    pub max_hp: i32,
    pub mp: i32,
    pub max_mp: i32,
    pub min_atk: i16,
    pub max_atk: i16,
    pub def: i32,
    pub atk_time: i16,
    pub next_atk_time: i16
}