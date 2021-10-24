use std::{marker::PhantomData, path::PathBuf, sync::Arc};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::{
    fs::{File, OpenOptions},
    io::AsyncWriteExt,
    sync::Mutex,
};

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

#[derive(Debug, Clone)]
pub struct FileStore<R> {
    file_path: PathBuf,
    file: Arc<Mutex<File>>,
    response: PhantomData<R>,
}

impl<R: Serialize> FileStore<R> {
    pub async fn create(file_path: PathBuf) -> Result<Self, tokio::io::Error> {
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&file_path)
            .await?;

        Ok(Self {
            file_path,
            file: Arc::new(Mutex::new(file)),
            response: Default::default(),
        })
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

// rate: f64,

#[cfg(test)]
mod test {
    use std::ops::Sub;

    use chrono::{DateTime, Utc};

    #[test]
    fn test_timestamp_hz() {
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
}
