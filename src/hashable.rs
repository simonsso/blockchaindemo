pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;
    fn hash(&self) -> [u8; 32] {
        hmac_sha256::Hash::hash(&self.bytes())
    }
}
