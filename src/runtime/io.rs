use alloc::sync::Arc;
use std::{
    fs::{self, OpenOptions},
    io::{self, Read, Seek, Write},
    sync::Mutex,
};

use java_runtime::{File, FileSize, FileStat, FileType, IOError, IOResult};

pub struct WriteStreamFile<W>
where
    W: Write + Send + Sync + 'static,
{
    write: Arc<Mutex<W>>,
}

impl<W> WriteStreamFile<W>
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
impl<W> File for WriteStreamFile<W>
where
    W: Write + Send + Sync + 'static,
{
    async fn read(&mut self, _buf: &mut [u8]) -> IOResult<usize> {
        Err(IOError::Unsupported)
    }

    async fn write(&mut self, buf: &[u8]) -> IOResult<usize> {
        let written = self.write.lock().unwrap().write(buf).unwrap();

        Ok(written)
    }

    async fn seek(&mut self, _pos: FileSize) -> IOResult<()> {
        Err(IOError::Unsupported)
    }

    async fn tell(&self) -> IOResult<FileSize> {
        Err(IOError::Unsupported)
    }

    async fn set_len(&mut self, _len: FileSize) -> IOResult<()> {
        Err(IOError::Unsupported)
    }

    async fn metadata(&self) -> IOResult<FileStat> {
        Err(IOError::Unsupported)
    }
}

impl<W> Clone for WriteStreamFile<W>
where
    W: Write + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self { write: self.write.clone() }
    }
}

pub struct InputStreamFile<R>
where
    R: Read + Send + Sync + 'static,
{
    read: Arc<Mutex<R>>,
}

impl<R> InputStreamFile<R>
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
impl<R> File for InputStreamFile<R>
where
    R: Read + Send + Sync + 'static,
{
    async fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        let read = self.read.lock().unwrap().read(buf).unwrap();

        Ok(read)
    }

    async fn write(&mut self, _buf: &[u8]) -> IOResult<usize> {
        Err(IOError::Unsupported)
    }

    async fn seek(&mut self, _pos: FileSize) -> IOResult<()> {
        Err(IOError::Unsupported)
    }

    async fn tell(&self) -> IOResult<FileSize> {
        Err(IOError::Unsupported)
    }

    async fn set_len(&mut self, _len: FileSize) -> IOResult<()> {
        Err(IOError::Unsupported)
    }

    async fn metadata(&self) -> IOResult<FileStat> {
        Err(IOError::Unsupported)
    }
}

impl<R> Clone for InputStreamFile<R>
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
    pub fn new(path: &str, write: bool) -> Self {
        let mut options = OpenOptions::new();
        let file = options.read(true).write(write).create(write).open(path).unwrap();

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

    async fn seek(&mut self, pos: FileSize) -> Result<(), IOError> {
        self.file.lock().unwrap().seek(io::SeekFrom::Start(pos)).unwrap();

        Ok(())
    }

    async fn tell(&self) -> Result<FileSize, IOError> {
        let pos = self.file.lock().unwrap().seek(io::SeekFrom::Current(0)).unwrap();

        Ok(pos as FileSize)
    }

    async fn set_len(&mut self, len: FileSize) -> Result<(), IOError> {
        self.file.lock().unwrap().set_len(len).unwrap();

        Ok(())
    }

    async fn metadata(&self) -> Result<FileStat, IOError> {
        let metadata = self.file.lock().unwrap().metadata().unwrap();
        let size = metadata.len();

        Ok(FileStat {
            size,
            r#type: FileType::File,
        })
    }
}
