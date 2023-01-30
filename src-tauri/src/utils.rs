// 窗口圆角
use regex::Regex;
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};
use tauri::{api::path, Manager, Runtime};
use window_shadows::set_shadow;
pub fn set_window_shadow<R: Runtime>(app: &tauri::App<R>) {
    let window = app.get_window("genshin-project").unwrap();

    set_shadow(&window, true).expect("Unsupported platform!");
}
//保存原神游戏路径
pub fn save_genshin_path() -> Result<(), String> {
    let genshin_output = path::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let genshin_output_cn =
        genshin_output.clone() + "\\AppData\\LocalLow\\miHoYo\\原神\\output_log.txt";
    let genshin_output_en =
        genshin_output.clone() + "\\AppData\\LocalLow\\miHoYo\\Genshin Impact\\output_log.txt";
    let mut genshin_output_file;
    if Path::exists(Path::new(&genshin_output_cn)) == true {
        genshin_output_file = File::open(genshin_output_cn).map_err(|err| err.to_string())?;
    } else {
        genshin_output_file = File::open(genshin_output_en).map_err(|err| err.to_string())?;
    }
    let mut output_context = String::new();
    genshin_output_file
        .read_to_string(&mut output_context)
        .map_err(|err| err.to_string())?;
    //正则
    let re1 = Regex::new(r"Warmup file (.+?)/StreamingAssets").map_err(|err| err.to_string())?;
    let mut genshin_path = String::new();
    for cap in re1.captures_iter(&output_context) {
        genshin_path = String::from(&cap[1]);
        break;
    }
    println!("genshin_path:{}", genshin_path);
    Ok(())
}
//读取原神游戏路径
pub fn read_genshin_path() -> Result<String, String> {
    let data_path = path::local_data_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
        + "\\genshin_project";
    if Path::new(&data_path).exists() == false {
        fs::create_dir(&data_path).expect("该路径未找到!");
    }
    let genshin_log_path_str = data_path + "\\path_log.txt";
    let genshin_log_path = Path::new(&genshin_log_path_str[..]);
    if !genshin_log_path.exists() {
        save_genshin_path();
    }
    let genshin_path_file = File::open(genshin_log_path_str).map_err(|err| err.to_string())?;
    let mut genshin_path_buf = String::new();
    genshin_path_file.read_to_string(&mut genshin_path_buf);
    Ok(genshin_path_buf)
}

// 获取卡池地址
pub fn get_gacha_url_impl(genshin_path: String) -> Result<String, String> {
    if genshin_path.is_empty() {
        return Ok("".to_string());
    }
    let log_path = genshin_path + "/webCaches/Cache/Cache_Data/data_2";
    let mut input = File::open(log_path).expect("file open wrong");
    // 转换成utf8
    let mut buf = vec![];
    input.read_to_end(&mut buf).map_err(|err| err.to_string())?;
    let contents = String::from_utf8_lossy(&buf);
    // 正则
    let re2 = Regex::new(r#"1/0/(https://hk4e-api.mihoyo.com/.+?)&gacha_type"#)
        .map_err(|err| err.to_string())?;
    let mut url_pre = String::new();
    for cap in re2.captures_iter(&contents) {
        url_pre = String::from(&cap[1]);
    }
    println!("url_pre:{}", url_pre);
    Ok(url_pre)
}
