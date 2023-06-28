#include <iostream>

typedef uint64_t u64;

class bitvec {
private:
    u64 size;
    u64* bits;
public:
// create uninitialized bitvec composed of 64-bit integers
    bitvec(u64 n) : size((n+63) / 64), bits(new u64[(n + 63)/64]) { 
    }
    ~bitvec() {
        delete[] bits;
    }
    void set(u64 i) {
        bits[i / 64] |= (1ULL << (i % 64));
    }
    bool get(u64 i) const {
        return bits[i / 64] & (1ULL << (i % 64));
    }
    bool clear(u64 i) {
        bits[i / 64] &= ~(1ULL << (i % 64));
    }
    u64& operator [](u64 i) {
        return bits[i];
    }
};


u64 eratosthenes(u32 n, bitvec& b) {
    for (u64 i = 0; i < b.size(); i++)
      b[i] = ~0ULL; // set all bits to zero 
    for (u64 i = 3; i <= n; i += 2)
      if (b.get(i)) {
        b.clear(i);
        for (u64 j = i*i; j <= n; j += 2 * i) {
            b.clear(j);
        }
      }
}