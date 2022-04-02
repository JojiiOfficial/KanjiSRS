use crate::{
    sm2::{RepQuality, SM2},
    utils,
};
use chrono::{TimeZone, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{copy, rename, File},
    io::BufReader,
    path::Path,
};

/// Storage for SRS data. The
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SRSStorage {
    file: String,
    data: HashMap<u32, Item>,
}

/// SRS info holding item
#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub struct Item {
    pub id: u32,
    pub srs_data: SM2,
    pub due_on: u64,
    pub in_learning: bool,
}

impl SRSStorage {
    /// Create a new srs-storage
    #[inline]
    pub fn new<P: AsRef<str>>(file: P) -> Self {
        let file = file.as_ref().to_string();

        if Path::new(&file).exists() {
            return Self::load(file).expect("Failed to load srs storage");
        }

        Self {
            file,
            data: HashMap::new(),
        }
    }

    /// Adds a new SRS item to the srs-storage
    pub fn add(&mut self, id: u32) -> bool {
        if self.find(id).is_some() {
            return false;
        }

        let new_item = Item::new(id);

        self.data.insert(id, new_item).is_none()
    }

    /// Returns an iterator over all SRS items, mutable
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Item> {
        self.data.iter_mut().map(|i| i.1)
    }

    /// Returns `true` if the storage does not contain a single item
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns an iterator over all SRS items
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Item> {
        self.data.iter().map(|i| i.1)
    }

    /// Find an SRS item by its item id mutable
    #[inline]
    pub fn get_mut(&mut self, id: u32) -> Option<&mut Item> {
        self.data.get_mut(&id)
    }

    /// Find an SRS item by its item id
    #[inline]
    pub fn find(&self, id: u32) -> Option<&Item> {
        self.data.get(&id)
    }

    /// Removes an item from the srs storage
    #[inline]
    pub(super) fn remove(&mut self, id: u32) -> Option<Item> {
        self.data.remove(&id)
    }

    /// Returns all items that need review
    pub fn get_due(&self) -> impl Iterator<Item = &Item> {
        let unix = utils::get_today_unix();
        let mut due = self
            .data
            .iter()
            .filter(move |i| {
                let due = i.1.due_on as u64;
                due > 0 && due <= unix && i.1.in_learning
            })
            .map(|i| i.1)
            .collect::<Vec<_>>();
        due.sort_by(|a, b| a.id.cmp(&b.id));
        due.into_iter()
    }

    /// Returns an iterator over new items
    pub fn get_new(&self) -> impl Iterator<Item = &Item> {
        let mut unlearned = self
            .data
            .iter()
            .filter_map(|i| (!i.1.in_learning).then(|| *i.0))
            .collect::<Vec<_>>();
        unlearned.sort_unstable();
        unlearned.into_iter().filter_map(|i| self.data.get(&i))
    }

    fn backup(&self) {
        let backup = Path::new(&self.file).with_file_name("srs_backup");
        let _ = copy(&self.file, backup);
    }

    /// Saves the SRS storage
    pub fn save(&self) {
        self.backup();
        let path = Path::new(&self.file).with_file_name("srs_data_new");
        let file = File::create(&path).expect("Couldn't write SrsStorage file");
        bincode::serialize_into(&file, &self).unwrap();
        if !Self::check_file(&path) {
            panic!("Failed to save srs storage");
        }
        rename(path, &self.file).unwrap()
    }

    fn check_file<F: AsRef<Path>>(file: F) -> bool {
        Self::load(file).is_some()
    }

    fn load<P: AsRef<Path>>(file: P) -> Option<Self> {
        let r = File::open(&file).ok()?;
        let item_storage: Self = bincode::deserialize_from(BufReader::new(r)).ok()?;
        return Some(item_storage);
    }
}

impl Item {
    /// Create a new item
    #[inline]
    pub fn new(id: u32) -> Self {
        let srs_data = SM2::new();
        Self {
            id,
            srs_data,
            in_learning: false,
            due_on: 0,
        }
    }

    /// Review an item
    pub fn review(&mut self, r_quality: RepQuality) {
        self.in_learning = true;
        self.srs_data = self.srs_data.review(r_quality);
        let next_review = self.srs_data.new_interval(&r_quality) as u32;
        self.due_on = utils::unix_n_days_offset(next_review)
    }

    /// Resets an item completely
    pub fn reset(&mut self) {
        *self = Self::new(self.id);
        self.in_learning = false;
        self.due_on = 0;
    }

    /// Returns `true` if Item can be learned or reviewed
    pub fn can_be_reviewed(&self) -> bool {
        if !self.in_learning || self.due_on == 0 {
            return true;
        }

        let unix = utils::get_today_unix();

        self.due_on <= unix
    }

    pub fn 直す(&mut self) {
        let time = Utc.timestamp(self.due_on as i64, 0);
        let time = time
            .with_hour(4)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_minute(0)
            .unwrap();
        self.due_on = time.timestamp() as u64;
    }
}
