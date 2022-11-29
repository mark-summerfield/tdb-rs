// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::field::Field;
use crate::value::Record;
use std::collections::BTreeMap;

pub struct Table {
    name: String,                    // tablename
    fields: BTreeMap<String, Field>, // metadata
    records: Vec<Record>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            name: "".to_string(),
            fields: BTreeMap::new(),
            records: vec![],
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && !self.fields.is_empty()
    }

    pub fn columns(&self) -> usize {
        self.fields.len()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }
}
