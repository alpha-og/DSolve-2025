use base64::Engine; // bring Engine trait into scope

#[derive(Clone)]
pub struct JWTKeyProvider {
    pub key_pair: std::sync::Arc<jwt_simple::prelude::ES256KeyPair>,
}
impl JWTKeyProvider {
    pub fn new(jwt_base64: &str) -> Self {
        Self {
            key_pair: std::sync::Arc::new(
                jwt_simple::prelude::ES256KeyPair::from_bytes(
                    &base64::engine::general_purpose::STANDARD
                        .decode(jwt_base64)
                        .unwrap(),
                )
                .unwrap(),
            ),
        }
    }
}
