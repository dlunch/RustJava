use alloc::sync::Arc;
use std::{
    fs,
    io::{self, Read, Seek, Write},
    sync::Mutex,
};

use java_runtime::{File, IOError};

pub struct WriteOnlyFile<W>
where
    W: Write + Send + Sync + 'static,
{
    write: Arc<Mutex<W>>,
}

impl<W> WriteOnlyFile<W>
where
    W: Write + Send + Sync + 'static,
{
    pub fn new(write: W) -> Self {
        Self {
            write: Arc::new(Mutex::new(write)),
        }
    }
}

#[async_trait::async_trait]
impl<W> File for WriteOnlyFile<W>
where
    W: Write + Send + Sync + 'static,
{
    async fn read(&mut self, _buf: &mut [u8]) -> Result<usize, IOError> {
        Err(IOError::Unsupported)
    }

    async fn write(&mut self, buf: &[u8]) -> Result<usize, IOError> {
        let written = self.write.lock().unwrap().write(buf).unwrap();

        Ok(written)
    }

    async fn seek(&mut self, _pos: u64) -> Result<(), IOError> {
        Err(IOError::Unsupported)
    }
}

impl<W> Clone for WriteOnlyFile<W>
where
    W: Write + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self { write: self.write.clone() }
    }
}

pub struct ReadOnlyFile<R>
where
    R: Read + Send + Sync + 'static,
{
    read: Arc<Mutex<R>>,
}

impl<R> ReadOnlyFile<R>
where
    R: Read + Send + Sync + 'static,
{
    pub fn new(read: R) -> Self {
        Self {
            read: Arc::new(Mutex::new(read)),
        }
    }
}

#[async_trait::async_trait]
impl<R> File for ReadOnlyFile<R>
where
    R: Read + Send + Sync + 'static,
{
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, IOError> {
        let read = self.read.lock().unwrap().read(buf).unwrap();

        Ok(read)
    }

    async fn write(&mut self, _buf: &[u8]) -> Result<usize, IOError> {
        Err(IOError::Unsupported)
    }

    async fn seek(&mut self, _pos: u64) -> Result<(), IOError> {
        Err(IOError::Unsupported)
    }
}

impl<R> Clone for ReadOnlyFile<R>
where
    R: Read + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self { read: self.read.clone() }
    }
}

#[derive(Clone)]
pub struct FileImpl {
    file: Arc<Mutex<fs::File>>,
}

impl FileImpl {
    pub fn new(path: &str) -> Self {
        let file = fs::File::open(path).unwrap();

        Self {
            file: Arc::new(Mutex::new(file)),
        }
    }
}

#[async_trait::async_trait]
impl File for FileImpl {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, IOError> {
        let read = self.file.lock().unwrap().read(buf).unwrap();

        Ok(read)
    }

    async fn write(&mut self, buf: &[u8]) -> Result<usize, IOError> {
        let write = self.file.lock().unwrap().write(buf).unwrap();

        Ok(write)
    }

    async fn seek(&mut self, pos: u64) -> Result<(), IOError> {
        self.file.lock().unwrap().seek(io::SeekFrom::Start(pos)).unwrap();

        Ok(())
    }
}
