// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::table::Table;
use std::collections::BTreeMap;
use anyhow::{bail, Result};

pub struct Tdb {
    tables: BTreeMap<String, Table>, // key is tablename
}

impl Tdb {
    pub fn write(&self, decimals: usize) -> Result<String> {
        bail!("Tdb::write not implemented") // TODO
    }
}
