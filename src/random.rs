extern {
    fn wasmblock_random_get_seed() -> f32;
}

#[inline]
pub fn get_seed() -> f32{
    unsafe {
        return wasmblock_random_get_seed();
    }
}
