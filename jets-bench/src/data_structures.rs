//! Data Structures used in the benchmarks for C compatibility
//!
//!
use bitcoin::secp256k1;
use elements::Txid;
use rand::{thread_rng, RngCore};
use simplicity::{
    bitcoin, elements,
    hashes::Hash,
    hex::FromHex,
    types::Final,
    BitIter, EarlyEndOfStreamError, Value,
};

/// Engine to compute SHA256 hash function.
/// We can't use hashes::sha256::HashEngine because it does not accept
/// bit level inputs. It also does not allow creation instances with arbitrary
/// length.
#[derive(Clone)]
pub struct SimplicityCtx8 {
    buffer: [u8; 64],
    h: [u32; 8],
    length: usize,
}

impl Default for SimplicityCtx8 {
    fn default() -> Self {
        SimplicityCtx8 {
            h: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
                0x5be0cd19,
            ],
            length: 0,
            buffer: [0; 64],
        }
    }
}

impl SimplicityCtx8 {
    pub fn with_len(n: usize) -> Self {
        SimplicityCtx8 {
            length: n,
            ..Default::default()
        }
    }
}

/// Read bytes from a Simplicity buffer of type (TWO^8)^<2^n as [`Value`].
/// The notation X^<2 is notation for the type (S X)
/// The notation X^<(2*n) is notation for the type S (X^n) * X^<n
///
/// There is a precondition that 0 <= n <= 16.
///
/// # Panics:
///
/// Panics if the length of the slice is >= 2^n bytes or if n > 16.
pub fn var_len_buf_from_slice(v: &[u8], n: usize) -> Result<Value, EarlyEndOfStreamError> {
    // This precondition can probably be removed or relaxed, though
    // we have no need to do so and it may require changing the type
    // of 'n' to something larger.
    assert!(n <= 16);
    assert!(v.len() < 1 << n);
    let mut iter = BitIter::new(v.iter().copied());
    let mut res = None;
    for i in (0..n).rev() {
        let ty = Final::two_two_n(i + 3);
        let v = if v.len() & 1 << i != 0 {
            let val = Value::from_compact_bits(&mut iter, &ty)?;
            Value::some(val)
        } else {
            Value::none(ty)
        };
        res = match res {
            Some(prod) => Some(Value::product(prod, v)),
            None => Some(v),
        };
    }
    assert_eq!(iter.next(), None);
    Ok(res.unwrap_or(Value::unit()))
}

// Field order: 2^256 - 2^32 - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1
fn out_of_field_order_point() -> [u8; 32] {
    let mut pt = [u64::MAX; 4];
    // The last u64 must be sampled between (2^32 + 2^9 + 2^8 + 2^7 + 2^6 + 2^4 + 1) to u64::MAX
    let offset: u64 = (1u64 << 32) + (1 << 9) + (1 << 8) + (1 << 7) + (1 << 6) + (1 << 4) + 1;
    // last u64 = u64::MAX - rand(0..offset)
    let sub_offset = rand::random::<u64>() % offset;
    pt[3] = u64::MAX - sub_offset;
    let mut bytes = [0u8; 32];
    // convert pt to bytes
    for i in 0..4 {
        bytes[8 * i] = (pt[i] >> 56) as u8;
        bytes[8 * i + 1] = (pt[i] >> 48) as u8;
        bytes[8 * i + 2] = (pt[i] >> 40) as u8;
        bytes[8 * i + 3] = (pt[i] >> 32) as u8;
        bytes[8 * i + 4] = (pt[i] >> 24) as u8;
        bytes[8 * i + 5] = (pt[i] >> 16) as u8;
        bytes[8 * i + 6] = (pt[i] >> 8) as u8;
        bytes[8 * i + 7] = pt[i] as u8;
    }
    bytes
}

/// A Fe point. This is the internal 32 byte array.
#[derive(Clone)]
pub enum SimplicityFe {
    /// A valid point
    RandomPoint([u8; 32]),
    /// Invalid point that is outside field order
    OutofOrderPoint([u8; 32]),
}

impl SimplicityFe {
    pub fn zero() -> Self {
        SimplicityFe::RandomPoint([0; 32])
    }

    pub fn as_inner(&self) -> &[u8; 32] {
        match self {
            SimplicityFe::RandomPoint(pt) => pt,
            SimplicityFe::OutofOrderPoint(pt) => pt,
        }
    }
}

/// Secp Ge point. Not necessarily on the curve
#[derive(Clone)]
pub enum SimplicityGe {
    /// A point on the curve
    ValidPoint(bitcoin::secp256k1::PublicKey),
    /// Invalid point (x, y)
    InvalidPoint(SimplicityFe, SimplicityFe),
}

/// Secp Gej point. Not necessarily on the curve
#[derive(Clone)]
pub struct SimplicityGej {
    /// The ge point (x, y). Could be valid or invalid
    pub ge: SimplicityGe,
    /// The z coordinate
    pub z: SimplicityFe,
}

/// A random secp scalar
#[derive(Clone)]
pub struct SimplicityScalar(pub [u8; 32]);

/// A secp point. Represented as (parity, x coordinate) in constrast to
/// (x, y) for `SimpliticyGe`.
#[derive(Clone)]
pub struct SimplicityPoint(pub bitcoin::secp256k1::PublicKey);

/// Trait defining how to encode a data structure into a Simplicity value
/// This is then used to write these vales into the bit machine.
pub trait SimplicityEncode {
    fn value(&self) -> Value;
}

impl SimplicityEncode for SimplicityCtx8 {
    fn value(&self) -> Value {
        let buf_len = self.length % 64;
        let buf = var_len_buf_from_slice(&self.buffer[..buf_len], 6).unwrap();
        let len = Value::u64(self.length as u64);
        // convert to 32 byte array
        let mut arr = [0u8; 32];
        for (i, byte) in self.h.iter().flat_map(|x| x.to_be_bytes()).enumerate() {
            arr[i] = byte;
        }
        let mid_state = Value::u256(arr);
        Value::product(buf, Value::product(len, mid_state))
    }
}

impl SimplicityEncode for elements::OutPoint {
    fn value(&self) -> Value {
        let txid = Value::u256(self.txid.to_byte_array());
        let vout = Value::u32(self.vout);
        Value::product(txid, vout)
    }
}

impl SimplicityEncode for elements::confidential::Asset {
    fn value(&self) -> Value {
        match self {
            elements::confidential::Asset::Explicit(a) => {
                let left = Final::product(Final::u1(), Final::u256());
                Value::right(left, Value::u256(a.into_inner().to_byte_array()))
            }
            elements::confidential::Asset::Confidential(gen) => {
                let ser = gen.serialize();
                let odd_gen = ser[0] & 1 == 1;
                let x_bytes = (&ser[1..33]).try_into().unwrap();
                let x_pt = Value::u256(x_bytes);
                let y_pt = Value::u1(odd_gen as u8);
                Value::left(Value::product(y_pt, x_pt), Final::u256())
            }
            elements::confidential::Asset::Null => panic!("Tried to encode Null asset"),
        }
    }
}

impl SimplicityEncode for elements::confidential::Value {
    fn value(&self) -> Value {
        match self {
            elements::confidential::Value::Explicit(v) => {
                let left = Final::product(Final::u1(), Final::u256());
                Value::right(left, Value::u64(*v))
            },
            elements::confidential::Value::Confidential(v) => {
                let ser = v.serialize();
                let x_bytes = (&ser[1..33]).try_into().unwrap();
                let x_pt = Value::u256(x_bytes);
                let y_pt = Value::u1((ser[0] & 1 == 1) as u8);
                Value::left(Value::product(y_pt, x_pt), Final::u64())
            }
            elements::confidential::Value::Null => panic!("Tried to encode Null value"),
        }
    }
}

impl SimplicityEncode for elements::confidential::Nonce {
    fn value(&self) -> Value {
        let ty_l = Final::product(Final::u1(), Final::u256());
        let ty_r = Final::u256();
        match self {
            elements::confidential::Nonce::Explicit(n) => {
                Value::some(Value::right(ty_l, Value::u256(*n)))
            }
            elements::confidential::Nonce::Confidential(n) => {
                let ser = n.serialize();
                let x_bytes = (&ser[1..33]).try_into().unwrap();
                let x_pt = Value::u256(x_bytes);
                let y_pt = Value::u1((ser[0] & 1 == 1) as u8);
                Value::some(Value::left(Value::product(y_pt, x_pt), ty_r))
            }
            elements::confidential::Nonce::Null => Value::none(Final::sum(ty_l, ty_r)),
        }
    }
}

impl SimplicityEncode for SimplicityFe {
    fn value(&self) -> Value {
        Value::u256(*self.as_inner())
    }
}

impl SimplicityEncode for SimplicityGe {
    fn value(&self) -> Value {
        let ser = match &self {
            SimplicityGe::ValidPoint(p) => p.serialize_uncompressed(),
            SimplicityGe::InvalidPoint(x, y) => {
                let mut ser = [0u8; 65];
                ser[0] = 4;
                ser[1..33].copy_from_slice(x.as_inner());
                ser[33..].copy_from_slice(y.as_inner());
                ser
            }
        };
        let x_bytes: [u8; 32] = (&ser[1..33]).try_into().unwrap();
        let y_bytes: [u8; 32] = (&ser[33..65]).try_into().unwrap();
        let x_pt = Value::u256(x_bytes);
        let y_pt = Value::u256(y_bytes);
        Value::product(x_pt, y_pt)
    }
}

impl SimplicityEncode for SimplicityGej {
    fn value(&self) -> Value {
        let ge = self.ge.value();
        let z = self.z.value();
        Value::product(ge, z)
    }
}

impl SimplicityEncode for SimplicityScalar {
    fn value(&self) -> Value {
        Value::u256(self.0)
    }
}

impl SimplicityEncode for SimplicityPoint {
    fn value(&self) -> Value {
        let ser = self.0.serialize(); // compressed
        let x_bytes = (&ser[1..33]).try_into().unwrap();
        let y_pt = Value::u1((ser[0] & 1 == 1) as u8);
        let x_pt = Value::u256(x_bytes);
        Value::product(y_pt, x_pt)
    }
}

pub trait BenchSample {
    fn sample() -> Self;
}

impl BenchSample for SimplicityCtx8 {
    fn sample() -> Self {
        let mut ctx = SimplicityCtx8::default();
        let mut rng = rand::thread_rng();
        // Sample a random length 32 bit
        let len = rng.next_u32() as usize;
        rng.fill_bytes(&mut ctx.buffer);
        for i in 0..8 {
            ctx.h[i] = rng.next_u32();
        }
        ctx.length = len;
        ctx
    }
}

impl BenchSample for elements::OutPoint {
    fn sample() -> Self {
        let mut rng = rand::thread_rng();
        let mut txid = [0u8; 32];
        rng.fill_bytes(&mut txid);
        let vout = rng.next_u32();
        elements::OutPoint {
            txid: Txid::from_byte_array(txid),
            vout,
        }
    }
}

impl BenchSample for elements::confidential::Asset {
    fn sample() -> Self {
        if rand::random() {
            let mut rng = rand::thread_rng();
            let mut asset = [0u8; 32];
            rng.fill_bytes(&mut asset);
            let asset = elements::AssetId::from_slice(&asset).unwrap();
            elements::confidential::Asset::Explicit(asset)
        } else {
            let asset_str = "0abb0d5d5843b9378c7f245fd7329d6fcef6926554f0c95f7cf0316239178f743c";
            let asset_bytes = Vec::<u8>::from_hex(asset_str).unwrap();
            elements::confidential::Asset::from_commitment(&asset_bytes).unwrap()
        }
    }
}

impl BenchSample for elements::confidential::Value {
    fn sample() -> Self {
        let mut rng = rand::thread_rng();
        if rand::random() {
            let value = rng.next_u64();
            elements::confidential::Value::Explicit(value)
        } else {
            let value_str = "09bb0d5d5843b9378c7f245fd7329d6fcef6926554f0c95f7cf0316239178f743c";
            let value_bytes = Vec::<u8>::from_hex(value_str).unwrap();
            elements::confidential::Value::from_commitment(&value_bytes).unwrap()
        }
    }
}

impl BenchSample for elements::confidential::Nonce {
    fn sample() -> Self {
        if rand::random() {
            elements::confidential::Nonce::Null
        } else {
            let nonce_str = "03bb0d5d5843b9378c7f245fd7329d6fcef6926554f0c95f7cf0316239178f743c";
            let nonce_bytes = Vec::<u8>::from_hex(nonce_str).unwrap();
            elements::confidential::Nonce::from_commitment(&nonce_bytes).unwrap()
        }
    }
}

impl BenchSample for SimplicityFe {
    fn sample() -> Self {
        if rand::random() {
            // Within field order with v high probability
            let v = rand::random::<[u8; 32]>();
            SimplicityFe::RandomPoint(v)
        } else {
            SimplicityFe::OutofOrderPoint(out_of_field_order_point())
        }
    }
}

impl BenchSample for SimplicityGe {
    fn sample() -> Self {
        if rand::random() {
            let sk = secp256k1::SecretKey::new(&mut thread_rng());
            let pk = secp256k1::PublicKey::from_secret_key(&secp256k1::Secp256k1::new(), &sk);
            SimplicityGe::ValidPoint(pk)
        } else {
            SimplicityGe::InvalidPoint(SimplicityFe::sample(), SimplicityFe::sample())
        }
    }
}

impl BenchSample for SimplicityGej {
    fn sample() -> Self {
        let ge = SimplicityGe::sample();
        let z = SimplicityFe::sample();
        SimplicityGej { ge, z }
    }
}

impl BenchSample for SimplicityScalar {
    fn sample() -> Self {
        let v = rand::random::<[u8; 32]>();
        // Within field order with v high probability
        SimplicityScalar(v)
    }
}

impl BenchSample for SimplicityPoint {
    fn sample() -> Self {
        let sk = secp256k1::SecretKey::new(&mut thread_rng());
        let pk = secp256k1::PublicKey::from_secret_key(&secp256k1::Secp256k1::new(), &sk);
        SimplicityPoint(pk)
    }
}

// Sample genesis pegin with 50% probability
pub fn genesis_pegin() -> Value {
    if rand::random() {
        Value::none(Final::two_two_n(8))
    } else {
        let genesis_hash = rand::random::<[u8; 32]>();
        Value::some(Value::u256(genesis_hash))
    }
}

impl SimplicityEncode for bitcoin::OutPoint {
    fn value(&self) -> Value {
        let txid = Value::u256(self.txid.to_byte_array());
        let vout = Value::u32(self.vout);
        Value::product(txid, vout)
    }
}

impl BenchSample for bitcoin::OutPoint {
    fn sample() -> Self {
        let mut rng = rand::thread_rng();
        let mut txid = [0u8; 32];
        rng.fill_bytes(&mut txid);
        let vout = rng.next_u32();
        bitcoin::OutPoint {
            txid: bitcoin::Txid::from_byte_array(txid),
            vout,
        }
    }
}
