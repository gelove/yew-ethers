use ethers::core::rand::thread_rng;
use ethers::prelude::{k256::ecdsa::SigningKey, *};
use ethers::utils::keccak256;
use eyre::Result;
use hex;
use log::info;
use sha3::digest::Update;
use sha3::{Digest, Keccak256};
use std::env;
use std::str::FromStr;

/// create_wallet 生成随机数作为私钥创建钱包账号实例
pub async fn create_wallet() -> Wallet<SigningKey> {
    let wallet = LocalWallet::new(&mut thread_rng());
    info!("wallet: {:?}", wallet);
    wallet.with_chain_id(1u64)
}

/// create_provider 创建提供者
pub async fn create_provider(http_url: String) -> Result<Provider<Http>> {
    // Provider::<http::Provider> 实现了 try_from
    let provider = Provider::try_from(http_url)?;
    Ok(provider)
}

pub async fn get_signer_by_keystore() -> Result<()> {
    // let dir = "./keystore/key"; // keystore的钱包路径
    // let password = "123456";
    // let wallet =
    //     Wallet::<SigningKey>::decrypt_keystore(&dir, password).expect_throw("create wallet failed");
    dotenv::dotenv().ok();

    let private_key = env::var("PRIVATE_KEY")?;
    let wallet = private_key.parse::<LocalWallet>()?;
    let digest = md5::compute(b"\"hello2\"");
    let k256 = keccak256(&digest[0..8]).into();
    let mut sig = wallet.sign_hash(k256); // 里面有对recover_id加27操作
    sig.v = sig.v - 27; // to_eip155_v(sig.v as u8 - 27, 1);
    let signstr = sig.to_vec();
    info!("{:?} {:?}", k256, hex::encode(signstr));

    // 这里使用的 sign_hash 函数, ethers-rs 还提供了 sign_message 函数, 使用更简单
    let signature = wallet.sign_message("hello world").await.unwrap();
    signature.verify("hello world", wallet.address()).unwrap();
    Ok(())
}

pub async fn signs() -> Result<()> {
    let wallet = LocalWallet::new(&mut thread_rng());

    let message = "Some data";

    // sign a message
    let signature = wallet.sign_message(message).await?;
    println!("Produced signature {}", signature);

    // verify the signature
    signature.verify(message, wallet.address()).unwrap();

    println!("Verified signature produced by {:?}!", wallet.address());

    Ok(())
}

/// Used to allow a private key execute txs.
// pub async fn setup_signer(
//     priv_key: String,
//     provider: Provider<Http>,
// ) -> Option<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
//     let chain_id: u64 = provider.get_chainid().await.unwrap().as_u64();

//     let wallet: Result<Wallet<SigningKey>, WalletError> = priv_key.parse::<LocalWallet>();
//     match wallet {
//         Ok(x) => {
//             let w = x.with_chain_id(chain_id).clone();
//             return Some(SignerMiddleware::new(provider, w));
//         }
//         Err(_) => {
//             info!("Failed to connect to wallet.");
//             return None;
//         }
//     }
// }
pub async fn setup_signer(
    private_key: String,
    provider: Provider<Http>,
) -> Result<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    let chain_id: u64 = provider.get_chainid().await?.as_u64();
    let wallet: Wallet<SigningKey> = private_key.parse::<LocalWallet>()?;
    let wallet = wallet.with_chain_id(chain_id).clone();
    Ok(SignerMiddleware::new(provider, wallet))
}

/// Create a message, returning a signature for verification purposes.
///
/// Example:
///
/// The msg:
/// "DeGatchi#9032"
///
/// From the address:
/// 0xdcd49c36e69bf85fa9c5a25dea9455602c0b289e
///
/// Creates the signature:
/// 0x9d0a74e1c6cb58cc8f5695fb0c1b7f2fc9795c47c45e35f05919555b2f90fa655cde29bba787dcb5e544c3b0604ba4b82a71178dde9e5ac04f3f1db151d0a1941b
pub async fn create_msg(
    mw: SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    msg: &str,
) -> Option<(Signature, String)> {
    // sign message from your wallet and print out signature produced.
    match mw.signer().sign_message(msg).await {
        Ok(sig) => {
            info!("Produced signature {} with {}.", sig, mw.address());

            // verify the signature produced from your wallet.
            sig.verify(msg, mw.address()).unwrap();
            info!("Verified signature produced by {:?}!", mw.address());

            return Some((sig, msg.to_string()));
        }
        Err(_) => {
            return None;
        }
    }
}

/// Recovers pub key from Signature.
///
/// Example:
///
/// The signature:
/// r: 71031592387720320433450688414937280839659347503695324159450811751904079575653
/// s: 42005310148794597377403683159426268297577400373531751122007758979183741542804
/// v: 27
///
/// Gives the signer:
/// 0xdcd49c36e69bf85fa9c5a25dea9455602c0b289e
// pub fn verify_sign(signature: Signature, msg: &str) -> Option<H160> {
//     let r: Result<H160, SignatureError> = signature.recover(msg);
//     match r {
//         Ok(signer) => return Some(signer),
//         Err(_) => return None,
//     }
// }
pub fn verify_sign(signature: Signature, msg: &str) -> Result<H160> {
    let signer = signature.recover(msg)?;
    Ok(signer)
}

/// Converts signature string into [r, s, v] and recovers pub key.
///
/// Example:
///
/// The signature:
/// 0x9d0a74e1c6cb58cc8f5695fb0c1b7f2fc9795c47c45e35f05919555b2f90fa655cde29bba787dcb5e544c3b0604ba4b82a71178dde9e5ac04f3f1db151d0a1941b
///
/// Gives the signer:
/// 0xdcd49c36e69bf85fa9c5a25dea9455602c0b289e
// pub fn verify_sign_str(signature: &str, msg: &str) -> Option<H160> {
//     let sig: Signature = FromStr::from_str(signature).unwrap();
//     info!("{:?}", sig);
//     let r: Result<H160, SignatureError> = sig.recover(msg);
//     match r {
//         Ok(signer) => return Some(signer),
//         Err(_) => return None,
//     }
// }
pub fn verify_sign_str(signature: &str, msg: &str) -> Result<H160> {
    let sig: Signature = FromStr::from_str(signature)?;
    info!("{:?}", sig);
    let signer = sig.recover(msg)?;
    Ok(signer)
}

pub fn hex_to_u8(input: &str) -> Vec<u8> {
    return hex::decode(input).expect("Decoding failed");
}

// 创建密钥对
// use secp256k1::rand::{rngs, SeedableRng};
// use secp256k1::{PublicKey, Secp256k1, SecretKey};
// pub fn create_keypair() -> Result<(SecretKey, PublicKey)> {
//     let secp = Secp256k1::new();
//     let mut rng = rngs::StdRng::seed_from_u64(6);
//     Ok(secp.generate_keypair(&mut rng))
// }

/**
  Protect the integrity of the address against typing or reading mistakes.
  Important if you don't want to send funds to the wrong person ;)
  Ref: https://github.com/Ethereum/EIPs/blob/master/EIPS/eip-55.md
*/
pub fn verify(address: String) -> bool {
    // Reduce the address to just a lowercased 20-byte hex string
    let normalized_address = address.to_lowercase().replace("0x", "");
    let normalized_hash = hex::encode(
        Keccak256::new()
            .chain(normalized_address.as_bytes())
            .finalize(),
    );
    let mut checksummed_address = "".to_owned();
    let mut hash_iter = normalized_hash.chars();
    for hex_char in normalized_address.chars() {
        let hashed_character = hash_iter.next().unwrap();
        if "0123456789".contains(hex_char) {
            // Cannot uppercase a digit
            checksummed_address.push(hex_char);
        } else if "abcdef".contains(hex_char) {
            if hashed_character.to_digit(16).unwrap() > 7 {
                checksummed_address.push_str(&hex_char.to_uppercase().to_string());
            } else {
                checksummed_address.push(hex_char);
            }
        } else {
            panic!("Detected a non hex character in address {}.", address)
        }
    }
    checksummed_address = "0x".to_owned() + &checksummed_address;
    return checksummed_address.eq(&address);
}
