enum_from_primitive! {
    pub enum ClientOpcode {
        PONG = 0x2001,
        MOVE_REQUEST = 0x3000,
        FIGHT_REQUEST = 0x6000,
        CREATE_ACCOUNT = 0x3306,
        CREATE_CHARACTER = 0x4000,
    }
}