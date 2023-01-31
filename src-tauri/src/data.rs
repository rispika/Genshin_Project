use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use tauri::api::path;
#[derive(Serialize, Deserialize)]
pub struct Data {
    pub uid: String,
    pub gacha_type: String,
    pub item_id: String,
    pub count: String,
    pub time: String,      //"2023-01-01 10:26:32",
    pub name: String,      //"罗莎莉亚",
    pub lang: String,      //"zh-cn",
    pub item_type: String, //"角色",
    pub rank_type: String, //"4",
    pub id: String,        //"1672538760006123626"
}
#[derive(Serialize, Deserialize)]
pub struct DataCount {
    pub name: String,
    pub time: String,
    pub count: i32,
}

pub struct DataApp {
    pub conn: Connection,
}

impl DataApp {
    pub fn new() -> Result<DataApp> {
        let data_path = path::local_data_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
            + "\\genshin_project";
        if Path::new(&data_path).exists() == false {
            fs::create_dir(&data_path).expect("该路径未找到!");
        }
        let db_path = data_path + "\\data.db";
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Data (
                uid         VARCHAR(64)       NOT NULL,
                gacha_type  VARCHAR(64)       NOT NULL,
                item_id     VARCHAR(64)       DEFAULT 0,
                count       VARCHAR(64)       DEFAULT 1,
                time        VARCHAR(64)       NOT NULL,
                name        VARCHAR(64)       NOT NULL,
                lang        VARCHAR(64)       NOT NULL,
                item_type   VARCHAR(64)       NOT NULL,
                rank_type   VARCHAR(64)       NOT NULL,
                id          VARCHAR(64)       PRIMARY KEY
            )",
            [],
        )?;
        Ok(DataApp { conn })
    }

    pub fn get_data(&self) -> Result<Vec<Data>,String> {
        let mut stmt = self.conn.prepare("SELECT * FROM Data").map_err(|err| err.to_string())?;
        let data_iter = stmt.query_map([], |row| {
            Ok(Data {
                uid: row.get(0)?,
                gacha_type: row.get(1)?,
                item_id: row.get(2)?,
                count: row.get(3)?,
                time: row.get(4)?,
                name: row.get(5)?,
                lang: row.get(6)?,
                item_type: row.get(7)?,
                rank_type: row.get(8)?,
                id: row.get(9)?,
            })
        }).map_err(|err| err.to_string())?;
        let mut data_list: Vec<Data> = Vec::new();
        for data in data_iter {
            data_list.push(data.map_err(|err| err.to_string())?);
        }
        Ok(data_list)
    }

    pub fn get_data_uid(&self) -> Result<Vec<String>,String> {
        let mut stmt = self.conn.prepare("SELECT * FROM Data GROUP BY uid").unwrap();
        let data_iter = stmt.query_map([], |row| {
            Ok(row.get(0)?)
        }).map_err(|err| err.to_string())?;
        let mut data_list: Vec<String> = Vec::new();
        for data in data_iter {
            data_list.push(data.map_err(|err| err.to_string())?);
        }
        Ok(data_list)
    }

    pub fn get_data_gacha(&self, gacha_type: &String, uid: &String) -> Result<Vec<Data>,String> {
        let sql = format!(
            "SELECT * FROM Data WHERE (GACHA_TYPE = {}) and uid = {} ORDER BY id",
            &gacha_type, &uid
        );
        let mut stmt = self.conn.prepare(&sql).map_err(|err| err.to_string())?;
        let data_iter = stmt.query_map([], |row| {
            Ok(Data {
                uid: row.get(0)?,
                gacha_type: row.get(1)?,
                item_id: row.get(2)?,
                count: row.get(3)?,
                time: row.get(4)?,
                name: row.get(5)?,
                lang: row.get(6)?,
                item_type: row.get(7)?,
                rank_type: row.get(8)?,
                id: row.get(9)?,
            })
        }).map_err(|err| err.to_string())?;
        let mut data_list: Vec<Data> = Vec::new();
        for data in data_iter {
            data_list.push(data.map_err(|err| err.to_string())?);
        }
        Ok(data_list)
    }

    pub fn add_data_list(&self, data_list: &Vec<Data>) -> Result<(),String> {
        let mut stmt = self
            .conn
            .prepare(
                "REPLACE INTO Data (
                    uid,
                    gacha_type,
                    item_id,
                    count,
                    time,
                    name,
                    lang,
                    item_type,
                    rank_type,
                    id
                ) VALUES (
                    ?1,
                    ?2,
                    ?3,
                    ?4,
                    ?5,
                    ?6,
                    ?7,
                    ?8,
                    ?9,
                    ?10
                )",
            )
            .map_err(|err| err.to_string()).map_err(|err| err.to_string())?;
        for data in data_list.iter() {
            stmt.execute((
                &data.uid,
                &data.gacha_type,
                &data.item_id,
                &data.count,
                &data.time,
                &data.name,
                &data.lang,
                &data.item_type,
                &data.rank_type,
                &data.id,
            ))
            .map_err(|err| err.to_string())?;
        }
        Ok(())
    }

    pub fn drop_data(&self) -> Result<(),String> {
        self.conn.execute("DELETE FROM Data", []).map_err(|err| err.to_string())?;
        Ok(())
    }

    pub fn get_count(&self, gacha_type: &String, rank_type: &String, uid: &String) -> Result<i32,String> {
        let sql = format!(
            "SELECT COUNT(*) FROM Data WHERE 
            gacha_type = {} and 
            rank_type = {} and uid = {}",
            gacha_type, rank_type, uid
        );
        let mut stmt = self.conn.prepare(&sql[..]).map_err(|err| err.to_string())?;
        let count = stmt.query_row((), |row| row.get(0)).map_err(|err| err.to_string())?;
        Ok(count)
    }


}