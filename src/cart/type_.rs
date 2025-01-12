#[derive(Clone, Copy, PartialEq, Eq, Debug, FromPrimitive)]
pub enum CartType {
    RomOnly = 0x00,
    Mbc1 = 0x01,
    Mbc1_Ram = 0x02,
    Mbc1_Ram_Battery = 0x03,
    Mbc2 = 0x05,
    Mbc2_Battery = 0x06,
    Rom_Ram = 0x08,
    Rom_Ram_Battery = 0x09,
    Mmm01 = 0x0B,
    Mmm01_Ram = 0x0C,
    Mmm01_Ram_Battery = 0x0D,
    Mbc3_Timer_Battery = 0x0F,
    Mbc3_Timer_Ram_Battery = 0x10,
    Mbc3 = 0x11,
    Mbc3_Ram = 0x12,
    Mbc3_Ram_Battery = 0x13,
    Mbc5 = 0x19,
    Mbc5_Ram = 0x1A,
    Mbc5_Ram_Battery = 0x1B,
    Mbc5_Rumble = 0x1C,
    Mbc5_Rumble_Ram = 0x1D,
    Mbc5_Rumble_Ram_Battery = 0x1E,
    Mbc6 = 0x20,
    Mbc7_Sensor_Rumble_Ram_Battery = 0x22,
    Pocket_Camera = 0xFC,
    Bandai_Tama5 = 0xFD,
    Hu3 = 0xFE,
    HuC1_Ram_Battery = 0xFF,
}
