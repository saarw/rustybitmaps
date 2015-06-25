extern crate rustc_serialize;

use std::collections::HashMap;
use rustc_serialize::json::Json;
use std::sync::{RwLock};


#[derive(RustcDecodable)]
pub struct RpcCall {
	id:	u32,
	method: String,
	params: Vec<String>
}

#[derive(RustcEncodable)]
pub struct RpcResponse {
	id:	u32,
	result: Json,
	error: Json
}

pub struct RpcApi<T> {
	api_impl: RwLock<Box<T>>,
	method_mappings: RwLock<HashMap<String, RwLock<Box<Fn(&T, Vec<String>) -> Result<String,String> + Sync + Send>>>>, 
}

impl <T> RpcApi<T> {
	pub fn new(p: T) -> RpcApi<T> {
		RpcApi{api_impl: RwLock::new(Box::new(p)), method_mappings: RwLock::new(HashMap::new())}
	}
	
	pub fn dispatch(&self, req: RpcCall) -> RpcResponse {
		let mut result : Result<String, String>; 
		result = match self.method_mappings.read().unwrap().get(&req.method) {
			Some(mapping_lock) => {	
				let api = self.api_impl.read().unwrap();				
				let mapping = mapping_lock.read().unwrap();
				mapping(&(*api), req.params)
				},
			None => Err(format!("Unknown method: {}", req.method))
			};
		match result {
			Ok(data) => RpcResponse{id: req.id, result: Json::String(data), error: Json::Null},
			Err(e) => RpcResponse{id: req.id, result: Json::Null, error: Json::String(e)},			 
		}
	}
	
	pub fn register_function<F: Fn(&T, Vec<String>) -> Result<String, String> + Sync + Send + 'static>(&mut self, s: &str, f: F) {
		self.method_mappings.write().unwrap().insert(s.to_string(), RwLock::new(Box::new(f)));
	}
}


