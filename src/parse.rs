// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::consts::*;
use crate::table::Table;
use crate::tdb::Tdb;
use anyhow::{bail, Result};

pub fn parse<'a>(raw: &'a [u8]) -> Result<Tdb<'a>> {
    let mut raw = raw;
    let mut db: Tdb;
    let mut table = Table::new();
    let mut lino: usize = 1;
    while raw.len() > 0 {
        let b = raw[0];
        if b == NL {
            lino += 1;
            raw = &raw[1..];
        } else if b == b'[' {
            (raw, table) = read_meta(&raw[1..], &mut lino)?;
        } else {
            raw = read_records(&raw, &mut table, &mut lino)?
        }
    }
    bail!("parse not implemented") // TODO
}

fn read_meta<'a>(
    raw: &'a [u8],
    lino: &mut usize,
) -> Result<(&'a [u8], Table<'a>)> {
    bail!("read_meta not implemented") // TODO
}

fn read_records<'a>(
    raw: &'a [u8],
    table: &mut Table,
    lino: &mut usize,
) -> Result<&'a [u8]> {
    let mut raw = raw;

    raw = &raw[1..]; // TODO delete

    println!("read_records not implemented"); // TODO
    Ok(raw)
}
