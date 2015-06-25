#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate rustc_serialize;
extern crate roaring;

mod rpc;
mod bitmaps;

use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper::method::Method;
use hyper::net::Fresh;
use rustc_serialize::json;
use std::io::Read;
use rpc::{RpcApi, RpcCall};
use bitmaps::Bitmaps;
use std::sync::{RwLock};


fn handle(api: &RpcApi<bitmaps::Bitmaps>, req: Request, res: Response<Fresh>) {
	match decode_request(req) {
		Ok(rpc_req) => {
			 match json::encode(&(api.dispatch(rpc_req))) {
				Ok(encoded_json) => send_response(res, StatusCode::Ok, encoded_json.as_bytes()),
				Err(e) => send_response(res, StatusCode::InternalServerError, format!("Failed to encode response: {}", e).as_bytes()), 	
			 };
		 },
		Err(e) => {
			let (status, message) = e; 
			send_response(res, status, message.as_bytes());
			},
	}
}


fn decode_request(mut req: Request) -> Result<RpcCall, (StatusCode, String)> {
	if req.method == Method::Post {
		let body = &mut String::new();
		info!("Received POST request {}", body);
		if let Err(e) = req.read_to_string(body) {
			Err((StatusCode::InternalServerError, format!("Failed to read body: {}", e)))
		} else {
			match json::decode(body) {
				Ok(call) => Ok(call),
				Err(e) => Err((StatusCode::BadRequest, format!("Error decoding JSON: {}", e)))
			}
		} 
	} else {
		Err((StatusCode::MethodNotAllowed, "Only POST allowed.".to_string()))
	}
}


fn send_response(mut rsp: Response, status: StatusCode, message: &[u8]) {
	info!("Sending response {} : {}", status, String::from_utf8_lossy(message));
	*rsp.status_mut() = status;
	if let Err(e) = rsp.send(message) {		
		error!("Failed to send response: {}", e); 
	}
} 

fn main() {		
	env_logger::init().unwrap();
	info!("Starting up");
	
	let mut rpc_api = RpcApi::new(Bitmaps::new());
	// Create functions that map from generic fxx(&Bitmaps, Vec<String>) to calls
	let create_fn = |bitmaps: &Bitmaps, params: Vec<String>| bitmaps.clone().create_new().map(|res| format!("{}", res));
	let insert_fn = |bitmaps: &Bitmaps, params: Vec<String>| {		
		let mut item_idx: u32;
		let mut bitmap_indices: Vec<u32> = Vec::new();
		match params[0].parse::<u32>() {
			Ok(n) => item_idx = n,
			Err(e) => return Err(format!("{}", e))
			}; 				 
		for i in 1..params.len() {
			match params[i].parse::<u32>() { 
				Ok(idx) => bitmap_indices.push(idx), 
				Err(e) => return Err(format!("{}", e)),
				}; 
		}						
		bitmaps.insert_item(item_idx, bitmap_indices)
	};
	let contains_fn = |bitmaps: &Bitmaps, params: Vec<String>| { 
		let mut item_idx: u32;
		let mut bitmap_indices: Vec<u32> = Vec::new();
		match params[0].parse::<u32>() {
			Ok(n) => item_idx = n,
			Err(e) => return Err(format!("{}", e))
			}; 				 
		for i in 1..params.len() {
			match params[i].parse::<u32>() { 
				Ok(idx) => bitmap_indices.push(idx), 
				Err(e) => return Err(format!("{}", e)),
				}; 
		}
		bitmaps.clone().contains_item(item_idx, bitmap_indices).map(|res| format!("{}", res))
		};		
	rpc_api.register_function("create_new", create_fn); 	
	rpc_api.register_function("insert_item", insert_fn);
	rpc_api.register_function("contains_item", contains_fn);
	
	let api_lock = RwLock::new(rpc_api);
	Server::new(move |req: Request, res: Response<Fresh>| handle(&(*api_lock.read().unwrap()), req, res)).listen("127.0.0.1:8081").unwrap();
}
