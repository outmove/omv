#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::string::String;

#[cfg(feature = "std")]
pub use std::io::Error;

#[cfg(not(feature = "std"))]
#[derive(Clone, Debug)]
pub struct Error(pub String);

#[cfg(not(feature = "std"))]
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		core::write!(f, "{:?}", self)
	}
}

#[cfg(not(feature = "std"))]
impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Error {
        Error(String::from(s))
    }
}

#[cfg(feature = "std")]
pub fn other_error(s: &str) -> Error {
    Error::new(std::io::ErrorKind::Other, s)
}

#[cfg(not(feature = "std"))]
pub fn other_error(s: &str) -> Error {
    Error(s.into())
}

#[cfg(feature = "std")]
pub use std::io::Write;

#[cfg(not(feature = "std"))]
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;
    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), Error> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => {
                    return Err("failed to write whole buffer".into());
                }
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

#[cfg(not(feature = "std"))]
impl Write for &mut [u8] {
    #[inline]
    fn write(&mut self, data: &[u8]) -> Result<usize, Error> {
        let amt = core::cmp::min(data.len(), self.len());
        let (a, b) = core::mem::replace(self, &mut []).split_at_mut(amt);
        a.copy_from_slice(&data[..amt]);
        *self = b;
        Ok(amt)
    }

    #[inline]
    fn write_all(&mut self, data: &[u8]) -> Result<(), Error> {
        if self.write(data)? == data.len() {
            Ok(())
        } else {
            Err("failed to write whole buffer".into())
        }
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(not(feature = "std"))]
impl Write for alloc::vec::Vec<u8> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error> {
        self.extend_from_slice(buf);
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(feature = "std")]
pub use std::io::Read;

#[cfg(not(feature = "std"))]
pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;

    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<(), Error> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(n) => {
                    let tmp = buf;
                    buf = &mut tmp[n..];
                }
                Err(e) => return Err(e),
            }
        }
        if !buf.is_empty() {
            Err("failed to fill whole buffer".into())
        } else {
            Ok(())
        }
    }
}

#[cfg(not(feature = "std"))]
impl Read for &[u8] {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let amt = core::cmp::min(buf.len(), self.len());
        let (a, b) = self.split_at(amt);

        // First check if the amount of bytes we want to read is small:
        // `copy_from_slice` will generally expand to a call to `memcpy`, and
        // for a single byte the overhead is significant.
        if amt == 1 {
            buf[0] = a[0];
        } else {
            buf[..amt].copy_from_slice(a);
        }

        *self = b;
        Ok(amt)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        if buf.len() > self.len() {
            return Err("failed to fill whole buffer".into());
        }
        let (a, b) = self.split_at(buf.len());

        // First check if the amount of bytes we want to read is small:
        // `copy_from_slice` will generally expand to a call to `memcpy`, and
        // for a single byte the overhead is significant.
        if buf.len() == 1 {
            buf[0] = a[0];
        } else {
            buf.copy_from_slice(a);
        }

        *self = b;
        Ok(())
    }
}

#[cfg(feature = "std")]
pub use std::io::Cursor;

#[cfg(not(feature = "std"))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Cursor<T> {
    inner: T,
    pos: u64,
}

#[cfg(not(feature = "std"))]
impl<T> Cursor<T>
where
    T: AsRef<[u8]>
{
    pub fn new(inner: T) -> Cursor<T> {
        Cursor { pos: 0, inner }
    }

    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    pub fn position(&self) -> u64 {
        self.pos
    }

    fn fill_buf(&mut self) -> Result<&[u8], Error> {
        let amt = core::cmp::min(self.pos, self.inner.as_ref().len() as u64);
        Ok(&self.inner.as_ref()[(amt as usize)..])
    }
}

#[cfg(not(feature = "std"))]
impl<T> Read for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let n = Read::read(&mut self.fill_buf()?, buf)?;
        self.pos += n as u64;
        Ok(n)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        let n = buf.len();
        Read::read_exact(&mut self.fill_buf()?, buf)?;
        self.pos += n as u64;
        Ok(())
    }
}