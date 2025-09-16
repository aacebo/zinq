#![allow(unused)]

use std::{borrow::Cow, fmt};

use crate::parse::Position;

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    ln: usize,
    left: usize,
    right: usize,
    data: Cow<'a, str>,
}

impl<'a> Scanner<'a> {
    pub fn ln(&self) -> usize {
        return self.ln;
    }

    pub fn left(&self) -> usize {
        return self.left;
    }

    pub fn right(&self) -> usize {
        return self.right;
    }

    pub fn position(&self) -> Position {
        return Position {
            ln: self.ln,
            start: self.left,
            end: self.right,
        };
    }

    pub fn count(&self) -> usize {
        return (self.right - self.left) + 1;
    }

    /// if the scanner is EOF you
    /// have reached the end of the dataset
    pub fn is_eof(&self) -> bool {
        return self.left > self.data.len() - 1 || self.right > self.data.len() - 1;
    }

    /// if the scanner is SOF you
    /// have not moved from the start of the dataset
    pub fn is_sof(&self) -> bool {
        return self.right < 1;
    }

    /// get the length of the inner dataset
    pub fn len(&self) -> usize {
        return self.data.len();
    }

    /// get the current byte
    pub fn curr(&self) -> u8 {
        return match self.data.as_bytes().get(self.right) {
            None => 0,
            Some(v) => *v,
        };
    }

    /// peek at the next byte
    /// without moving forward
    pub fn peek(&self) -> Option<u8> {
        return match self.data.as_bytes().get(self.right + 1) {
            None => None,
            Some(v) => Some(*v),
        };
    }

    /// peek at the Nth next byte
    /// without moving forward
    pub fn peek_offset(&self, n: isize) -> Option<u8> {
        return match self
            .data
            .as_bytes()
            .get(((self.right as isize) + n) as usize)
        {
            None => None,
            Some(v) => Some(*v),
        };
    }

    /// peek at the next N bytes without
    /// moving forward
    pub fn peek_n(&self, n: usize) -> Option<&str> {
        if self.right + n > self.data.len() {
            return None;
        }

        return Some(&self.data[self.right..self.right + n + 1]);
    }

    /// move forward until you find byte
    pub fn seek(&mut self, byte: u8) -> bool {
        while !self.is_eof() && self.curr() != byte {
            self.next();
        }

        return self.curr() == byte;
    }

    /// move backward
    pub fn prev(&mut self) -> Option<u8> {
        self.right -= 1;

        if self.right < 1 {
            return None;
        }

        return match self.data.as_bytes().get(self.right) {
            None => None,
            Some(v) => Some(*v),
        };
    }

    /// move forward
    pub fn next(&mut self) -> Option<u8> {
        self.right += 1;

        if self.is_eof() {
            return None;
        }

        if self.curr() == b'\n' {
            self.ln += 1;
        }

        return match self.data.as_bytes().get(self.right) {
            None => None,
            Some(v) => Some(*v),
        };
    }

    /// move forward n times
    pub fn next_n(&mut self, n: usize) -> Option<u8> {
        for _ in 0..n {
            self.right += 1;

            if self.is_eof() {
                return None;
            }
        }

        return match self.data.as_bytes().get(self.right) {
            None => None,
            Some(v) => Some(*v),
        };
    }

    /// move forward if a given byte
    /// matches the current byte
    pub fn next_if(&mut self, byte: u8) -> bool {
        if self.is_eof() {
            return false;
        }

        return match self.peek() {
            None => false,
            Some(v) => {
                if v == byte {
                    self.next();
                }

                v == byte
            }
        };
    }

    /// move forward if the next N bytes match
    /// a given sequence of bytes
    pub fn next_if_bytes(&mut self, bytes: &[u8]) -> bool {
        if self.is_eof() {
            return false;
        }

        if let Some(v) = self.peek_n(bytes.len() - 1)
            && v.as_bytes() == bytes
        {
            self.skip(bytes.len() - 1);
            return true;
        }

        return false;
    }

    /// move forward if the current byte
    /// is not equal to a given byte
    pub fn next_if_not(&mut self, byte: u8) -> bool {
        if self.is_eof() {
            return false;
        }

        return match self.peek() {
            None => false,
            Some(v) => {
                if v != byte {
                    self.next();
                }

                v != byte
            }
        };
    }

    /// move forward until you reach a
    /// given byte
    pub fn next_until(&mut self, byte: u8) -> &mut Self {
        if self.is_eof() {
            return self;
        }

        while self.curr() != byte && !self.is_eof() {
            self.next();
        }

        if !self.is_eof() {
            self.prev();
        }

        return self;
    }

    /// move forward until you find a given
    /// sequence of bytes
    pub fn next_until_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        if self.is_eof() {
            return self;
        }

        let mut iter = self.clone();
        iter.goto(self.right);

        while !iter.is_eof() {
            if iter.curr() == bytes[0] {
                if let Some(v) = iter.peek_n(bytes.len() - 1)
                    && v.as_bytes() == bytes
                {
                    self.right = iter.left - 1;
                    break;
                }
            }

            iter.fshift(1);
        }

        return self;
    }

    /// move forward while the current byte
    /// equals byte
    pub fn next_while(&mut self, byte: u8) -> &mut Self {
        if self.is_eof() {
            return self;
        }

        while self.curr() == byte && !self.is_eof() {
            self.next();
        }

        return self;
    }

    /// move forward until end of input
    pub fn next_until_end(&mut self) -> &mut Self {
        self.right = self.data.len() - 1;
        return self;
    }

    /// reset forward pointer to backward pointer location
    pub fn reset(&mut self) -> &mut Self {
        self.right = self.left;
        return self;
    }

    /// go to a specific index for both forward and backward pointers
    pub fn goto(&mut self, n: usize) -> &mut Self {
        self.left = n;
        self.right = n;
        return self;
    }

    /// shift backwards by n
    pub fn bshift(&mut self, n: usize) -> &mut Self {
        self.left -= n;
        self.right -= n;
        return self;
    }

    /// shift forwards by n
    pub fn fshift(&mut self, n: usize) -> &mut Self {
        self.left += n;
        self.right += n;
        return self;
    }

    /// move forward pointer by n
    pub fn skip(&mut self, n: usize) -> &mut Self {
        self.right += n;
        return self;
    }

    /// move backward pointer to forward pointer
    /// location and return bytes
    pub fn commit(&mut self) -> &str {
        let mut end = self.right + 1;

        if end > self.data.len() {
            end = self.data.len();
        }

        let bytes = &self.data[self.left..end];
        self.left = self.right + 1;
        self.right += 1;
        return bytes;
    }

    /// get the current byte window as a &str
    pub fn as_str(&self) -> &str {
        let mut end = self.right + 1;

        if end > self.data.len() {
            end = self.data.len()
        }

        return &self.data[self.left..end];
    }

    /// get the current by window
    pub fn as_bytes(&self) -> &[u8] {
        let mut end = self.right + 1;

        if end > self.data.len() {
            end = self.data.len()
        }

        return &self.data[self.left..end].as_bytes();
    }

    /// get the entire inner dataset
    pub fn into_inner(&self) -> &str {
        return &self.data;
    }
}

impl<'a> From<&'a [u8]> for Scanner<'a> {
    fn from(value: &'a [u8]) -> Self {
        return Self {
            ln: 0,
            left: 0,
            right: 0,
            data: String::from_utf8_lossy(value),
        };
    }
}

impl<'a> From<&'a str> for Scanner<'a> {
    fn from(value: &'a str) -> Self {
        return Self {
            ln: 0,
            left: 0,
            right: 0,
            data: Cow::Borrowed(value),
        };
    }
}

impl<'a> fmt::Display for Scanner<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.as_str());
    }
}
