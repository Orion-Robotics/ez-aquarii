use std::{io::Write, sync::Arc};

use byteorder::WriteBytesExt;
use parking_lot::Mutex;
use serde::Serialize;
use tokio::sync::mpsc::{self, error::SendError, UnboundedSender};

// Recorder is a struct that manages the recording of state history.
pub struct Recorder<T: Write + Send + Sync + 'static, S: Serialize + Send + Sync + 'static> {
	pub output: Arc<Mutex<Option<T>>>,
	pub send: UnboundedSender<S>,
}

impl<T: Write + Send + Sync + 'static, S: Serialize + Send + Sync + 'static> Recorder<T, S> {
	pub fn new(output: Arc<Mutex<Option<T>>>) -> Self {
		let (s, mut r) = mpsc::unbounded_channel::<S>();

		{
			let output = output.clone();
			std::thread::spawn(move || {
				while let Some(value) = r.blocking_recv() {
					if let Some(output) = output.lock().as_mut() {
						let _ = serde_json::to_writer(output.by_ref(), &value);
						let _ = output.write_u8(b'\n');
					}
				}
			});
		}

		Recorder { output, send: s }
	}

	pub fn record(&self, value: S) -> Result<(), SendError<S>> {
		self.send.send(value)
	}
}
