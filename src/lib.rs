#[macro_use]
extern crate libc;

#[macro_use]
extern crate serde_json;

use serde_json::Value;

use libc::{c_int, c_char};

use std::ffi::{CStr,CString};

#[macro_use]
extern crate nom;

use std::str;
use nom::{alpha, is_alphabetic};


//label = (letter() + many(letter() | digit() | string('_'))).parsecmap(
/*
eq_condition = (
  s(label) + s(string('=')) + s(string('?'))
).parsecmap(
  lambda s: BaseCondition(label=s[0][0], type='eq')
)
@attr.s
class BaseCondition(object):
  label = attr.ib()
  type = attr.ib()
*/
named!(label<&[u8], &str>,
    map_res!(take_while!(is_alphabetic), str::from_utf8)
);

#[derive(Debug,PartialEq,Clone)]
pub enum CondType {
    Eq,
}
#[derive(Debug,PartialEq,Clone)]
pub struct BaseCond {
    pub label: String,
    pub cond_type: CondType,
}
named!(eq_cond<&[u8], BaseCond>,
    do_parse!(
        label: label >>
        many0!(char!(' ')) >>
        one_of!("=") >>
        many0!(char!(' ')) >>
        one_of!("?") >>
        (BaseCond { label: String::from(label), cond_type: CondType::Eq })
    )
);

#[no_mangle]
pub extern fn base_cond_parse(c: *const c_char) -> *mut BaseCond {
    unsafe {
        let s = CStr::from_ptr(c).to_bytes();
        let q = eq_cond(s).unwrap().1;
        let b = Box::new(q);
        Box::into_raw(b)
    }
}

#[no_mangle]
pub extern fn base_cond_free(ptr: *mut BaseCond) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern fn base_cond2json(ptr: *mut BaseCond) -> *const c_char {
    let cond = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let mut hm0 = serde_json::Map::new();
    hm0.insert(cond.label.clone(), json!("?"));
    let js = json!({
        "terms": hm0
    });
    let s = serde_json::to_string_pretty(&js).unwrap();
    let s2 = CString::new(s.clone()).unwrap();
    let p = s2.as_ptr();
    std::mem::forget(s2);
    p
}

#[no_mangle]
pub extern fn cond2query_str(ptr: *mut Cond) -> *const c_char {
    let cond = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let s = format!(
        "{} = ? {} {} = ?",
        &cond.cond0.label,
        match &cond.cond_join_type {
            And => "AND",
            Or => "OR",
        },
        &cond.cond1.label,
    );
    let s2 = CString::new(s.clone()).unwrap();
    let p = s2.as_ptr();
    std::mem::forget(s2);
    p
}

#[no_mangle]
pub extern fn base_cond_plus(ptr0: *mut BaseCond, ptr1: *mut BaseCond) -> *mut Cond {
    let cond0 = unsafe {
        assert!(!ptr0.is_null());
        &*ptr0
    };
    let cond1 = unsafe {
        assert!(!ptr1.is_null());
        &*ptr1
    };
    unsafe {
        let cond = Cond {
            cond_join_type: CondJoinType::And,
            cond0: Box::new(cond0.clone()),
            cond1: Box::new(cond1.clone()),
        };
        let b = Box::new(cond);
        Box::into_raw(b)
    }
}

/*
@attr.s
class ConditionJoin(object):
  type = attr.ib()
  conditionA = attr.ib()
  conditionB = attr.ib()
  */
#[derive(Debug,PartialEq)]
pub enum CondJoinType {
    And,
    Or,
}
#[derive(Debug,PartialEq)]
pub struct Cond {
    pub cond_join_type: CondJoinType,
    pub cond0: Box<BaseCond>,
    pub cond1: Box<BaseCond>,
}

#[derive(Debug,PartialEq)]
pub enum ACond {
    Cond(Cond),
    BaseCond(BaseCond),
}


named!(cond_join<&[u8], Cond>,
    do_parse!(
        c0 : eq_cond >>
        many0!(char!(' ')) >>
        tag!("AND") >>
        many0!(char!(' ')) >>
        c1 : eq_cond >>
        (Cond { cond_join_type: CondJoinType::And, cond0: Box::new(c0), cond1: Box::new(c1) })
    )
);

named!(where_clause<&[u8], BaseCond>,
    do_parse!(
        tag!("WHERE") >>
        many0!(char!(' ')) >>
        c: eq_cond >>
        (c)
    )
);

#[derive(Debug,PartialEq)]
pub struct Query {
    pub label: String,
    pub cond: BaseCond
}

named!(query<&[u8], Query>,
    do_parse!(
        tag!("QUERY") >>
        many0!(char!(' ')) >>
        l: label >>
        many0!(char!(' ')) >>
        c: where_clause >>
        (Query { label: String::from(l), cond: c })
    )
);

#[no_mangle]
pub extern fn query_parse(c: *const c_char) -> *mut Query {
    unsafe {
        let s = CStr::from_ptr(c).to_bytes();
        let q = query(s).unwrap().1;
        let b = Box::new(q);
        Box::into_raw(b)
    }
}

#[no_mangle]
pub extern fn query2json(ptr: *mut Query) -> *const c_char {
    let q = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let cond = &q.cond;
    let mut hm0 = serde_json::Map::new();
    hm0.insert(cond.label.clone(), json!("?"));
    let js = json!({
        "query": {
            "filter": {
                "bool": {
                    "must": [
                      { "terms": hm0 }
                    ]
                }
            }
        }
    });
    let s = serde_json::to_string_pretty(&js).unwrap();
    let s2 = CString::new(s.clone()).unwrap();
    let p = s2.as_ptr();
    std::mem::forget(s2);
    p
}

#[no_mangle]
pub extern fn cond_free(ptr: *mut Cond) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern fn query_free(ptr: *mut Query) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern fn query_label(ptr: *mut Query) -> *const c_char {
    let query = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let s = CString::new(query.label.clone()).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}
