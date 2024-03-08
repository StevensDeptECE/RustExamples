enum Type {
    GisMap,    // map storing GIS shapes, like ESRI Shapefile but more efficient
    Dict,      // hashmap with string keys and template values
    GapMinder, // binary database of floating point variables from Gapminder
    U32Map,    // a hashmap with 32-bit int keys and templated values
    Webp,      // webp image header can be stored once for multiple identical images
} 

const MAGIC : u32 = 0x644C4221;

struct BlockLoader {
    magic : u32,         // magic number for all block loaders
    num_sections : u16,  // the number of sections, each requiring a header
    header_size : u16,   // number of 64-bit words used for all headers
    version : u16,       // version
    type_: Type,          // type of block loader
    author_id : u64,     // key to find author id on server
    doc_id : u64,        // unique id of document by author

    //    type : u16,          // type of block loader
    // static constexpr uint32_t bh = 0x644C4221;  // !BLd
    mem : Vec<u64>,       // 64-bit aligned memory
}


impl BlockLoader {
    fn new(type_ : Type, sz : u32) -> BlockLoader {
        BlockLoader {
            magic: MAGIC,
            num_sections: 1,
            header_size: 0,
            version: 0,
            type_,
            author_id: 0,
            doc_id: 0,
            mem: Vec::new(),
        }
    }
}

struct Node {
  offset : u32,
  next:    u32,  // relative pointer (offset into nodes)
  val:     u32,
}

impl Node {
    fn new(offset : u32, next: u32, val : u32) -> Node {
        Node {
            offset,
            next,
            val,
        }
    }
}



use std::{hash::Hash, mem::MaybeUninit};

struct BlockLoaderDict {
    mem: Vec<u8>,
    symbol_count : usize,    // number of symbols
    node_count : usize,      // number of nodes
    symbols: &[MaybeUninit<u8>],      // symbols
    nodes: &[MaybeUninit<Node>],      // nodes
    table: &[u32],       // table of nodes, 0 = empty
}

impl BlockLoaderDict {
    fn new(symbol_capacity : usize, node_capacity : usize, table_capacity : usize) -> BlockLoaderDict {
        use std::slice::from_raw_parts;
        use std::slice::from_raw_parts_mut;
        use std::mem::size_of;
        let symbol_capacity = (symbol_capacity + 7) / 8 * 8;

        let mut v: Vec<u8> = {
            let node_capacity_bytes  = node_capacity * size_of::<Node>(); 
            let table_capacity_bytes = table_capacity * size_of::<u32>();
            let total_capacity = symbol_capacity + node_capacity_bytes + table_capacity_bytes;
            Vec::with_capacity(size_of::<BlockLoader>() + total_capacity)
        };

        let symbols = unsafe { 
            let sym_ptr = v.as_ptr().add(size_of::<BlockLoader>()) as *const MaybeUninit<u8>;
            let sym_slice: &[MaybeUninit<u8>] = from_raw_parts(sym_ptr, symbol_capacity); 
            sym_slice
        };

        let nodes = unsafe { 
            let node_ptr = symbols.as_ptr().add(symbol_capacity) as *mut MaybeUninit<Node>;
            let node_slice: &mut [MaybeUninit<Node>] = from_raw_parts_mut(node_ptr, node_capacity);
            node_slice
        };

        let table: &[MaybeUninit<u32>] = unsafe {
            let table_ptr = nodes.as_ptr().add(node_capacity) as *const MaybeUninit<u32>;
            let table_slice: &[MaybeUninit<u32>] = from_raw_parts(table_ptr, table_capacity);
            table_slice
        };

        // set each table entry to 0
        for i in 0..table_capacity {
            table[i].write(0u32);
        }

        let table = unsafe { std::mem::transmute::<_, &[u32]>(table) };


        BlockLoaderDict {
            mem: v,
            symbol_count : 0,
            node_count : 1,
            symbols,
            nodes,
            table, 
        }
    }

    fn add_symbol(&mut self, sym : u8, val : u32) {
        use std::hash::{DefaultHasher, Hash, Hasher};

        let mut state = DefaultHasher::new();
        let ind = sym.hash(&mut state).finish() % self.table.len();
        self.symbols[self.symbol_count].write(sym);
        self.symbols[self.symbol_count].write('\0'.into());

        if self.table[ind] == 0 {
            self.table[ind] = self.node_count;
            self.nodes[self.node_count].write(Node::new(self.symbol_count as u32, 0, val));
            self.node_count += 1;
        } else {
            let old = self.table[ind];
            self.table[ind] = self.node_count;
            self.nodes[self.node_count].write(Node::new(self.symbol_count as u32, old, val));
            self.node_count += 1;
        }
        self.symbol_count += 2;
    }
}