use std::{marker::PhantomData, path::PathBuf, sync::Arc};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tokio::{fs::{File, OpenOptions}, io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufStream}, sync::Mutex};

#[derive(Serialize, Deserialize)]
pub struct Entry<R> {
    pub timestamp: DateTime<Utc>,
    pub response: R,
}

impl<R> Entry<R> {
    pub fn new(response: R) -> Self {
        Entry {
            timestamp: Utc::now(),
            response,
        }
    }
}

#[async_trait]
pub trait Recorder {
    type Record: Serialize;

    async fn add(&self, entry: Entry<Self::Record>) -> anyhow::Result<()>;
}

#[async_trait]
pub trait Replay {
    type Record: DeserializeOwned;

    /// Reads next line from the replayer and deserializes it
    async fn read_next(&self) -> anyhow::Result<Option<Entry<Self::Record>>>;

    /// Reads and deserializes all entries
    async fn read_to_end(&self) -> anyhow::Result<Vec<Entry<Self::Record>>>;
}

#[derive(Debug, Clone)]
pub struct FileStore<R> {
    file_path: PathBuf,
    file: Arc<Mutex<BufStream<File>>>,
    response: PhantomData<R>,
}

impl<R: Serialize> FileStore<R> {
    pub async fn create(file_path: PathBuf) -> Result<Self, tokio::io::Error> {
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&file_path)
            .await?;

        let buf_stream = BufStream::new(file);

        Ok(Self {
            file_path,
            file: Arc::new(Mutex::new(buf_stream)),
            response: Default::default(),
        })
    }

    pub async fn create_with_config<C: Serialize>(
        file_path: PathBuf,
        config: &C,
    ) -> anyhow::Result<Self> {
        let file_store = Self::create(file_path).await?;

        // serialize and config and put it on first line
        let mut serialized = serde_json::to_vec(config)?;
        serialized.push(b'\n');

        {
            let mut file = file_store.file.lock().await;
            file.write_all(&serialized).await?;
        }
        Ok(file_store)
    }
}

/// TODO: Test deserialization / reading
impl<R: DeserializeOwned> FileStore<R> {
    pub async fn read_with_config<C: DeserializeOwned>(file_path: PathBuf) -> anyhow::Result<(C, Self)> {
        let file = OpenOptions::new()
            .create_new(false)
            .write(false)
            .read(true)
            .open(&file_path)
            .await?;

        let mut buf_stream = BufStream::new(file);

        let config = {
            // Enhancement: init with size of Config struct
            let mut config_json = String::new();
            let _line = buf_stream.read_line(&mut config_json).await?;

            serde_json::from_str::<C>(&config_json)?
        };

        Ok((
            config,
            Self {
                file_path,
                file: Arc::new(Mutex::new(buf_stream)),
                response: Default::default(),
            },
        ))
    }
}

#[async_trait]
impl<R: Serialize + Sync + Send> Recorder for FileStore<R> {
    type Record = R;

    async fn add(&self, entry: Entry<R>) -> anyhow::Result<()> {
        let mut serialized = serde_json::to_vec(&entry)?;

        // Add a new line between records
        serialized.push(b'\n');
        let mut file = self.file.lock().await;

        Ok(file.write_all(&serialized).await?)
    }
}

#[async_trait]
impl<R: DeserializeOwned + Sync + Send> Replay for FileStore<R> {
    type Record = R;

    /// Reads next line from the file
    async fn read_next(&self) -> anyhow::Result<Option<Entry<Self::Record>>> {
        let mut file = self.file.lock().await;
        let mut entry_line = String::new();
        let _line = file.read_line(&mut entry_line);

        // if no more lines, just return None.
        let entry = if entry_line.is_empty() {
            None
        } else {
            Some(serde_json::from_str::<Entry<_>>(&entry_line)?)
        };

        Ok(entry)
    }

    /// Reads and deserializes all entries
    async fn read_to_end(&self) -> anyhow::Result<Vec<Entry<Self::Record>>> {
        let mut file = self.file.lock().await;

        let content = {
            let mut buf_string = String::new();
            file.read_to_string(&mut buf_string).await?;

            buf_string
        };

        Ok(content.lines().map(|line| {
            serde_json::from_str::<Entry<Self::Record>>(line)
        }).collect::<Result<Vec<_>, _>>()?)
    }
}

pub mod run {
    use mav_sdk::grpc::telemetry::{AttitudeQuaternionResponse, PositionResponse};

    use crate::Entry;

    pub struct Logs<R> {
        pub rate: f64,
        pub records: Vec<Entry<R>>,
    }

    pub struct Run {
        pub position: Logs<PositionResponse>,
        pub attitude: Logs<AttitudeQuaternionResponse>,
    }
}

#[cfg(test)]
mod test {
    use std::ops::Sub;

    use chrono::{DateTime, Utc};

    // use once_cell::sync::Lazy;

    // pub static DIFFERENT_RATES_SET: Lazy<[DateTime<Utc>]>


    fn test_file_store_read_and_write_with_config() {
        todo!("Test the FileStore")
    }

    #[test]
    fn test_timestamp_hz_no_set_request() {
        let dt_first: DateTime<Utc> = "2021-10-24T14:30:30.071538262Z"
            .parse()
            .expect("Should parse DateTime");
        let dt_second: DateTime<Utc> = "2021-10-24T14:30:30.278002798Z"
            .parse()
            .expect("Should parse DateTime");

        // 50 hz
        // 20000000 nanos
        let expected_mills = chrono::Duration::milliseconds(20);

        // 206464536 nanos
        let duration = dt_second.sub(dt_first);

        assert_eq!(expected_mills, duration);
    }

    #[test]
    fn test_set_different_rate_position() {
        let dt_first: DateTime<Utc> = "2021-11-07T08:12:38.851345237Z"
            .parse()
            .expect("Should parse DateTime");
        let dt_second: DateTime<Utc> = "2021-11-07T08:12:38.857478607Z"
            .parse()
            .expect("Should parse DateTime");

        // 100 Hz
        // 10 milliseconds
        // 10000000 nanos
        let expected_mills = chrono::Duration::milliseconds(10);

        // 6133370 nanos
        let duration = dt_second.sub(dt_first);

        assert_eq!(expected_mills, duration);
    }
}
