// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::field::Field;
use crate::value::Record;
use std::collections::HashMap;

pub struct Table<'a> {
    name: String,                   // tablename
    field_names: Vec<&'a str>,      // to preserve reading order
    fields: HashMap<String, Field>, // metadata
    records: Vec<Record>,
}

impl<'a> Table<'a> {
    pub fn new() -> Table<'a> {
        Table {
            name: "".to_string(),
            field_names: vec![],
            fields: HashMap::new(),
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
