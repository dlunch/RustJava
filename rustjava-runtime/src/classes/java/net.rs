use alloc::borrow::Cow;

use jvm::{Jvm, Result};
use percent_encoding::{AsciiSet, CONTROLS, percent_decode_str};

mod jar_url_connection;
mod malformed_url_exception;
mod unknown_service_exception;
mod url;
mod url_class_loader;
mod url_connection;
mod url_stream_handler;

pub use self::{
    jar_url_connection::JarURLConnection, malformed_url_exception::MalformedURLException, unknown_service_exception::UnknownServiceException,
    url::URL, url_class_loader::URLClassLoader, url_connection::URLConnection, url_stream_handler::URLStreamHandler,
};
pub(crate) const URL_PATH: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'%')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}')
    .add(b'!');

pub(crate) async fn decode_url_path<'a>(jvm: &Jvm, path: &'a str) -> Result<Cow<'a, str>> {
    match percent_decode_str(path).decode_utf8() {
        Ok(path) => Ok(path),
        Err(_) => Err(jvm.exception("java/lang/IllegalArgumentException", "Invalid UTF-8 in URL path").await),
    }
}
