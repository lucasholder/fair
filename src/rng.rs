use hmac::{Hmac, Mac};
use sha2::Sha256;

use hmac::crypto_mac::generic_array::typenum::*;
use hmac::crypto_mac::generic_array::GenericArray;

use std::marker::PhantomData;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

pub struct ProvablyFairConfig {
    client_seed: String,
    server_seed: String,
    nonce: u64,
}

impl ProvablyFairConfig {
    pub fn new(client_seed: &str, server_seed: &str, nonce: u64) -> ProvablyFairConfig {
        ProvablyFairConfig {
            client_seed: client_seed.to_string(),
            server_seed: server_seed.to_string(),
            nonce,
        }
    }
}

pub struct ProvablyFairRNG<T> {
    config: ProvablyFairConfig,
    current_round: u64,
    current_round_cursor: usize,
    current_round_mac: Option<GenericArray<u8, U32>>,
    rng_type: PhantomData<T>,
}

///
/// ## Examples
///
/// For byte mode:
///
/// ```
/// use fair::ProvablyFairRNG;
/// let client_seed = "some client seed";
/// let server_seed = "some server seed";
/// let nonce = 1;
/// let mut rng: ProvablyFairRNG<u8> = ProvablyFairRNG::new(client_seed, server_seed, nonce);
/// // this is an inifinite iterator, it never returns None
/// println!("{}", rng.next().unwrap())
/// ```
///
/// For float mode:
///
/// ```
/// use fair::ProvablyFairRNG;
/// let client_seed = "some client seed";
/// let server_seed = "some server seed";
/// let nonce = 1;
/// let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::new(client_seed, server_seed, nonce);
/// // this is an inifinite iterator, it never returns None
/// println!("{}", rng.next().unwrap())
/// ```
///
///
impl<T> ProvablyFairRNG<T> {
    pub fn from_config(config: ProvablyFairConfig) -> ProvablyFairRNG<T> {
        ProvablyFairRNG {
            config,
            // TODO: group this under iter field?
            current_round: 0,
            current_round_cursor: 0,
            current_round_mac: None,
            rng_type: PhantomData,
        }
    }

    pub fn new(client_seed: &str, server_seed: &str, nonce: u64) -> ProvablyFairRNG<T> {
        let config = ProvablyFairConfig {
            client_seed: client_seed.to_string(),
            server_seed: server_seed.to_string(),
            nonce,
        };
        Self::from_config(config)
    }

    fn update_current_round_buffer(&mut self) {
        // Create HMAC-SHA256 instance which implements `Mac` trait
        let key = self.config.server_seed.as_bytes();
        let input = format!(
            "{}:{}:{}",
            self.config.client_seed, self.config.nonce, self.current_round
        );

        let mut mac =
            HmacSha256::new_varkey(key).expect("HMAC can take key of any size, never errors here");
        mac.input(input.as_bytes());
        let result = mac.result();
        self.current_round_mac = Some(result.code());
    }

    fn next_byte(&mut self) -> u8 {
        // 32 = number of bytes in self.current_round_buffer (aka size of hmac signature)
        // TODO: use sizeof pragma?
        let mac = match &self.current_round_mac {
            None => {
                self.update_current_round_buffer();
                return self.next_byte();
            }
            Some(v) => v,
        };

        let buf = mac;
        let result = buf[self.current_round_cursor];
        if self.current_round_cursor == 31 {
            self.current_round_cursor = 0;
            self.current_round += 1;
            self.current_round_mac = None;
        } else {
            self.current_round_cursor += 1;
        }
        return result;
    }

    fn next_float(&mut self) -> f64 {
        let bytes_per_float = 4;
        let bytes = &mut [0; 4];
        for i in 0..bytes_per_float {
            let byte = self.next_byte();
            bytes[i] = byte;
        }
        let result = bytes_to_float(bytes);
        return result;
    }
}

impl std::iter::Iterator for ProvablyFairRNG<f64> {
    type Item = f64;
    fn next(&mut self) -> Option<f64> {
        Some(self.next_float())
    }
}

impl std::iter::Iterator for ProvablyFairRNG<u8> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        Some(self.next_byte())
    }
}

// technique for converting groups of bytes into a float
fn bytes_to_float(bytes: &[u8]) -> f64 {
    let (float, _) = bytes.iter().fold((0., 0.), |(result, i), &value| {
        let value = value as f64;
        let divider = 256_f64.powf(i + 1.);
        let partial_result = value / divider as f64;
        (result + partial_result, i + 1.)
    });
    float
}

// TODO: use that function everywhere we are picking a number in a range
impl ProvablyFairRNG<f64> {
    // get a random number in [start, end[ range
    pub fn range(&mut self, start: usize, end: usize) -> usize {
        assert!(end > start);
        let range = (end as i32 - start as i32) as usize;
        (self.next().unwrap() * range as f64) as usize + start as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn provably_fair_rng() {
        let client_seed = "some client seed";
        let server_seed = "some server seed";
        let nonce = 1;
        let mut rng: ProvablyFairRNG<u8> = ProvablyFairRNG::new(client_seed, server_seed, nonce);
        let expected_values = vec![
            151, 136, 121, 135, 209, 159, 189, 233, 43, 248, 146, 253, 71, 34, 215, 176, 139, 160,
            47, 225, 233, 221, 169, 198, 187, 103, 171, 31, 87, 118, 23, 138, 198, 14, 60, 130,
            130, 198, 153, 83,
        ];
        for val in expected_values {
            assert_eq!(rng.next(), Some(val));
        }
    }

    #[test]
    fn provably_fair_rng_float() {
        let client_seed = "some client seed";
        let server_seed = "some server seed";
        let nonce = 1;
        let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::new(client_seed, server_seed, nonce);
        let expected_values = vec![
            0.5919261889066547,
            0.81884371698834,
            0.17176169087179005,
            0.277875404804945,
            0.5454130100551993,
            0.913538561668247,
            0.732050604885444,
            0.34164569014683366,
            0.7736547295935452,
            0.5108428790699691,
        ];
        for val in expected_values {
            let actual = rng.next();
            // println!("{} == {} ?", actual.unwrap(), val);
            assert_eq!(actual, Some(val));
        }
    }

    #[test]
    fn test_rng_float_starts_with_0() {
        let client_seed = "83e27f682128eb1852b048203dfd6931";
        let server_seed = "e8df2cc3b9ccb583ce5ea92336842387";
        let nonce = 1942124;
        let mut rng: ProvablyFairRNG<f64> = ProvablyFairRNG::new(client_seed, server_seed, nonce);
        assert_eq!(rng.next().unwrap(), 0.00000025122426450252533);
    }
}
