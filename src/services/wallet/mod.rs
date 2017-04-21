mod default;

use self::default::DefaultWalletType;

use errors::wallet::WalletError;
use utils::environment::EnvironmentUtils;
use utils::json::{JsonEncodable, JsonDecodable};

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::fs::{File, DirBuilder};
use std::io::{Read, Write};
use std::path::PathBuf;

trait Wallet {
    fn set(&self, key: &str, value: &str) -> Result<(), WalletError>;
    fn get(&self, key: &str) -> Result<String, WalletError>;
}

trait WalletType {
    fn create(&self, name: &str, config: &str, credentials: &str) -> Result<(), WalletError>;
    fn delete(&self, name: &str) -> Result<(), WalletError>;
    fn open(&self, name: &str, credentials: &str) -> Result<Box<Wallet>, WalletError>;
}

#[derive(RustcDecodable, RustcEncodable)]
struct WalletDescriptor {
    pool_name: String,
    xtype: String,
    name: String
}

impl WalletDescriptor {
    pub fn new(pool_name: &str, xtype: &str, name: &str) -> WalletDescriptor {
        WalletDescriptor {
            pool_name: pool_name.to_string(),
            xtype: xtype.to_string(),
            name: name.to_string()
        }
    }
}

impl JsonEncodable for WalletDescriptor {}

impl JsonDecodable for WalletDescriptor {}

pub struct WalletService {
    types: RefCell<HashMap<&'static str, Box<WalletType>>>,
    wallets: RefCell<HashMap<i32, Box<Wallet>>>
}

impl WalletService {
    pub fn new() -> WalletService {
        let mut types: HashMap<&str, Box<WalletType>> = HashMap::new();
        types.insert("default", Box::new(DefaultWalletType::new()));

        WalletService {
            types: RefCell::new(types),
            wallets: RefCell::new(HashMap::new())
        }
    }

    pub fn resiter_type(xtype: &str,
                        create: fn(name: &str,
                                   config: &str,
                                   credentials: &str) -> Result<(), WalletError>,
                        open: fn(name: &str,
                                 credentials: &str) -> Result<i32, WalletError>,
                        set: extern fn(handle: i32,
                                       key: &str, sub_key: &str,
                                       value: &str) -> Result<(), WalletError>,
                        get: extern fn(handle: i32,
                                       key: &str, sub_key: &str) -> Result<(String, i32), WalletError>,
                        close: extern fn(handle: i32) -> Result<(), WalletError>,
                        delete: extern fn(name: &str) -> Result<(), WalletError>) {
        unimplemented!();
    }

    pub fn create(&self, pool_name: &str, xtype: &str, name: &str, config: &str, credentials: &str) -> Result<(), WalletError> {
        let wallet_types = self.types.borrow();
        if !wallet_types.contains_key(xtype) {
            return Err(WalletError::UnknownType(xtype.to_string()))
        }

        let wallet_path = _wallet_path(name);
        if wallet_path.exists() {
            return Err(WalletError::AlreadyExists(name.to_string()))
        }
        DirBuilder::new()
            .recursive(true)
            .create(wallet_path)?;

        let wallet_type = wallet_types.get(xtype).unwrap();
        wallet_type.create(name, config, credentials)?;

        let mut descriptor_file = File::create(_wallet_descriptor_path(name))?;
        descriptor_file
            .write_all(
                &WalletDescriptor::new(name, config, credentials)
                    .encode()
                    .unwrap() // TODO: FIXME: Provide error mapping!!!
                    .as_str()
                    .as_bytes()
            )?;
        descriptor_file.sync_all()?;

        let mut config_file = File::create(_wallet_config_path(name))?;
        config_file.write_all(config.as_bytes())?;
        config_file.sync_all()?;

        Ok(())
    }

    pub fn delete(&self, name: &str) -> Result<(), WalletError> {
        let desciptor = {
            let mut descriptor_json = String::new();
            WalletDescriptor::decode({
                let mut descriptor_file = File::open(_wallet_descriptor_path(name))?; // FIXME: Better error!
                descriptor_file.read_to_string(&mut descriptor_json)?;
                descriptor_json.as_str()
            }).unwrap() // FIXME: Provide type mapping
        };


        let wallet_types = self.types.borrow();
        if !wallet_types.contains_key(desciptor.xtype.as_str()) {
            return Err(WalletError::UnknownType(desciptor.xtype));
        }

        let wallet_type = wallet_types.get(desciptor.xtype.as_str()).unwrap();
        wallet_type.delete(name)?;

        fs::remove_dir_all(_wallet_path(name))?;
        Ok(())
    }

    pub fn open(name: &str, credentials: &str) -> Result<i32, WalletError> {
        unimplemented!()
    }

    pub fn close(handle: i32) -> Result<(), WalletError> {
        unimplemented!()
    }

    pub fn set(handle: i32, key: &str, sub_key: &str, value: &str) -> Result<i32, WalletError> {
        unimplemented!()
    }

    pub fn get(handle: i32, key: &str, sub_key: &str) -> Result<(String, i32), WalletError> {
        unimplemented!()
    }
}

fn _wallet_path(name: &str) -> PathBuf {
    EnvironmentUtils::wallet_path(name)
}

fn _wallet_descriptor_path(name: &str) -> PathBuf {
    _wallet_path(name).join("wallet.json")
}

fn _wallet_config_path(name: &str) -> PathBuf {
    _wallet_path(name).join("config.json")
}

//
//#[cfg(test)]
//mod tests {
//    use super::*;

//    #[test]
//    fn json_decode_works() {
//        let json = "{key1: \"value1\", key2: \"value2\"}";
//
//        json::decode(json).unwrap();
//    }
//}