use i2c;

//const FT6X06_ADDRESS: i2c::Address = i2c::Address::bits_7(0b0111000);
const FT6X06_ADDRESS: i2c::Address = i2c::Address::bits_7(0b0101010);

pub fn init_ft6x06(i2c_4: &mut i2c::I2C) -> Result<(), i2c::Error> {
    // read and check device family ID
    let mut ok = false;
    for _ in 0..3 {
        if i2c_4.read(FT6X06_ADDRESS, 0xA8)? == 0x11 {
            ok = true;
            break;
        }
    }
    assert!(ok, "not the expected hardware id");
    // set polling mode
    i2c_4.write(FT6X06_ADDRESS, 0xA4, 0)?;
    Ok(())
}
