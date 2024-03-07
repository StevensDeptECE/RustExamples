#[no_mangle]
#[inline]
pub fn popcnt_u64(mut x: u64) -> u64 {
	use std::arch::asm;
	unsafe {
	    asm!(
	        "popcnt {x}, {x}",
					x = inout(reg) x,
	    );
	return x;
	}
}

// make every element of a the min, and b the max of the corresponding pair
fn sort2(mut a : u64, mut b : u64) {
   use std::arch::asm;
   unsafe {
     asm!(
         "vmovdqa {a}, {temp} // make a temp copy so we can wipe out ymm0
 	  vpminsd {a}, {b}, {a}	 # ymm0 now holds min values
	  vpmaxsd {a}, {b}, {b}"	 # ymm1 now holds max values
     );
}
// sort 8 32-bit values in 8 avx256 registers
pub fn sort16_32bit(p: *u64) {
	use std::arch::asm;
	unsafe {
	    asm!(
	    	"vmovdqa ({p}), %ymm0     # load a,b,c,d from memory
		 add     $32, {p}
		 vmovdqa ({p}), %ymm1
		 add     $32, {p}
		 vmovdqa ({p}), %ymm2
		 add     $32, {p}
		 vmovdqa ({p}), %ymm3
		 add     $32, {p}"
		 
/*
	//sort2(a,b)
	vmovdqa %ymm0, %ymm9		 # make a temp copy so we can wipe out ymm0
	vpminsd %ymm0, %ymm1, %ymm0	 # ymm0 now holds min values
	vpmaxsd %ymm9, %ymm1, %ymm1	 # ymm1 now holds max values

	//sort2(c,d)
	vmovdqa %ymm2, %ymm9           # make a temp copy so we can wipe out ymm2
	vpminsd %ymm2, %ymm3, %ymm2    # ymm2 now holds min values
	vpmaxsd %ymm9, %ymm3, %ymm3    # ymm3 now holds max values

	vmovdqa (%rdi), %ymm4
	add     $32, %rdi
	vmovdqa (%rdi), %ymm5
	add     $32, %rdi
	vmovdqa (%rdi), %ymm6
	add     $32, %rdi
	vmovdqa (%rdi), %ymm7
	sub     $224, %rdi
	
	//sort2(e,f)
	vmovdqa %ymm4, %ymm9           # make a temp copy so we can wipe out ymm4
	vpminsd %ymm4, %ymm5, %ymm4    # ymm4 now holds min values
	vpmaxsd %ymm9, %ymm5, %ymm5    # ymm5 now holds max values

	//sort2(g,h)
	vmovdqa %ymm6, %ymm9           # ymm9 = temp copy of g
	vpminsd %ymm6, %ymm7, %ymm6    # ymm6 = min(g,h)
	vpmaxsd %ymm9, %ymm7, %ymm7    # ymm7 = max(g,h)

	
	//sort2(a,c);
	vmovdqa %ymm0, %ymm9		 # make a temp copy so we can wipe out ymm0
	vpminsd %ymm0, %ymm2, %ymm0	 # ymm0 now holds min values
	vpmaxsd %ymm9, %ymm2, %ymm2	 # ymm2 now holds max values

	//sort2(b,d);
	vmovdqa %ymm1, %ymm9		 # make a temp copy so we can wipe out ymm0
	vpminsd %ymm1, %ymm3, %ymm1	 # ymm1 = min(b,d)
	vpmaxsd %ymm9, %ymm3, %ymm3	 # ymm3 = max(b,d)

	//sort2(e,g);
	vmovdqa %ymm4, %ymm9		 # ymm9 = temp copy of e
	vpminsd %ymm4, %ymm6, %ymm4	 # ymm4 = min(e,g)
	vpmaxsd %ymm9, %ymm6, %ymm6	 # ymm6 = max(e,g)
	
	//sort2(f,h);
	vmovdqa %ymm5, %ymm9		 # ymm9 = temp copy of f
	vpminsd %ymm5, %ymm7, %ymm5	 # ymm5 = min(f,h)
	vpmaxsd %ymm9, %ymm7, %ymm7	 # ymm7 = max(f,h)

	//sort2(b,c);
	vmovdqa %ymm1, %ymm9		 # ymm9 = temp copy of b
	vpminsd %ymm1, %ymm2, %ymm1	 # ymm1 = min(b,c)
	vpmaxsd %ymm9, %ymm2, %ymm2	 # ymm2 = max(b,c)

	//sort2(a,e);
	vmovdqa %ymm0, %ymm9		 # ymm9 = temp copy of a
	vpminsd %ymm0, %ymm4, %ymm0	 # ymm1 = min(a,e)
	vpmaxsd %ymm9, %ymm4, %ymm4	 # ymm4 = max(a,e)
	
	//sort2(f,g);
	vmovdqa %ymm5, %ymm9		 # ymm9 = temp copy of f
	vpminsd %ymm5, %ymm6, %ymm5	 # ymm5 = min(f,g)
	vpmaxsd %ymm9, %ymm6, %ymm6	 # ymm6 = max(f,g)
	
	//sort2(d,h);
	vmovdqa %ymm3, %ymm9		 # ymm9 = temp copy of d
	vpminsd %ymm3, %ymm7, %ymm3	 # ymm3 = min(d,h)
	vpmaxsd %ymm9, %ymm7, %ymm7	 # ymm7 = max(d,h)

	//sort2(b,f);
	vmovdqa %ymm1, %ymm9		 # ymm9 = temp copy of b
	vpminsd %ymm1, %ymm5, %ymm1	 # ymm1 = min(b,f)
	vpmaxsd %ymm9, %ymm5, %ymm5	 # ymm5 = max(b,f)
	
	//sort2(c,g);
	vmovdqa %ymm2, %ymm9		 # ymm9 = temp copy of c
	vpminsd %ymm2, %ymm6, %ymm2	 # ymm2 = min(c,g)
	vpmaxsd %ymm9, %ymm6, %ymm6	 # ymm6 = max(c,g)

	// sort2(b,e);
	vmovdqa %ymm1, %ymm9		 # ymm9 = temp copy of b
	vpminsd %ymm1, %ymm4, %ymm1	 # ymm1 = min(b,e)
	vpmaxsd %ymm9, %ymm4, %ymm4	 # ymm4 = max(b,e)

	// sort2(d,g);
	vmovdqa %ymm3, %ymm9		 # ymm9 = temp copy of d
	vpminsd %ymm3, %ymm6, %ymm3	 # ymm3 = min(d,g)
	vpmaxsd %ymm9, %ymm6, %ymm6	 # ymm6 = max(d,g)

	//sort2(c,e);
	vmovdqa %ymm2, %ymm9		 # ymm9 = temp copy of c
	vpminsd %ymm2, %ymm4, %ymm2	 # ymm2 = min(c,e)
	vpmaxsd %ymm9, %ymm4, %ymm4	 # ymm4 = max(c,e)

	//sort2(d,f);
	vmovdqa %ymm3, %ymm9		 # ymm9 = temp copy of d
	vpminsd %ymm3, %ymm5, %ymm3	 # ymm3 = min(d,f)
	vpmaxsd %ymm9, %ymm5, %ymm5	 # ymm5 = max(d,f)

	//sort2(d,e);
	vmovdqa %ymm3, %ymm9		 # ymm9 = temp copy of d
	vpminsd %ymm3, %ymm4, %ymm3	 # ymm3 = min(d,e)
	vpmaxsd %ymm9, %ymm4, %ymm4	 # ymm4 = max(d,e)

	vmovdqa %ymm0, (%rdi)            # store a,b,c,d
	add     $32,   %rdi
	vmovdqa %ymm1, (%rdi)
	add     $32,   %rdi	
	vmovdqa %ymm2, (%rdi)
	add     $32,   %rdi
	vmovdqa %ymm3, (%rdi)
	add     $32,   %rdi

	vmovdqa %ymm4, (%rdi)        # store e,f,g,h
	add     $32,   %rdi
	vmovdqa %ymm5, (%rdi)
	add     $32,   %rdi
	vmovdqa %ymm6, (%rdi)
	add     $32,   %rdi
	vmovdqa %ymm7, (%rdi)
	ret



	        "popcnt {x}, {x}",
					x = inout(reg) x,
	    );
	return x;
	}
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = popcnt_u64(2);
        assert_eq!(result, 1);
        let result = popcnt_u64(3);
        assert_eq!(result, 2);
        let result = popcnt_u64(u64::MAX);
        assert_eq!(result, 64);
    }
}fn main() {
    println!("Hello, world!");
}
