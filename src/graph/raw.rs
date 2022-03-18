use std::ffi::c_void;

#[repr(C)]
pub struct GraphContext {
	pub layout: *mut c_void,
	pub graph: *mut c_void,
}

#[repr(C)]
pub struct EdgeEntry {
	pub target: u64,
}

#[repr(C)]
pub struct BlockEntry {
	pub address: u64,
	pub width: i32,
	pub height: i32,
	pub edges: *const EdgeEntry,
	pub edge_count: u32,
}

#[repr(C)]
pub struct Point {
	pub x: f32,
	pub y: f32,
}

#[repr(C)]
pub struct EdgeData {
	pub polyline: *mut Point,
	pub point_count: u32,
}

#[repr(C)]
pub struct BlockData {
	pub address: u64,
	pub x: i32,
	pub y: i32,
	pub width: i32,
	pub height: i32,
	pub edges: *mut EdgeData,
	pub edge_count: u32,
}

#[repr(C)]
pub struct Layout {
	pub blocks: *mut BlockData,
	pub block_count: u32,
	pub width: i32,
	pub height: i32,
}

extern "C" {
	pub fn create_context() -> *mut GraphContext;
	pub fn delete_context(context: *mut GraphContext);
	pub fn add_block(ctx: *mut GraphContext, entry: *const BlockEntry);
	pub fn calculate_layout(ctx: *mut GraphContext, entry_point: u64) -> *mut Layout;
	pub fn delete_layout(layout: *mut Layout);
}
