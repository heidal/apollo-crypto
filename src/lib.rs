extern crate console_error_panic_hook;

mod utils;

use std::fmt;
use curve25519_dalek::constants;
use rand::Rng;

use wasm_bindgen::prelude::*;
use curve25519_dalek::scalar::Scalar;
use crate::utils::set_panic_hook;
use curve25519_dalek::edwards::{EdwardsPoint, CompressedEdwardsY};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


pub trait Base64Convertible {
    fn to_base64(&self) -> String;
    fn from_base64(data: &str) -> Self;
}


#[wasm_bindgen]
pub struct ElGamal {
    g: EdwardsPoint,
    h: EdwardsPoint,
    x: Scalar
}


impl Base64Convertible for EdwardsPoint {
    fn to_base64(&self) -> String {
        base64::encode(self.compress().to_bytes())
    }

    fn from_base64(data: &str) -> EdwardsPoint {
        let data = base64::decode(data).unwrap();
        let mut array = [0_u8; 32];
        array.copy_from_slice(&data);
        CompressedEdwardsY(array).decompress().unwrap()
    }
}


#[wasm_bindgen]
impl ElGamal {
    pub fn new() -> ElGamal {
        let mut csprng = rand::thread_rng();
        let x  = Scalar::random(&mut csprng);
        let g = constants::ED25519_BASEPOINT_POINT;
        let h = g * x;
        ElGamal {
            g,
            h,
            x
        }
    }

    pub fn encrypt(&self, message: bool) -> String {
        let mut csprng = rand::thread_rng();
        let randomness = Scalar::random(&mut csprng);
        let mut bytes = [0_u8; 32];
        bytes[0] = message.into();
        let message = CompressedEdwardsY::from_slice(&bytes).decompress().unwrap();
        let s = self.h * randomness;
        let c1 = self.g * randomness;
        let c2 = message + s;
        format!("{},{}", c1.to_base64(), c2.to_base64())
    }

    pub fn decrypt(&self, c1: String, c2: String) -> bool {
        let c1 = EdwardsPoint::from_base64(&c1);
        let c2 = EdwardsPoint::from_base64(&c2);
        let s: EdwardsPoint = c1 * self.x;
        let m: EdwardsPoint = c2 + (-s);
        m.compress().to_bytes()[0] == 1
    }
}
