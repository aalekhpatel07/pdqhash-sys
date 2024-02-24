use std::ffi::CString;

use cxx::{let_cxx_string, vector::VectorElement};
use autocxx::{c_char16_t, prelude::*};
use pdqhash_sys::PDQHash;

// // pub mod io;

// use autocxx::prelude::*;


// include_cpp! {
//     #include "pdq/cpp/common/pdqbasetypes.h"
//     #include "pdq/cpp/common/pdqhashtypes.h"
//     #include "pdq/cpp/common/pdqhamming.h"
//     #include "pdq/cpp/hashing/pdqhashing.h"
//     #include "pdq/cpp/downscaling/downscaling.h"
//     #include "pdq/cpp/index/mih.h"
//     #include "pdq/cpp/io/hashio.h"


//     safety!(unsafe)
//     generate_ns!("facebook::pdq::hashing")
//     generate_ns!("facebook::pdq::downscaling")
//     generate_ns!("facebook::pdq::index")
//     generate_ns!("facebook::pdq::io")
// }


// pub use ffi::facebook::pdq::*;


// #[derive(Debug, Default)]
// pub struct PDQHash {
//     data: [u8; 32]
// }

// impl PDQHash {
//     // pub fn compute(image: &image::DynamicImage) -> PDQHash {
        
//     // }
// }


fn main() {
    let img = image::io::Reader::open("bridge.jpg").unwrap().decode().unwrap();

    let hash = PDQHash::compute_from_image(&img);
    println!("{:?}", hash);
    // let mut vector = cxx::CxxVector::new();

    // // let filename = std::path::PathBuf::from("pdqs.txt").to_string_lossy().to_string();
    // let cstr = CString::new("pdqs.txt".to_string()).unwrap();
    // let bytes = cstr.as_bytes_with_nul();
    // let ptr = bytes.as_ptr();
    
    // unsafe {
    //     io::loadHashesFromFile(ptr as *const i8, vector.pin_mut());
    // }
    // println!("{:?}", vector.len());
    // let s = "a5f0a457a48995e8c9065c275aaa5498b61ba4bdf8fcf80387c32f8b1bfc4f05";
    // let hash = s.parse::<PDQHash>().unwrap();
    // println!("{:?}", hash);
    // let_cxx_string!(s = );
    // let mut h = hashing::Hash256::new().within_box();
    
    // let mut b = hashing::Hash256::fromLineOrDie(s).within_box();


    // h.as_mut().setBit(autocxx::c_int(0));
    // println!("{:?}", h.getBit(autocxx::c_int(0)));
}