#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod utils;
mod data;
use crate::utils::{get_gacha_url_impl, set_window_shadow};
use fs2::FileExt;
use data::{Data, DataApp, DataCount};
use std::{fs::{self, File}, sync::Mutex, path::Path};
use substring::Substring;
use tauri::{
    CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use utils::{read_genshin_path, save_genshin_path_impl};
struct AppState {
    app: Mutex<DataApp>,
}

//启动游戏
#[tauri::command]
async fn start_genshin() -> Result<(), String> {
    let game_path_pre = read_genshin_path()?;
    let game_path = game_path_pre + "/YuanShen.exe";
    opener::open(game_path).map_err(|err| err.to_string())?;
    Ok(())
}

//保存原神路径
#[tauri::command]
async fn save_genshin_path() -> Result<(), String> {
    save_genshin_path_impl()?;
    Ok(())
}

#[tauri::command]
//检查原神路径
async fn check_genshin_path() -> Result<String, String> {
    let genshin_path = read_genshin_path()?;
    let paths = fs::read_dir(genshin_path.clone()).map_err(|err| err.to_string())?;
    let mut flag = true;
    for path in paths {
        let g_path = path.unwrap().path().to_str().unwrap().to_string();
        println!("{}", g_path);
        if g_path.contains("YuanShen.exe") {
            flag = false;
            break;
        }
    }
    if flag {
        //没找到
        return Err("没有获取正确的原神路径!".to_string());
    }
    let game_path = genshin_path + "/YuanShen.exe";
    Ok(game_path)
}

//获取卡池地址
#[tauri::command(async)]
fn get_gacha_url() -> Result<String, String> {
    let genshin_path = read_genshin_path()?;
    let url = get_gacha_url_impl(genshin_path)?;
    Ok(url)
}

#[tauri::command(async)]
fn get_data_list(state: State<AppState>) -> Result<Vec<Data>, String> {
    let app = state.app.lock().unwrap();
    let data = app.get_data()?;
    Ok(data)
}

#[tauri::command(async)]
fn get_data_uid(state: State<AppState>) -> Result<Vec<String>, String> {
    let app = state.app.lock().unwrap();
    let data = app.get_data_uid()?;
    Ok(data)
}

#[tauri::command(async)]
fn add_data_list(data_list: Vec<Data>, state: State<AppState>) -> Result<(), String> {
    let app = state.app.lock().unwrap();
    app.add_data_list(&data_list)?;
    Ok(())
}

#[tauri::command(async)]
fn get_count(gacha_type: String, rank_type: String, state: State<AppState>) -> Result<i32, String> {
    let app = state.app.lock().unwrap();
    let mut res = app.get_count(&gacha_type, &rank_type)?;
    if gacha_type == "301".to_string() {
        res += app.get_count(&"400".to_string(), &rank_type)?;
    }
    Ok(res)
}

#[tauri::command(async)]
fn drop_data(state: State<AppState>) -> Result<(), String> {
    let app = state.app.lock().unwrap();
    let res = app.drop_data()?;
    Ok(res)
}

#[tauri::command(async)]
fn get_count_rank_list(
    mut gacha_type: String,
    state: State<AppState>,
) -> Result<Vec<DataCount>, String> {
    let app = state.app.lock().unwrap();
    // let key = ;
    if gacha_type == String::from("301") {
        gacha_type = "301 or GACHA_TYPE = 400".to_string();
    }
    let res = app.get_data_gacha(&gacha_type)?;
    let mut count: i32 = 0;
    let mut data_list: Vec<DataCount> = Vec::new();
    for data in res.iter() {
        count += 1;
        if data.rank_type == "5".to_string() {
            let data_count = DataCount {
                name: data.name.clone(),
                time: data.time.clone(),
                count: count.clone(),
            };
            count = 0;
            data_list.push(data_count);
        }
    }
    Ok(data_list)
}

// 获取卡池时间范围
#[tauri::command(async)]
fn get_gacha_time(mut gacha_type: String, state: State<AppState>) -> Result<String, String> {
    let app = state.app.lock().unwrap();
    if gacha_type == String::from("301") {
        gacha_type = "301 or GACHA_TYPE = 400".to_string();
    }
    let query = app.get_data_gacha(&gacha_type)?;
    let mut start_time = "";
    let mut end_time = "";
    let mut count = true;
    if query.len() == 0 {
        return Err("找不到数据".to_string());
    }
    for data in query.iter() {
        if count {
            start_time = data.time.substring(0, 9);
        } else {
            end_time = data.time.substring(0, 9);
        }
        count = false;
    }
    let time_range: String = start_time.to_string() + " TO " + end_time;
    Ok(time_range)
}
fn main() {
    //
    if !Path::exists(Path::new("process.lock")) {
        File::create("process.lock").unwrap();
    }
    let lock = File::open("process.lock").unwrap();
    println!("准备加锁!");
    lock.lock_exclusive().unwrap();
    //
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let app = DataApp::new().unwrap();
    tauri::Builder::default()
        .setup(|app| {
            set_window_shadow(app);
            Ok(())
        })
        .system_tray(SystemTray::new().with_menu(tray_menu))
        // .plugin(tauri_plugin_sql::Builder::default().build())
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");
                let window = app.get_window("genshin-project").unwrap();
                window.show().unwrap();
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a double click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("genshin-project").unwrap();
                    window.hide().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            get_data_list,
            add_data_list,
            get_count,
            drop_data,
            get_count_rank_list,
            get_data_uid,
            get_gacha_url,
            get_gacha_time,
            save_genshin_path,
            check_genshin_path,
            start_genshin
        ])
        .manage(AppState {
            app: Mutex::from(app),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
