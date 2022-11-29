// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

pub struct Field {
    name: String, // fieldname
    kind: FieldType,
}

#[derive(Clone, Debug)]
pub enum FieldType {
    Bool,
    Bytes,
    Date,
    DateTime,
    Int,
    Real,
    Str,
}
