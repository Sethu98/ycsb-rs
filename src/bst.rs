extern crate data_structures;

use std::cell::RefCell;
use std::collections::HashMap;

use data_structures::binary_search_tree::{LockFreeBinarySearchTree, TreeParams};
use data_structures::fix_sized_key::{FixSizedKey, FixSizedKeyParams};
use data_structures::interfaces::Tree;

use crate::db::DB;

// struct YCSBParams;
//
// impl FixSizedKeyParams for YCSBParams {
//     const KEY_SIZE: usize = 32;
//     const ALLOW_INT_CMP: bool = true;
// }
//
// impl TreeParams for YCSBParams {
//     type ValueType = String;
//     type IKeyType = FixSizedKey<YCSBParams>;
// }
//
// type LockFreeBSTYCSB = LockFreeBinarySearchTree<YCSBParams>;
//
// pub struct BST {
//     tables: RefCell<HashMap<&'static str, LockFreeBSTYCSB>>,
// }
//
// impl BST {
//     fn new() -> Self {
//         BST {
//             tables: RefCell::new(HashMap::new())
//         }
//     }
// }
//
// impl DB for BST {
//     fn init(&self) -> anyhow::Result<()> {
//         Ok(())
//     }
//
//     fn insert(&self, table: &str, key: &str, values: &HashMap<&str, String>) -> anyhow::Result<()> {
//         let mut_self = unsafe { &mut *(self as *const Self as *mut Self) };
//         let value = values.get(&"field1").unwrap();
//
//         let tree: &mut LockFreeBSTYCSB = mut_self.tables.borrow_mut()
//             .entry(table.clone().into())
//             .or_insert(LockFreeBSTYCSB::new());
//
//         tree.put(key, value.clone());
//
//         Ok(())
//     }
//
//     fn read(&self, table: &str, key: &str, result: &mut HashMap<String, String>) -> anyhow::Result<()> {
//         let mut_self = unsafe { &mut *(self as *const Self as *mut Self) };
//         let tree: &mut LockFreeBSTYCSB = mut_self.tables.borrow_mut().entry(table.clone().into())
//             .or_insert(LockFreeBSTYCSB::new());
//
//         if let Some(res) = tree.get(key) {
//             result.insert(String::from("field1"), res.clone());
//         }
//
//         Ok(())
//     }
// }