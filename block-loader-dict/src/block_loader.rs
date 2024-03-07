enum BlockLoaderType {
    gismap,    // map storing GIS shapes, like ESRI Shapefile but more efficient
    dict,      // hashmap with string keys and template values
    gapminder, // binary database of floating point variables from Gapminder
    i32map,    // a hashmap with 32-bit int keys and templated values
    webp,      // webp image header can be stored once for multiple identical images
  };

struct BlockLoader {
    magic : u32,         // magic number for all block loaders
    num_sections : u16;  // the number of sections, each requiring a header
    header_size : u16;   // number of 64-bit words used for all headers
    version : u16,       // version
    author_id : u64,     // key to find author id on server
    doc_id : u64,        // unique id of document by author

//    type : u16,          // type of block loader
    static constexpr uint32_t bh = 0x644C4221;  // !BLd
 
    mem : Vec<u64>,       // 64-bit aligned memory
}

  // std::unique_ptr<uint64_t> mem;
  uint64_t* mem;
  uint64_t size;
  struct GeneralHeader {
    GeneralHeader(Type type, uint16_t version)


struct BlockLoaderDict {
    b : BlockLoader,      // block loader common header 
    symbolCapacity : u32, // number of elements holding symbols
    tableCapacity : u32,  // number of elements holding table
    nodeCapacity : u32,   // number of elements holding nodes
    nodeCount : u32,      // number of nodes
    symbolCount : u32,    // number of bytes used by symbols

    struct Node {
        uint32_t offset;
        uint32_t next;  // relative pointer (offset into nodes)
        Val val;
        Node() {
        }  // this is so the empty block can be initialized without doing anything
        Node(uint32_t offset, uint32_t next, Val v)
            : offset(offset), next(next), val(v) {}
      };
    
}