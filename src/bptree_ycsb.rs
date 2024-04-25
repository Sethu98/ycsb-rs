extern crate data_structures;

use std::cell::RefCell;
use std::collections::HashMap;

use data_structures::binary_search_tree::{LockFreeBinarySearchTree, TreeParams};
use data_structures::fix_sized_key::{FixSizedKey, FixSizedKeyParams};
use data_structures::interfaces::Tree;
use data_structures::bptree::BpTree;

use crate::db::DB;

type BpTreeDef = BpTree<String, String>;

pub struct BpTreeYCSB {
    tables: RefCell<HashMap<String, BpTreeDef>>,
}

impl BpTreeYCSB {
    pub fn new() -> Self {
        BpTreeYCSB {
            tables: RefCell::new(HashMap::new())
        }
    }
}

#[allow(invalid_reference_casting)]
impl DB for BpTreeYCSB {
    fn init(&self) -> anyhow::Result<()> {
        Ok(())
    }

    fn insert(&self, table: &str, key: &str, values: &HashMap<&str, String>) -> anyhow::Result<()> {
        let mut_self = unsafe { &mut *(self as *const Self as *mut Self) };
        // println!("{:?}", values);
        let value = values.get(&"field0").unwrap();

        let mut binding = mut_self.tables.borrow_mut();
        let tree: &mut BpTreeDef = binding.entry(table.to_string())
            .or_insert(BpTreeDef::new());

        tree.put(key.to_string(), value.clone());

        Ok(())
    }

    fn read(&self, table: &str, key: &str, result: &mut HashMap<String, String>) -> anyhow::Result<()> {
        let mut_self = unsafe { &mut *(self as *const Self as *mut Self) };
        let mut binding = mut_self.tables.borrow_mut();
        let tree: &mut BpTreeDef = binding.entry(table.to_string())
            .or_insert(BpTreeDef::new());

        if let Some(res) = tree.get_val(key.to_string()) {
            result.insert(String::from("field0"), res.clone());
        }

        Ok(())
    }
}