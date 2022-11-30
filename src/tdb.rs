// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::table::Table;
use anyhow::{bail, Result};
use std::collections::HashMap;

pub struct Tdb<'a> {
    table_names: Vec<&'a str>, // ref to tablename to preserve reading order
    tables: HashMap<String, Table>, // key is tablename
}

impl<'a> Tdb<'a> {
    pub fn write(&self, decimals: usize) -> Result<String> {
        bail!("Tdb::write not implemented") // TODO
    }
}
