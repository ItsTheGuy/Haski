// Haski - Oscar
// The use of this is restricted to only the authors

pub mod file {
    pub mod hashPatterns {
        use sled::{open, Db, Result};
        pub struct ConfigRetrieve {
            pub CsT: Result<Option<sled::IVec>>,
            pub ClB: Result<Option<sled::IVec>>,
            pub ClF: Result<Option<sled::IVec>>,
            pub CpT: Result<Option<sled::IVec>>,
        }

        pub fn openDB(db: String) -> Result<Db> {
            open(db)
        }

        pub fn getPattern(db: &Db, hash: u64) -> Result<Option<sled::IVec>> {
            db.get(hash.to_string())
        }

        pub fn writePattern(
            db: &Db,
            hash: u64,
            action: &crate::trader::heart::Actions,
        ) -> Result<()> {
            let actionBytes: [u8; 1];
            match action {
                crate::trader::heart::Actions::Buy => actionBytes = (0x00 as u8).to_be_bytes(),
                crate::trader::heart::Actions::Sell => actionBytes = (0x01 as u8).to_be_bytes(),
                crate::trader::heart::Actions::Hold => actionBytes = (0x02 as u8).to_be_bytes(),
            }
            let _ = db.insert(hash.to_string(), &actionBytes)?;
            Ok(())
        }

        pub fn getConfig(db: &Db) -> ConfigRetrieve {
            ConfigRetrieve {
                CsT: (db.get("CsT")),
                ClB: (db.get("ClB")),
                ClF: (db.get("ClF")),
                CpT: (db.get("CpT")),
            }
        }

        pub fn writeConfig(
            db: &Db,
            lookBack: usize,
            lookForward: usize,
            patternThreshold: usize,
        ) -> Result<()> {
            let toInsert = crate::HashMap::from([
                ("ClB", lookBack.to_be_bytes()),
                ("ClF", lookForward.to_be_bytes()),
                ("CpT", patternThreshold.to_be_bytes()),
            ]);
            let _ = db.insert("CsT", &0x00_i32.to_be_bytes());
            for (key, value) in toInsert {
                db.insert(key, &value)?;
            }
            Ok(())
        }
    }
}
