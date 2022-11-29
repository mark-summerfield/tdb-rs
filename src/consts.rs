// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

/*! Constants used throughout the tdb module. */
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

pub const NL: u8 = b'\n';
pub const NUL: char = '\0';

pub static ISO8601_DATE: &str = "%Y-%m-%d"; //YYYY-MM-DD
pub static ISO8601_DATETIME: &str = "%Y-%m-%dT%H:%M:%S"; // YYYY-MM-DDTHH:MM:SS
pub static ISO8601_DATETIME_M: &str = "%Y-%m-%dT%H:%M"; // YYYY-MM-DDTHH:MM
pub static ISO8601_DATETIME_H: &str = "%Y-%m-%dT%H"; // YYYY-MM-DDTHH

pub static VTYPE_NAME_BOOL: &str = "bool";
pub static VTYPE_NAME_BYTES: &str = "bytes";
pub static VTYPE_NAME_DATE: &str = "date";
pub static VTYPE_NAME_DATETIME: &str = "datetime";
pub static VTYPE_NAME_INT: &str = "int";
pub static VTYPE_NAME_REAL: &str = "real";
pub static VTYPE_NAME_STR: &str = "str";

pub static DATE_STR_SENTINAL: &str = "1808-08-08";
pub static DATE_TIME_STR_SENTINAL: &str = "1808-08-08T08:08:08";
pub const INT_SENTINAL: i64 = -1808080808;
pub const REAL_SENTINAL: f64 = -1808080808.0808;
