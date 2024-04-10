use alloc::sync::Arc;
use std::{
    fs,
    io::{self, Read, Write},
    sync::Mutex,
};

use java_runtime::{File, FileStat, IOError};

pub struct DummyRead;
impl Read for DummyRead {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        panic!("Tried to read from unreadable file")
    }
}

pub struct DummyWrite;
impl Write for DummyWrite {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        panic!("Tried to write to unwritable file")
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct FileImpl<R, W>
where
    R: Read + Sync + Send,
    W: Write + Sync + Send,
{
    read: Arc<Mutex<R>>,
    write: Arc<Mutex<W>>,
}

impl FileImpl<fs::File, fs::File> {
    pub fn from_path(path: &str) -> io::Result<Self> {
        let file = fs::File::open(path).unwrap();

        Ok(Self {
            read: Arc::new(Mutex::new(file.try_clone().unwrap())),
            write: Arc::new(Mutex::new(file)),
        })
    }
}

impl<R> FileImpl<R, DummyWrite>
where
    R: Read + Sync + Send,
{
    pub fn from_read(read: R) -> Self {
        Self {
            read: Arc::new(Mutex::new(read)),
            write: Arc::new(Mutex::new(DummyWrite)),
        }
    }
}

impl<W> FileImpl<DummyRead, W>
where
    W: Write + Sync + Send,
{
    pub fn from_write(write: W) -> Self {
        Self {
            read: Arc::new(Mutex::new(DummyRead)),
            write: Arc::new(Mutex::new(write)),
        }
    }
}

#[async_trait::async_trait]
impl<R, W> File for FileImpl<R, W>
where
    R: Read + Sync + Send,
    W: Write + Sync + Send,
{
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, IOError> {
        Ok(self.read.lock().unwrap().read(buf).unwrap())
    }

    async fn write(&mut self, buf: &[u8]) -> Result<usize, IOError> {
        Ok(self.write.lock().unwrap().write(buf).unwrap())
    }

    async fn stat(&self) -> Result<FileStat, IOError> {
        todo!()
    }
}

impl<R, W> Clone for FileImpl<R, W>
where
    R: Read + Sync + Send,
    W: Write + Sync + Send,
{
    fn clone(&self) -> Self {
        Self {
            read: self.read.clone(),
            write: self.write.clone(),
        }
    }
}
