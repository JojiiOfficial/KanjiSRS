use std::{
    fs::{copy, rename, File},
    io::BufReader,
    path::Path,
};

use serde::{Deserialize, Serialize};

/// Storage for single characters to learn
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ItemStorage {
    file: String,
    items: Vec<Item>,
    last_id: u32,
}

/// A single item to learn
#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub struct Item {
    pub id: u32,
    pub literal: char,
}

impl ItemStorage {
    /// Creates a new ItemStorage
    #[inline]
    pub fn new<P: AsRef<str>>(file: P) -> Self {
        let file = file.as_ref().to_string();

        if Path::new(&file).exists() {
            return Self::load(file).expect("Failed to load item sorage");
        }

        Self {
            file,
            items: vec![],
            last_id: 0,
        }
    }

    /// Returns the length of the itemStorage
    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` if itemStorage is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an Item by its id. `None` if there is no item with given id in the
    /// storage
    #[inline]
    pub fn item_by_id(&self, id: u32) -> Option<&Item> {
        self.items.iter().find(|i| i.id == id)
    }

    /// Returns an Item by its literal. `None` if there is no item with given literal in the
    /// storage
    #[inline]
    pub fn item_by_literal(&self, lit: char) -> Option<&Item> {
        self.items.iter().find(|i| i.literal == lit)
    }

    /// Adds a new item to the storage
    pub(super) fn add_item(&mut self, literal: char) -> Option<&Item> {
        // literal already in storage
        if self.item_by_literal(literal).is_some() {
            return None;
        }

        self.items.push(Item {
            id: self.last_id + 1,
            literal,
        });

        self.last_id += 1;

        self.items.last()
    }

    /// Returns an Iterator over all items in the stroage
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Item> {
        self.items.iter()
    }

    fn backup(&self) {
        let backup = Path::new(&self.file).with_file_name("item_backup");
        let _ = copy(&self.file, backup);
    }

    /// Save ItemStorage
    pub fn save(&self) {
        self.backup();
        let path = Path::new(&self.file).with_file_name("item_data_new");
        let file = File::create(&path).expect("Couldn't write Item Storage file");
        bincode::serialize_into(&file, &self).unwrap();
        if !Self::check_file(&path) {
            panic!("Failed to save item storage");
        }
        rename(path, &self.file).unwrap()
    }

    /// Remove an item by its ID
    pub(super) fn remove_item(&mut self, id: u32) -> bool {
        let len = self.items.len();
        self.items.retain(|i| i.id != id);
        self.items.len() < len
    }

    fn check_file<P: AsRef<Path>>(file: P) -> bool {
        Self::load(file).is_some()
    }

    fn load<P: AsRef<Path>>(file: P) -> Option<Self> {
        let r = File::open(&file).ok()?;
        let item_storage: Self = bincode::deserialize_from(BufReader::new(r)).ok()?;
        return Some(item_storage);
    }
}
