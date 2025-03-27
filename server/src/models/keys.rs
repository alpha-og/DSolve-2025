use rsa::{
    pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey},
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};

pub struct Keys {
    pub public_key: String,
    pub private_key: String,
}

impl Keys {
    pub async fn generate_keys(pool: sqlx::postgres::PgPool) -> Result<(), sqlx::Error> {
        let mut rng = rand::thread_rng(); // rand@0.8
        let bits = 2048;

        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);
        let priv_key_pem = priv_key.to_pkcs1_pem(rsa::pkcs1::LineEnding::LF).unwrap();
        let pub_key_pem = pub_key.to_pkcs1_pem(rsa::pkcs1::LineEnding::LF).unwrap();
        sqlx::query!(
            r#"
            INSERT INTO keys (public_key, private_key) 
            VALUES($1, $2)
            "#,
            &pub_key_pem.to_string(),
            &priv_key_pem.to_string()
        )
        .fetch_one(&pool)
        .await?;
        Ok(())
    }
}
