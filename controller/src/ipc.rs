use std::io::{Read, Take};

use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};
use bytes::Bytes;

pub fn read<T: Read>(mut input: T) -> Result<Take<T>> {
    let length = input.read_i32::<LittleEndian>()?;
    Ok(input.take(length as u64))
}

pub fn read_proto<T, R>(input: R) -> Result<T>
where
    T: prost::Message + Default,
    R: Read,
{
    let mut buf = Vec::new();
    read(input)?.read_to_end(&mut buf)?;
    Ok(T::decode(Bytes::from(buf))?)
}
