use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

// Arc: Atomic Reference Counted
//  This is similar to a smart pointer.
//  Grants multiple ownership over something
// Mutex: Mutual Exclusion
//  This is a synchronization primitive used to prevent
//  threads from accessing shared data at the same time
// HashMap: Your regular old hashmap!
// tl;dr:
//     Arc gives multiple people keys
//     Mutex makes sure only one person can unlock at any time
pub type Db = Arc<Mutex<HashMap<i8, String>>>;
