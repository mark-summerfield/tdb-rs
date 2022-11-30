// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

/*!
A Value object represents a Tdb field's value in memory.
*/
use crate::consts::*;
use crate::util::{escape, isclose64, sanitize_decimals};
use chrono::{NaiveDate, NaiveDateTime};
use std::fmt::{self, Write as _};

pub type Record = Vec<Option<Value>>;

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Bytes(Vec<u8>),
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    Int(i64),
    Real(f64),
    Str(String),
}

impl Value {
    /// Returns `true` if `Value::Bool`; otherwise returns `false`.
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    /// Returns `true` if `Value::Bytes`; otherwise returns `false`.
    pub fn is_bytes(&self) -> bool {
        matches!(self, Value::Bytes(_))
    }

    /// Returns `true` if `Value::Date`; otherwise returns `false`.
    pub fn is_date(&self) -> bool {
        matches!(self, Value::Date(_))
    }

    /// Returns `true` if `Value::DateTime`; otherwise returns `false`.
    pub fn is_datetime(&self) -> bool {
        matches!(self, Value::DateTime(_))
    }

    /// Returns `true` if `Value::Int`; otherwise returns `false`.
    pub fn is_int(&self) -> bool {
        matches!(self, Value::Int(_))
    }

    /// Returns `true` if `Value::Real`; otherwise returns `false`.
    pub fn is_real(&self) -> bool {
        matches!(self, Value::Real(_))
    }

    /// Returns `true` if `Value::Str`; otherwise returns `false`.
    pub fn is_str(&self) -> bool {
        matches!(self, Value::Str(_))
    }

    /// Returns `Some(bool)` if `Value::Bool`; otherwise returns `None`.
    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// Returns `Some(&Vec<u8>)` if `Value::Bytes`; otherwise returns
    /// None`.
    pub fn as_bytes(&self) -> Option<&Vec<u8>> {
        if let Value::Bytes(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// Returns `Some(NaiveDate)` if `Value::Date`; otherwise returns
    /// `None`.
    pub fn as_date(&self) -> Option<NaiveDate> {
        if let Value::Date(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// Returns `Some(NaiveDateTime)` if `Value::DateTime`; otherwise
    /// returns `None`.
    pub fn as_datetime(&self) -> Option<NaiveDateTime> {
        if let Value::DateTime(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// Returns `Some(i64)` if `Value::Int`; otherwise returns `None`.
    pub fn as_int(&self) -> Option<i64> {
        if let Value::Int(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// Returns `Some(f64)` if `Value::Real`; otherwise returns `None`.
    pub fn as_real(&self) -> Option<f64> {
        if let Value::Real(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// Returns `Some(&str)` if `Value::Str`; otherwise returns `None`.
    pub fn as_str(&self) -> Option<&str> {
        if let Value::Str(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// Returns the Value's `typename` (`bool`, `bytes', ... `str`).
    pub fn typename(&self) -> &'static str {
        match self {
            Value::Bool(_) => VTYPE_NAME_BOOL,
            Value::Bytes(_) => VTYPE_NAME_BYTES,
            Value::Date(_) => VTYPE_NAME_DATE,
            Value::DateTime(_) => VTYPE_NAME_DATETIME,
            Value::Int(_) => VTYPE_NAME_INT,
            Value::Real(_) => VTYPE_NAME_REAL,
            Value::Str(_) => VTYPE_NAME_STR,
        }
    }

    /// Returns the value as a Tdb field value correctly accounting for
    /// decimals and sentinals. The caller should check for None and output
    /// '?' in that case; otherwise use this.
    pub fn text(&self, decimals: usize) -> String {
        match self {
            Value::Bool(true) => "T".to_string(),
            Value::Bool(false) => "F".to_string(),
            Value::Bytes(b) => bytes_to_tdb(b),
            Value::Date(d) => {
                let s = d.format(ISO8601_DATE).to_string();
                if s == DATE_STR_SENTINAL {
                    "!".to_string()
                } else {
                    s
                }
            }
            Value::DateTime(dt) => {
                let s = dt.format(ISO8601_DATETIME).to_string();
                if s == DATE_TIME_STR_SENTINAL {
                    "!".to_string()
                } else {
                    s
                }
            }
            Value::Int(i) => {
                if *i == INT_SENTINAL {
                    "!".to_string()
                } else {
                    i.to_string()
                }
            }
            Value::Real(r) => {
                if isclose64(*r, REAL_SENTINAL) {
                    "!".to_string()
                } else {
                    let dp = sanitize_decimals(decimals);
                    if dp == 0 {
                        r.to_string()
                    } else {
                        format!("{r:.dp$}")
                    }
                }
            }
            Value::Str(s) => format!("<{}>", escape(s)),
        }
    }
}

impl fmt::Display for Value {
    /// Provides a .to_string() that returns a valid Tdb field value; does
    /// _not_ account for the decimals setting _or_ for sentinal values:
    /// use Value::value() to account for these.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Bool(true) => "T".to_string(),
                Value::Bool(false) => "F".to_string(),
                Value::Bytes(b) => bytes_to_tdb(b),
                Value::Date(d) => d.format(ISO8601_DATE).to_string(),
                Value::DateTime(dt) =>
                    dt.format(ISO8601_DATETIME).to_string(),
                Value::Int(i) => i.to_string(),
                Value::Real(r) => r.to_string(),
                Value::Str(s) => format!("<{}>", escape(s)),
            }
        )
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<Vec<u8>> for Value {
    fn from(b: Vec<u8>) -> Self {
        Value::Bytes(b)
    }
}

impl From<NaiveDate> for Value {
    fn from(d: NaiveDate) -> Self {
        Value::Date(d)
    }
}

impl From<NaiveDateTime> for Value {
    fn from(dt: NaiveDateTime) -> Self {
        Value::DateTime(dt)
    }
}

impl From<usize> for Value {
    fn from(i: usize) -> Self {
        Value::Int(i as i64)
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::Int(i as i64)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Int(i)
    }
}

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        Value::Real(f as f64)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Real(f)
    }
}

impl From<&str> for Value {
    /// Converts a &str to a Value::Str.
    fn from(s: &str) -> Self {
        Value::Str(s.to_string())
    }
}

impl From<String> for Value {
    /// Converts a String to a Value::Str.
    fn from(s: String) -> Self {
        Value::Str(s)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::Bool(b) => {
                if let Some(other) = other.as_bool() {
                    *b == other
                } else {
                    false
                }
            }
            Value::Bytes(b) => {
                if let Some(other) = other.as_bytes() {
                    b == other
                } else {
                    false
                }
            }
            Value::Date(d) => {
                if let Some(other) = other.as_date() {
                    *d == other
                } else {
                    false
                }
            }
            Value::DateTime(dt) => {
                if let Some(other) = other.as_datetime() {
                    *dt == other
                } else {
                    false
                }
            }
            Value::Int(i) => {
                if let Some(other) = other.as_int() {
                    *i == other
                } else {
                    false
                }
            }
            Value::Str(s) => {
                if let Some(other) = other.as_str() {
                    s == other
                } else {
                    false
                }
            }
            Value::Real(r) => {
                if let Some(other) = other.as_real() {
                    isclose64(*r, other)
                } else {
                    false
                }
            }
        }
    }
}

impl Eq for Value {}

pub(crate) fn bytes_to_tdb(b: &[u8]) -> String {
    let mut s = String::from("(");
    for x in b {
        let _ = write!(s, "{:02X}", x);
    }
    s.push_str(")");
    s
}
