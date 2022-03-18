mod raw;

pub use raw::EdgeEntry;

pub struct Edge {
    polyline: Vec<(f32, f32)>,
}

impl Edge {
    pub fn points(&self) -> &[(f32, f32)] {
        &self.polyline
    }
}

pub struct Block {
    address: u64,
    position: (i32, i32),
    size: (i32, i32),
    edges: Vec<Edge>,
}

impl Block {
    fn from_raw(raw: *mut raw::BlockData) -> Self {
        unsafe {
            let mut ret = Self {
                address: 0,
                position: (0, 0),
                size: (0, 0),
                edges: Vec::with_capacity((*raw).edge_count as usize),
            };
            for i in 0..(*raw).edge_count {
                let mut edge = Edge { polyline: vec![] };
                let ptr = (*raw).edges.offset(i as isize);
                for i in 0..(*ptr).point_count {
                    let point = (*ptr).polyline.offset(i as isize);
                    edge.polyline.push(((*point).x, (*point).y));
                }
                ret.edges.push(edge);
            }
            ret.position = ((*raw).x, (*raw).y);
            ret.size = ((*raw).width, (*raw).height);
            ret.address = (*raw).address;
            ret
        }
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn address(&self) -> u64 {
        self.address
    }

    pub fn size(&self) -> (i32, i32) {
        self.size
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }
}

pub struct Graph {
    context: *mut raw::GraphContext,
}

pub struct Layout {
    blocks: Vec<Block>,
    size: (i32, i32),
}

impl Layout {
    fn from_raw(ctx: *mut raw::GraphContext, entry_point: u64) -> Self {
        unsafe {
            let l = raw::calculate_layout(ctx, entry_point);
            let mut ret = Self {
                blocks: Vec::with_capacity((*l).block_count as usize),
                size: (0, 0),
            };
            for i in 0..(*l).block_count {
                ret.blocks
                    .push(Block::from_raw((*l).blocks.offset(i as isize)));
            }
            ret.size = ((*l).width, (*l).height);
            raw::delete_layout(l);
            ret
        }
    }

    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    pub fn size(&self) -> (i32, i32) {
        self.size
    }
}

impl Graph {
    pub fn new() -> Self {
        unsafe {
            Self {
                context: raw::create_context(),
            }
        }
    }

    pub fn add_block(&mut self, address: u64, size: (i32, i32), edge_targets: &[EdgeEntry]) {
        unsafe {
            let raw_entry = raw::BlockEntry {
                address,
                width: size.0,
                height: size.1,
                edges: edge_targets.as_ptr(),
                edge_count: edge_targets.len() as u32,
            };

            raw::add_block(self.context, &raw_entry);
        }
    }

    pub fn layout(&self, entry_point: u64) -> Layout {
        Layout::from_raw(self.context, entry_point)
    }
}

impl Drop for Graph {
    fn drop(&mut self) {
        unsafe {
            raw::delete_context(self.context);
        }
    }
}
