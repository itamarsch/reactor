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
}
