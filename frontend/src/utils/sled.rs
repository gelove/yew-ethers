use serde::{Serialize, Deserialize, de::DeserializeOwned};

use crate::R;

pub trait DbExenpt {
    fn get_string(&self, k: &str) -> Option<String>;
    fn get_string_or_default(&self, k: &str) -> String;
    fn insert_object<T: Serialize>(&self, k: &str, v: T) -> R;
    fn get_object<T: DeserializeOwned>(&self, k: &str) -> R<Option<T>>;
}

impl DbExenpt for sled::Db {
    fn get_string(&self, k: &str) -> Option<String> {
        if let Ok(Some(v)) = self.get(k) {
            return Some(String::from_utf8(v.to_vec()).unwrap());
        }
        None
    }

    fn get_string_or_default(&self, k: &str) -> String {
        if let Ok(Some(v)) = self.get(k) {
            return String::from_utf8(v.to_vec()).unwrap();
        }
        String::new()
    }

    fn insert_object<T: Serialize>(&self, k: &str, v: T) -> R{
        let v = json!(v).to_string();
        self.insert(k, v.as_bytes())?;
        Ok(())
    }

    fn get_object<T: DeserializeOwned>(&self, k: &str) -> R<Option<T>>{
        let v = self.get_string(k);
        match v{
            None => return Ok(None),
            Some(v) => {
                let t = serde_json::from_str::<T>(&v)?;
                return Ok(Some(t));
            }
        }
    }
}

pub fn user_path() -> String {


    let project_name = env!("CARGO_PKG_NAME").to_string();

    // 当前程序执行路径
    #[cfg(debug_assertions)]
    let dir = format!("../{project_name}-datadir");

    // 桌面路径
    #[cfg(not(debug_assertions))]
    let dir = {
        let dir = dirs::desktop_dir().unwrap_or_default();
        format!("{}\\{project_name}-datadir", dir.display())
    };

    return dir;
}
