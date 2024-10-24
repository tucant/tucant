use indexed_db::Factory;
use js_sys::wasm_bindgen::JsValue;

pub struct Database {
    database: indexed_db::Database<std::io::Error>,
}

impl Database {
    pub async fn new() -> Self {
        let factory = Factory::<std::io::Error>::get().unwrap();

        let database = factory
            .open("database", 1, |evt| async move {
                let db = evt.database();
                db.build_object_store("store").create()?;
                Ok(())
            })
            .await
            .unwrap();

        Database { database }
    }

    pub async fn get<V: serde::de::DeserializeOwned>(&mut self, key: &str) -> Option<V> {
        let key = JsValue::from(key);
        let result = self
            .database
            .transaction(&["store"])
            .run(|t| async move {
                let store = t.object_store("store")?;
                let value = store.get(&key).await.unwrap();

                Ok(value.unwrap())
            })
            .await
            .unwrap();
        serde_wasm_bindgen::from_value(result).unwrap()
    }

    pub async fn put<V: serde::ser::Serialize + ?Sized>(&mut self, key: &str, value: &V) {
        let key = JsValue::from(key);
        let value = serde_wasm_bindgen::to_value(value).unwrap();
        self.database
            .transaction(&["store"])
            .rw()
            .run(|t| async move {
                let store = t.object_store("store")?;
                store.put_kv(&key, &value).await.unwrap();
                Ok(())
            })
            .await
            .unwrap();
    }
}
