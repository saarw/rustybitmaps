use roaring::RoaringBitmap;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct Bitmaps {
	next_index: RwLock<u32>,
	bitmaps: RwLock<HashMap<u32, RwLock<RoaringBitmap<u32>>>>
}

impl Bitmaps {
	pub fn new() -> Bitmaps {
		Bitmaps{next_index: RwLock::new(1), bitmaps: RwLock::new(HashMap::new())}
	}
	
	pub fn create_new(&self) -> Result<u32, String> {
		let index = *self.next_index.read().unwrap();
		self.bitmaps.write().unwrap().insert(index, RwLock::new(RoaringBitmap::new()));
		*self.next_index.write().unwrap() += 1;
		return Ok(index);		
	}
	
	pub fn insert_item(&self, item: u32, bitmap_indices: Vec<u32>) -> Result<String, String> {
		if bitmap_indices.len() == 0 {
			return Err(format!("No bitmaps indices specified"));
		}
		for i in bitmap_indices {					
			match self.bitmaps.read().unwrap().get(&i) {
				Some(ref bitmap_lock) => bitmap_lock.write().unwrap().insert(item),
				None => return Err(format!("No bitmap with id {}", i))
			};
		}
		Ok("true".to_string())
	}
	
	pub fn contains_item(&self, item: u32, bitmap_indices: Vec<u32>) -> Result<bool, String> {
		if bitmap_indices.len() == 0 {
			return Err(format!("No bitmap indices specified"));
		}
		for i in bitmap_indices {
			match self.bitmaps.read().unwrap().get(&i) {
				Some(ref bitmap_lock) => if !bitmap_lock.read().unwrap().contains(item) { return Ok(false); }, 				
				None => return Err(format!("No bitmap with id {}", i))
			};
		}
		Ok(true)
	}
}

