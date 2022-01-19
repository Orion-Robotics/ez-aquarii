use anyhow::Result;
use bytes::Bytes;
use tokio::io::{AsyncRead, AsyncReadExt, Take};

pub async fn read<T: AsyncRead + Unpin>(mut input: T) -> Result<Take<T>> {
	let length = input.read_i32_le().await?;
	Ok(input.take(length as u64))
}

pub async fn read_proto<T, R>(input: R) -> Result<T>
where
	T: prost::Message + Default,
	R: AsyncRead + Unpin,
{
	let mut buf = Vec::new();
	read(input).await?.read_to_end(&mut buf).await?;
	Ok(T::decode(Bytes::from(buf))?)
}
