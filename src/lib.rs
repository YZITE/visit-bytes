#![cfg_attr(not(feature = "camino"), no_std)]
#![forbid(unsafe_code)]

/** `visit-bytes` applies a visitor pattern to byte sequences,
  and is primarily used for (byte-)string rewriting in
  diverse/heterogenous structures (e.g. in-memory directory trees)
**/

pub trait Element {
    fn accept<V: Visitor>(&self, visitor: &mut V);
    fn accept_mut<V: VisitorMut>(&mut self, visitor: &mut V);
}

pub trait Visitor {
    fn visit_bytes(&mut self, bytes: &[u8]);
}

pub trait VisitorMut {
    fn visit_bytes(&mut self, bytes: &mut [u8]);
}

impl Element for [u8] {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_bytes(self);
    }
    fn accept_mut<V: VisitorMut>(&mut self, visitor: &mut V) {
        visitor.visit_bytes(self);
    }
}

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::String;

#[cfg(feature = "alloc")]
impl Element for String {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_bytes(self.as_bytes());
    }
    fn accept_mut<V: VisitorMut>(&mut self, visitor: &mut V) {
        let mut bytes = self.as_bytes().to_vec();
        visitor.visit_bytes(&mut bytes[..]);
        *self = Self::from_utf8(bytes).expect("illegal hash characters used");
    }
}

#[cfg(feature = "camino")]
use camino_::Utf8PathBuf;

#[cfg(feature = "camino")]
impl Element for Utf8PathBuf {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_bytes(self.as_str().as_bytes());
    }
    fn accept_mut<V: VisitorMut>(&mut self, visitor: &mut V) {
        let mut s = String::from(core::mem::replace(self, Utf8PathBuf::from("")));
        s.accept_mut(visitor);
        *self = s.into();
    }
}
