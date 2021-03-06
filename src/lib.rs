mod utils;

use doublets::doublets::ILinksExtensions;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn test_doublets(count: usize) -> usize {
    use doublets::mem::HeapMem;
    use doublets::doublets::mem::united::Links;

    let mem = HeapMem::new();
    let mut links = Links::<usize, _>::new(mem);

    let instant = instant::Instant::now();

    for _ in 0..count {
        links.create_point();
    }

    alert(&format!("rust performance: {:?}", instant.elapsed()));

    return links.count();
}