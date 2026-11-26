use alloc::{boxed::Box, string::ToString, vec::Vec};
use core::iter;

// XXX for zip..
extern crate std;
use std::io::{self, Read, Seek, SeekFrom};

use zip::ZipArchive;

use jvm::{Array, ArrayRawBuffer, ClassInstance, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        io::{File, InputStream},
        lang::String,
        util::{Enumeration, zip::ZipEntry},
    },
};

// class java.util.zip.ZipFile
#[java_class(name = "java/util/zip/ZipFile", public)]
pub struct ZipFile {
    #[field(private)]
    zipData: ClassInstanceRef<Array<i8>>,
}

#[java_methods]
impl ZipFile {
    pub(crate) async fn get_zip_archive(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<ZipArchive<ZipDataReader>> {
        let zip_data: ClassInstanceRef<Array<i8>> = jvm.get_field(this, "java/util/zip/ZipFile", "zipData", "[B").await?;
        let reader = ZipDataReader {
            length: jvm.array_length(&zip_data).await? as u64,
            buffer: jvm.array_raw_buffer(&zip_data).await?,
            position: 0,
        };

        match ZipArchive::new(reader) {
            Ok(x) => Ok(x),
            Err(err) => Err(jvm.exception("java/util/zip/ZipException", &err.to_string()).await),
        }
    }

    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.util.zip.ZipFile::<init>({this:?}, {file:?})");

        if file.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "file is null").await);
        }

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let length: i64 = jvm.invoke_virtual(&file, "java/io/File", "length", "()J", ()).await?;
        let buf = jvm.instantiate_array("B", length as _).await?;
        let is = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (file,)).await?;
        let read: Result<()> = async {
            let input = jvm
                .new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (is.clone(),))
                .await?;
            jvm.invoke_virtual(&input, "java/io/DataInputStream", "readFully", "([B)V", (buf.clone(),))
                .await
        }
        .await;
        let closed: Result<()> = jvm.invoke_virtual(&is, "java/io/InputStream", "close", "()V", ()).await;
        read.and(closed)?;

        jvm.put_field(&mut this, "java/util/zip/ZipFile", "zipData", "[B", buf).await?;

        // the constructor throws ZipException for a malformed archive
        let _ = Self::get_zip_archive(jvm, &this).await?;

        Ok(())
    }

    #[method(public)]
    async fn get_entry(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<ZipEntry>> {
        tracing::debug!("java.util.zip.ZipFile::getEntry({this:?}, {name:?})");

        let entry = jvm.new_class("java/util/zip/ZipEntry", "(Ljava/lang/String;)V", (name.clone(),)).await?;
        let name = JavaLangString::to_rust_string(jvm, &name).await?;

        let mut zip = Self::get_zip_archive(jvm, &this).await?;
        let file_size = zip.by_name(&name).map(|x| x.size());

        if let Ok(x) = file_size {
            let _: () = jvm
                .invoke_virtual(&entry, "java/util/zip/ZipEntry", "setSize", "(J)V", (x as i64,))
                .await?;

            Ok(entry.into())
        } else {
            Ok(None.into())
        }
    }

    #[method(public)]
    async fn entries(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Enumeration>> {
        tracing::debug!("java.util.zip.ZipFile::entries({this:?})");

        let zip = Self::get_zip_archive(jvm, &this).await?;
        let names = match zip.file_names().collect::<core::result::Result<Vec<_>, _>>() {
            Ok(names) => names,
            Err(err) => return Err(jvm.exception("java/util/zip/ZipException", &err.to_string()).await),
        };

        let mut name_array = jvm.instantiate_array("Ljava/lang/String;", names.len() as _).await?;
        for (i, name) in names.iter().enumerate() {
            let name = JavaLangString::from_rust_string(jvm, name).await?;
            jvm.store_array(&mut name_array, i as _, iter::once(name)).await?;
        }

        let entries = jvm
            .new_class(
                "java/util/zip/ZipFile$Entries",
                "(Ljava/util/zip/ZipFile;[Ljava/lang/String;)V",
                (this, name_array),
            )
            .await?;

        Ok(entries.into())
    }

    #[method(public)]
    async fn get_input_stream(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        entry: ClassInstanceRef<ZipEntry>,
    ) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("java.util.zip.ZipFile::getInputStream({this:?}, {entry:?})");

        if entry.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "entry is null").await);
        }

        let entry_name: Box<dyn ClassInstance> = jvm
            .invoke_virtual(&entry, "java/util/zip/ZipEntry", "getName", "()Ljava/lang/String;", ())
            .await?;
        let entry_name = JavaLangString::to_rust_string(jvm, &entry_name).await?;

        let data = {
            let mut zip = Self::get_zip_archive(jvm, &this).await?;
            let file = zip.by_name(&entry_name);
            let Ok(mut file) = file else {
                // getInputStream returns null when the entry is not in this zip
                return Ok(None.into());
            };

            let mut buf = Vec::new();
            file.read_to_end(&mut buf).map(|_| buf)
        };
        let Ok(data) = data else {
            return Err(jvm.exception("java/util/zip/ZipException", "invalid entry data").await);
        };

        // TODO do we have to use InflaterInputStream?
        let mut java_buf = jvm.instantiate_array("B", data.len() as _).await?;
        jvm.array_raw_buffer_mut(&mut java_buf).await?.write(0, &data)?;

        let input_stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (java_buf,)).await?;

        Ok(input_stream.into())
    }
}

// Reads the archive in place from the zipData array, so each lookup only touches the
// central directory and the requested entry instead of copying the whole archive.
pub(crate) struct ZipDataReader {
    buffer: Box<dyn ArrayRawBuffer>,
    length: u64,
    position: u64,
}

impl Read for ZipDataReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let count = buf.len().min(self.length.saturating_sub(self.position) as usize);
        // Corrupt archives can point past the end; the raw buffer slices at `position` even for empty reads.
        if count == 0 {
            return Ok(0);
        }
        self.buffer
            .read(self.position as usize, &mut buf[..count])
            .map_err(|_| io::Error::other("zip data read failed"))?;
        self.position += count as u64;

        Ok(count)
    }
}

impl Seek for ZipDataReader {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let position = match pos {
            SeekFrom::Start(offset) => Some(offset),
            SeekFrom::End(offset) => self.length.checked_add_signed(offset),
            SeekFrom::Current(offset) => self.position.checked_add_signed(offset),
        };
        self.position = position.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "seek before start of zip data"))?;

        Ok(self.position)
    }
}
