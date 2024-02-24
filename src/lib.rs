use std::fmt::Debug;

use cxx::let_cxx_string;

// pub mod io;
#[cfg(feature = "image")]
mod image_io;
use ffi::facebook::pdq::hashing::Hash256;
#[cfg(feature = "image")]
pub use image_io::*;

mod dct;


use autocxx::prelude::*;


include_cpp! {
    #include "pdq/cpp/common/pdqbasetypes.h"
    #include "pdq/cpp/common/pdqhashtypes.h"
    #include "pdq/cpp/common/pdqhamming.h"
    #include "pdq/cpp/hashing/pdqhashing.h"
    #include "pdq/cpp/hashing/torben.h"
    #include "pdq/cpp/downscaling/downscaling.h"
    #include "pdq/cpp/index/mih.h"
    #include "pdq/cpp/io/hashio.h"


    safety!(unsafe)
    generate_ns!("facebook::pdq::hashing")
    generate_ns!("facebook::pdq::downscaling")
    generate_ns!("facebook::pdq::index")
    generate_ns!("facebook::pdq::io")
}


pub use ffi::facebook::pdq::*;


#[derive(Debug, Default)]
pub struct PDQHash {
    data: [u8; 32],
    quality: f32
}



impl std::str::FromStr for PDQHash {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 64 {
            return Err("Expected hex encoded PDQ hash to be 64 chars long.".into());
        }
        if s.chars().into_iter().any(|c| !c.is_ascii_hexdigit()) {
            return Err("Expected hex encoded PDQ hash to be 64 chars long.".into());
        }
        let data = hex::decode(s).map_err(|err| {
            err.to_string()
        })?;
        Ok(Self { data: data.try_into().unwrap(), quality: 0.0 })
    }
}

impl PDQHash {
    pub fn data(&self) -> &[u8; 32] {
        &self.data
    }

    unsafe fn pdq_float256_from_float_luma(
        num_rows: usize,
        num_cols: usize,
        data: &mut [f32]
    ) -> Self {
        let buffer1 = data;
        let mut buffer2: Vec<f32> = vec![0.0.into(); buffer1.len()];

        let mut buffer64x64: [[f32; 64]; 64] = [[0.0; 64]; 64];
        // let mut quality = autocxx::c_int(0).within_box();

        if num_rows == 64 && num_cols == 64 {
            let mut k = 0;
            for i in 0..64 {
                for j in 0..64 {
                    buffer64x64[i][j] = buffer1[k];
                    k += 1;
                }
            }
        } else {
            let window_size_along_rows = downscaling::computeJaroszFilterWindowSize(autocxx::c_int(num_cols as i32), autocxx::c_int(64));
            let window_size_along_cols = downscaling::computeJaroszFilterWindowSize(autocxx::c_int(num_rows as i32), autocxx::c_int(64));

            unsafe {
                downscaling::jaroszFilterFloat(
                    buffer1.as_mut_ptr(),
                    buffer2.as_mut_ptr(),
                    autocxx::c_int(num_rows as i32), 
                    autocxx::c_int(num_cols as i32), 
                    window_size_along_rows, 
                    window_size_along_cols, 
                    autocxx::c_int(2)
                );

                downscaling::decimateFloat(
                    buffer1.as_mut_ptr(),
                    autocxx::c_int(num_rows as i32), 
                    autocxx::c_int(num_cols as i32), 
                    buffer64x64.as_mut_ptr() as *mut f32,
                    autocxx::c_int(64),
                    autocxx::c_int(64)
                );
            }
        }
    
        let mut buffer16x16 = Self::dct64_to_16(&buffer64x64);
        Self::pdq_buffer16x16_to_bits(&mut buffer16x16)

    }

    fn pdq_buffer16x16_to_bits(buffer: &mut [[f32; 16]; 16]) -> PDQHash {

        let median = unsafe {
            hashing::torben(buffer.as_mut_ptr() as *mut f32, autocxx::c_int(16 * 16))
        };

        // let mut hash = Hash256::new().within_box();
        let mut hash = [0u8; 32];
        
        for i in 0..16 {
            for j in 0..16 {
                if buffer[i][j] > median {
                    let idx = i * 16 + j;
                    let byte_idx = idx / 8;
                    let byte = &mut hash[byte_idx];
                    let idx_within_byte = idx % 8;
                    *byte |= 1 << idx_within_byte;
                }
            }
        }

        PDQHash {
            data: hash,
            quality: 0.
        }
    }

    /// Perform a discrete cosine transform from a 64x64 matrix and compute only a 16x16 corner of it. Quicker than computing the whole thing.
    fn dct64_to_16(
        input: &[[f32; 64]; 64],
    ) -> [[f32; 16]; 16] {
        let mut intermediate_matrix = [[0.0; 64]; 16];
        for i in 0..16 {
            for j in 0..64 {
                let mut sumk = 0.0;
                for k in 0..64 {
                    sumk += f32::from_bits(dct::DCT_MATRIX[i][k]) * input[k][j];
                }

                intermediate_matrix[i][j] = sumk;
            }
        }

        let mut output = [[0.0; 16]; 16];
        for i in 0..16 {
            for j in 0..16 {
                let mut sumk = 0.0;
                for k in 0..64 {
                    sumk += intermediate_matrix[i][k] * f32::from_bits(dct::DCT_MATRIX[j][k]);
                }
                output[i][j] = sumk;
            }
        }
        output
    }


    #[cfg(feature = "image")]
    pub fn compute_from_image(image: &image::DynamicImage) -> PDQHash {
        let (num_cols, num_rows, mut image) = to_luma_image(image);
        
        unsafe { Self::pdq_float256_from_float_luma(num_rows, num_cols, &mut image) }

    }
    // pub fn compute(image: &image::DynamicImage) -> PDQHash {
        
    // }
}