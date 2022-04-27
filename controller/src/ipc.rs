use anyhow::Result;
use tokio::io::{AsyncRead, AsyncReadExt, Take};

pub async fn read<T: AsyncRead + Unpin>(mut input: T) -> Result<Take<T>> {
	let length = input.read_i32_le().await?;
	Ok(input.take(length as u64))
}

pub async fn read_msgpack<'a, T, R>(input: R) -> Result<T>
where
	R: AsyncRead + Unpin,
	T: serde::de::DeserializeOwned,
{
	let mut buf = Vec::new();
	read(input).await?.read_to_end(&mut buf).await?;
	Ok(rmp_serde::from_slice::<T>(&buf)?)
}
