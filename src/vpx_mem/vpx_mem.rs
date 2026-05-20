pub type size_t = usize;

pub const DEFAULT_ALIGNMENT: usize = 32;

pub struct AlignedBox {
    vec: Vec<u8>,
    offset: usize,
    size: usize,
}

impl AlignedBox {
    pub fn new(align: usize, size: usize) -> Option<Self> {
        let mut align = align;
        if align == 0 {
            align = DEFAULT_ALIGNMENT;
        }
        if !align.is_power_of_two() {
            align = align.next_power_of_two();
        }
        
        // Allocate extra space for alignment
        let mut vec = Vec::new();
        if vec.try_reserve(size + align).is_err() {
            return None;
        }
        vec.resize(size + align, 0u8);
        
        let raw_ptr = vec.as_ptr() as usize;
        let aligned_ptr = (raw_ptr + align - 1) & !(align - 1);
        let offset = aligned_ptr - raw_ptr;
        
        Some(Self { vec, offset, size })
    }

    pub fn as_ptr(&self) -> *mut u8 {
        let slice = self.as_slice();
        if slice.is_empty() {
            std::ptr::null_mut()
        } else {
            slice.as_ptr() as *mut u8
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.vec[self.offset .. self.offset + self.size]
    }

    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        &mut self.vec[self.offset .. self.offset + self.size]
    }
}
