use super::*;
use std::{
    io::{Read, Write, copy, empty},
    net::TcpStream,
    ops::Deref,
};

static BUF: usize = 64 * 1024;

pub struct Request {
    buf: Box<[u8; BUF]>,
    at: usize,
    used: usize,
    stream: TcpStream,
}

impl Request {
    pub fn new(mut stream: TcpStream) -> Result<Self> {
        stream.write_all("HTTP/1.0 200 OK\r\n".as_bytes())?;
        Ok(Self {
            buf: Box::new([0; BUF]),
            at: 0,
            used: 0,
            stream,
        })
    }

    pub fn receive_multipart<W: Write>(&mut self, write_to: W) -> Result<Option<W>> {
        Ok(if self.collect_until(b" ")? == b"POST" {
            self.skip_until(b"Content-Type: multipart/form-data; boundary=")?;
            let boundary = self.receive_until(b"\r\n", Vec::from(b"\r\n--"))?;
            self.skip_until(b"\r\n\r\n")?;
            // Skip to the body within the multipart body
            self.skip_until(b"\r\n\r\n")?;
            Some(self.receive_until(&boundary, write_to)?)
        } else {
            None
        })
    }

    pub fn send_header(&mut self, write_from: impl Deref<Target = str>) -> Result<()> {
        self.stream.write_all(write_from.as_bytes())?;
        self.stream.write_all(b"\r\n")?;
        Ok(())
    }

    pub fn send_body(self, write_from: impl Deref<Target = str>) -> Result<()> {
        self.send_body_from(write_from.as_bytes())
    }

    pub fn send_body_from(mut self, mut write_from: impl Read) -> Result<()> {
        self.stream.write_all(b"\r\n")?;
        copy(&mut write_from, &mut self.stream)?;
        Ok(())
    }

    fn skip_until(&mut self, target: &[u8]) -> Result<()> {
        self.receive_until(target, empty())?;
        Ok(())
    }

    fn collect_until(&mut self, target: &[u8]) -> Result<Vec<u8>> {
        self.receive_until(target, Vec::new())
    }

    fn receive_until<W: Write>(&mut self, target: &[u8], mut write_to: W) -> Result<W> {
        let mut offset = 0;
        while let Some(b) = self.next()? {
            if b.eq_ignore_ascii_case(&target[offset]) {
                offset += 1;
                if offset == target.len() {
                    break;
                }
                continue;
            } else if offset != 0 {
                write_to.write_all(&target[0..offset])?;
                offset = 0;
            }
            write_to.write_all(&[b])?;
        }
        write_to.flush()?;
        Ok(write_to)
    }

    fn next(&mut self) -> Result<Option<u8>> {
        Ok(if self.at == self.used {
            self.at = 0;
            self.used = self.stream.read(self.buf.as_mut_slice())?;
            if self.used == 0 {
                None
            } else {
                Some(self.consume())
            }
        } else {
            Some(self.consume())
        })
    }

    fn consume(&mut self) -> u8 {
        let at = self.at;
        self.at += 1;
        self.buf[at]
    }
}
