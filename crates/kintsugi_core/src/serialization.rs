use anyhow::Result;
use std::io::{Read, Write};

pub trait Serialize: Sized {
    fn serialize<W: Write>(&self, w: &mut W);
    fn deserialize<R: Read>(r: &mut R) -> Result<Self>;
}

impl Serialize for u8 {
    fn serialize<W: Write>(&self, w: &mut W) {
        w.write_all(&[*self]).unwrap();
    }

    fn deserialize<R: Read>(r: &mut R) -> Result<Self> {
        let mut buf = [0; 1];
        r.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

impl Serialize for u16 {
    fn serialize<W: Write>(&self, w: &mut W) {
        let a = (*self >> 8) as u8;
        let b = (*self) as u8;
        w.write_all(&[a, b]).unwrap();
    }

    fn deserialize<R: Read>(r: &mut R) -> Result<Self> {
        let mut buf = [0; 2];
        r.read_exact(&mut buf)?;
        let a = buf[0] as u16;
        let b = buf[1] as u16;
        Ok((a << 8) | b)
    }
}

impl Serialize for u32 {
    fn serialize<W: Write>(&self, w: &mut W) {
        let a = (*self >> 24) as u8;
        let b = (*self >> 16) as u8;
        let c = (*self >> 8) as u8;
        let d = (*self) as u8;
        w.write_all(&[a, b, c, d]).unwrap();
    }

    fn deserialize<R: Read>(r: &mut R) -> Result<Self> {
        let mut buf = [0; 4];
        r.read_exact(&mut buf)?;
        let a = buf[0] as u32;
        let b = buf[1] as u32;
        let c = buf[2] as u32;
        let d = buf[3] as u32;
        Ok((a << 24) | (b << 16) | (c << 8) | d)
    }
}
