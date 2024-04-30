// struct MyHashMap {
//     tables: Vec<i32>,
// }
//
// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl MyHashMap {
//     fn new() -> Self {
//         MyHashMap {
//             tables: vec![],
//         }
//     }
//
//     fn put(&self, key: i32, value: i32) {
//         self.tables[key] = value;
//     }
//
//     fn get(&self, key: i32) -> i32 {
//         0
//     }
//
//     fn remove(&self, key: i32) {}
// }