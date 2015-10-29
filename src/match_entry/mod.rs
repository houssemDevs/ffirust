

use libc::{c_char, uint32_t,size_t};
use std::ffi::CString;

#[allow(dead_code)]
#[derive(Debug,Clone)]
#[repr(C)]
pub struct MatchEntry {
    pub file: String,
    pub kind: String,
    pub pos: (u32,u32),
    pub name: String
}

#[allow(dead_code)]
impl MatchEntry {
    pub fn new() -> MatchEntry {
        MatchEntry{file:".".to_string(),kind:"".to_string(),
                    name:"".to_string(),pos:(0,0)}
    }
    pub fn name(mut self, nm: String) -> Self {
        self.name = nm;
        self
    }
    pub fn kind(mut self, kd: String) -> Self {
        self.kind = kd;
        self
    }
    pub fn pos(mut self, p: (u32,u32)) -> Self {
        self.pos = p;
        self
    }
    pub fn file(mut self, f: String) -> Self {
        self.file = f;
        self
    }
}


#[derive(Debug)]
#[repr(C)]
pub struct c_mentry {
    pub file: *mut c_char,
    pub kind: *mut c_char,
    pub name: *mut c_char,
    pub row: uint32_t,
    pub col: uint32_t
}

impl Into<c_mentry> for MatchEntry {
    fn into(self) -> c_mentry {
        c_mentry{name:CString::new(self.name.into_bytes()).unwrap().into_raw(),
                 kind: CString::new(self.kind.into_bytes()).unwrap().into_raw(),
                 file: CString::new(self.file.into_bytes()).unwrap().into_raw(),
                 row: self.pos.0 as uint32_t, col: self.pos.1 as uint32_t}
    }
}


#[repr(C)]
#[derive(Debug)]
pub struct c_mentries {
    pub entries: *mut c_mentry,
    pub len: size_t
}

impl From<Vec<MatchEntry>> for c_mentries {
    fn from(v: Vec<MatchEntry>) -> Self {
        let mut vcm: Vec<c_mentry> = v.iter().map(|x| {let b = x.clone(); b.into() } ).collect();
        c_mentries{entries:vcm.as_mut_ptr(),len:vcm.len() as size_t}
    }
}

// #[test]
// fn me_builder() {
//     let mut me = MatchEntry::new().name("houssem".to_string()).kind("humain".to_string());
//     println!("{:?}", me);
// }
