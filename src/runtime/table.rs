use crate::types::{Limit, RefType, TableIdx, TableType};

use super::value::Ref;

#[derive(Debug, Clone, Copy)]
pub struct TableElementIdx(pub usize);

#[derive(Debug)]
pub struct Tables(Vec<Table>);

impl Tables {
    pub fn new(table_types: &[TableType]) -> Self {
        Self(
            table_types
                .iter()
                .map(|TableType(RefType::FuncRef, limit)| Table::new(*limit))
                .collect(),
        )
    }

    pub fn table_mut(&mut self, TableIdx(table_idx): TableIdx) -> &mut Table {
        &mut self.0[table_idx as usize]
    }

    pub fn table(&self, TableIdx(table_idx): TableIdx) -> &Table {
        &self.0[table_idx as usize]
    }

    pub fn copy(
        &mut self,
        dst_idx: TableIdx,
        src_idx: TableIdx,
        TableElementIdx(dst_offset): TableElementIdx,
        TableElementIdx(src_offset): TableElementIdx,
        len: usize,
    ) {
        // Borrow the tables to get access to their elements
        let mut refs = vec![None; len];

        // Perform the copy
        let src_table = self.table(src_idx);
        for (i, ref_value) in refs.iter_mut().enumerate() {
            let table_value = src_table.get(TableElementIdx(src_offset + i));
            *ref_value = table_value;
        }

        let dst_table = self.table_mut(dst_idx);
        for (i, ref_value) in refs.iter().enumerate() {
            dst_table.set(TableElementIdx(dst_offset + i), *ref_value);
        }
    }
}

#[derive(Debug)]
pub struct Table {
    refs: Vec<Ref>,
}

impl Table {
    pub fn new(limit: Limit) -> Self {
        Self {
            refs: vec![Default::default(); limit.min as usize],
        }
    }

    pub fn get(&self, TableElementIdx(idx): TableElementIdx) -> Ref {
        self.refs[idx]
    }

    pub fn set(&mut self, TableElementIdx(idx): TableElementIdx, element: Ref) {
        self.refs[idx] = element
    }

    pub fn fill(&mut self, TableElementIdx(idx): TableElementIdx, elements: &[Ref]) {
        self.refs[idx..idx + elements.len()].copy_from_slice(elements)
    }

    pub fn fill_value(&mut self, TableElementIdx(idx): TableElementIdx, value: Ref, len: usize) {
        for i in 0..len {
            self.refs[idx + i] = value;
        }
    }

    pub fn grow(&mut self, len: usize, value: Ref) -> usize {
        self.refs.resize(len, value);
        self.refs.len()
    }

    pub fn size(&self) -> usize {
        self.refs.len()
    }
}
