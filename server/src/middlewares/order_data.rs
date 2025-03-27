use rand::rngs::OsRng;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
#[derive(Clone, Debug)]
pub struct OrderData {
    pub order_id: u32,
    pub quantity: i32,
    pub price: u32,
    pub seller_id: u32,
    pub pickup_point: Option<String>,
    pub delivery_point: Vec<u8>,
    pub delivery_partner_id: Option<u32>,
}

impl OrderData {
    pub fn new(
        order_id: u32,
        quantity: i32,
        price: u32,
        seller_id: u32,
        pickup_point: Option<String>,
        delivery_point: Vec<u8>,
        delivery_partner_id: Option<u32>,
        public_key: RsaPublicKey,
    ) -> Self {
        let mut rng = OsRng;
        let encrypted_delivery_point = public_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, &delivery_point[..])
            .expect("Delivery point encryption failed");
        OrderData {
            order_id,
            quantity,
            price,
            seller_id,
            pickup_point,
            delivery_point: encrypted_delivery_point,
            delivery_partner_id,
        }
    }

    pub fn decrypt_address(&mut self, private_key: &RsaPrivateKey) -> &mut Self {
        self.delivery_point = private_key
            .decrypt(Pkcs1v15Encrypt, &self.delivery_point)
            .expect("Delivery point decryption failed");
        self
    }
}

#[test]
fn test_encryption_decryption() {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    let address = b"home";
    let mut order_data = OrderData::new(123, 3, 124, 123, None, address.to_vec(), None, pub_key);
    let order_data_clone = order_data.clone();
    let decrypted_data = order_data.decrypt_address(&priv_key);
    assert_ne!(
        order_data_clone.delivery_point,
        decrypted_data.delivery_point
    );
}
