use std::{
    fs,
    io::{self, Read, Write},
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

// TODO
pub struct FileImpl<R, W>
where
    R: Read + Sync + Send,
    W: Write + Sync + Send,
{
    read: R,
    write: W,
}

impl FileImpl<fs::File, fs::File> {
    pub fn from_path(path: &str) -> io::Result<Self> {
        let file = fs::File::open(path).unwrap();

        Ok(Self {
            read: file.try_clone().unwrap(),
            write: file,
        })
    }
}

impl<R> FileImpl<R, DummyWrite>
where
    R: Read + Sync + Send,
{
    pub fn from_read(read: R) -> Self {
        Self { read, write: DummyWrite }
    }
}

impl<W> FileImpl<DummyRead, W>
where
    W: Write + Sync + Send,
{
    pub fn from_write(write: W) -> Self {
        Self { read: DummyRead, write }
    }
}

#[async_trait::async_trait]
impl<R, W> File for FileImpl<R, W>
where
    R: Read + Sync + Send,
    W: Write + Sync + Send,
{
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, IOError> {
        Ok(self.read.read(buf).unwrap())
    }

    async fn write(&mut self, buf: &[u8]) -> Result<usize, IOError> {
        Ok(self.write.write(buf).unwrap())
    }

    async fn stat(&self) -> Result<FileStat, IOError> {
        todo!()
    }
}
