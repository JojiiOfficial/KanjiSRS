pub mod item;
pub mod srs;

pub use item::ItemStorage;
pub use srs::SRSStorage;

use crate::sm2::SM2;

/// ItemStorage and SRSStorage combined for functions which need both
#[derive(Debug)]
pub struct Storage {
    item_storage: ItemStorage,
    srs_storage: SRSStorage,
}

/// A full storage item
#[derive(Debug)]
pub struct Item<'a> {
    item_val: &'a item::Item,
    srs: &'a srs::Item,
}

impl<'a> Item<'a> {
    /// Returns the items literal
    #[inline]
    pub fn get_literal(&self) -> char {
        self.item_val.literal
    }

    /// Returns SRS Info
    #[inline]
    pub fn get_srs(&self) -> SM2 {
        self.srs.srs_data
    }

    /// Returns the ID of the item
    #[inline]
    pub fn get_id(&self) -> u32 {
        self.item_val.id
    }

    /// Returns `true` if item is in learning
    #[inline]
    pub fn is_learning(&self) -> bool {
        self.srs.in_learning
    }

    /// Returns `true` if item can be reviewed or learned
    #[inline]
    pub fn can_be_reviewed(&self) -> bool {
        self.srs.can_be_reviewed()
    }
}

impl Storage {
    /// New Storage
    #[inline]
    pub fn new(item: ItemStorage, srs: SRSStorage) -> Self {
        Self {
            item_storage: item,
            srs_storage: srs,
        }
    }

    /// Get srs item mutable
    pub fn get_srs_mut(&mut self, id: u32) -> Option<&mut srs::Item> {
        self.srs_storage.get_mut(id)
    }

    /// Returns a full item from the storage
    pub fn get_by_lit(&self, literal: char) -> Option<Item> {
        let item_val = self.item_storage.item_by_literal(literal)?;
        let srs = self.srs_storage.find(item_val.id)?;
        Some(Item { item_val, srs })
    }

    /// Returns a full item from the storage
    pub fn get_by_id(&self, id: u32) -> Option<Item> {
        let item_val = self.item_storage.item_by_id(id)?;
        let srs = self.srs_storage.find(id)?;
        Some(Item { item_val, srs })
    }

    /// Returns a mutable reference to the srs storage
    #[inline]
    pub fn get_srs_storage_mut(&mut self) -> &mut SRSStorage {
        &mut self.srs_storage
    }

    /// Returns a reference to the srs storage
    #[inline]
    pub fn get_srs_storage(&self) -> &SRSStorage {
        &self.srs_storage
    }

    /// Returns a reference to the item storage
    #[inline]
    pub fn get_item_storage(&self) -> &ItemStorage {
        &self.item_storage
    }

    /// Returns an iterator over all items in the storage
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = Item> {
        self.srs_storage.iter().filter_map(|i| self.get_by_id(i.id))
    }

    /// Adds a new item to the storage
    pub fn add(&mut self, literal: char) -> bool {
        let item = self.item_storage.add_item(literal);
        if item.is_none() {
            return false;
        }
        let item = item.unwrap();

        self.srs_storage.add(item.id)
    }

    /// Removes an item from the storage
    pub fn remove(&mut self, literal: char) -> bool {
        let item = self.item_storage.item_by_literal(literal).copied();
        if item.is_none() {
            return false;
        }
        let item = item.unwrap();

        let success =
            self.item_storage.remove_item(item.id) && self.srs_storage.remove(item.id).is_some();

        success
    }

    /// Resets an item by its literal
    pub fn reset(&mut self, literal: char) -> bool {
        let id = match self.get_by_lit(literal) {
            Some(i) => i.get_id(),
            None => return false,
        };

        self.get_srs_mut(id)
            .map(|i| {
                i.reset();
                true
            })
            .unwrap_or(false)
    }

    /// Returns the amount of items in the storage
    #[inline]
    pub fn len(&self) -> usize {
        self.item_storage.len()
    }

    /// Returns `true` if the storage is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the amount of kanji currently in learning
    #[inline]
    pub fn learning_kanji(&self) -> usize {
        self.srs_storage.iter().filter(|i| i.in_learning).count()
    }

    /// Returns `true` if the storage is properly built or empty and `false` if there is data corruption
    pub fn check(&self) -> bool {
        if self.is_empty() {
            return true;
        }

        if self.item_storage.is_empty() || self.srs_storage.is_empty() {
            return false;
        }

        // check item_storage is subset of srs_storage
        let is_subset_of_srs = !self
            .item_storage
            .iter()
            .any(|item| !self.srs_storage.find(item.id).is_some());

        if !is_subset_of_srs {
            return false;
        }

        // check srs_storage is subset of item_storage
        let srs_subset_of_is = !self
            .srs_storage
            .iter()
            .any(|i| !self.item_storage.item_by_id(i.id).is_some());

        if !srs_subset_of_is {
            return false;
        }

        true
    }

    /// Tries to repair a broken storage. Does nothing if storage isn't broken. Can fail if data is too broken. Returns `true`
    /// if Database has been successfully repaired, `false` on error or if nothing has to be repaired. This can be checked with `check()`
    pub fn repair(&mut self) -> bool {
        if self.check() {
            return false;
        }

        if self.srs_storage.is_empty() {
            // Fill SRS with new empty values
            return self.repair_srs();
        }

        if self.item_storage.is_empty() {
            // Pray to god this won't happen
            return true;
        }

        // TODO: add functionality to merge both and drop broken data in order to continue
        // with items that aren't broken

        false
    }

    /// Fills SRS storage with empty values for kanji in order to have both structures being balanced
    fn repair_srs(&mut self) -> bool {
        let mut updated = 0;
        for i in self.item_storage.iter() {
            let added = self.srs_storage.add(i.id);
            if added {
                updated += 1;
            }
        }
        updated > 0
    }

    fn save(&self) {
        self.item_storage.save();
        self.srs_storage.save();
    }
}

impl Drop for Storage {
    #[inline]
    fn drop(&mut self) {
        self.save()
    }
}
