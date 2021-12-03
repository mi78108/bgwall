#[macro_use]
use lazy_static;
use std::{time::{Duration, SystemTime, UNIX_EPOCH}, thread, process::{Command, ExitStatus}, ops::Add, fs::{ReadDir, DirEntry}, os::unix::prelude::MetadataExt, path::{Path, PathBuf}, fmt::Error, collections::HashMap, cell::RefCell, sync::{Arc, RwLock}, rc::Rc, borrow::{BorrowMut, Borrow}};


fn download_image() -> String {
    let mut _d_img = IMG_DIR.to_string().add("/");
    if let Some(scripts) = Path::new(*FETCH_SCRIPT_DIR).get_exec_script() {
        let index = (|| {
            if let Ok(read) = VALS.read() {
                return read.get("FETCH_INDEX").unwrap().parse::<usize>().unwrap();
            };
            return 0;
        })();

        if let Ok(mut write) = VALS.write() {
            write.get_mut("FETCH_INDEX").unwrap().clear();
            write.get_mut("FETCH_INDEX").unwrap().push_str((index + 1).to_string().as_str());
        };

        if let Some(script) = scripts.get(index % scripts.len()) {
            let val = (|| {
                if let Ok(read) = VALS.read() {
                    let _val = read.get(script.file_name().to_str().unwrap()).unwrap_or("".to_string().borrow()).clone();
                    return (
                        read.get("FETCH_INDEX").unwrap().parse::<usize>().unwrap(),
                        read.get("FETCH_COUNT").unwrap().parse::<usize>().unwrap(),
                        Some(_val)
                    );
                };
                return (0, 0, None);
            })();
            _d_img.push_str(script.file_name().to_string_lossy().add("_").add(val.1.to_string().as_str()).borrow());
            format!("执行下载脚本 [{:?}] [{}] [{}]",script.file_name(),_d_img,val.0).as_str().to_log();
            if let Ok(mut child) = Command::new(script.path())
                .arg(_d_img.as_str())
                .arg(val.0.to_string()).spawn()
            {
                if let Ok(code) = child.wait() {
                    if code.success() {
                        format!("执行处理脚本 [{:?}] [{}] [{}] 成功; 脚本正常退出",script.file_name(),_d_img,val.0).as_str().to_log();
                        return _d_img;
                    }
                }
            }
            format!("执行下载脚本 [{:?}] [{}] [{}] 失败; 脚本异常退出",script.file_name(),_d_img,val.0).as_str().to_log();
        }
    }
    return _d_img;
}

fn process_image() -> String {
    let mut _c_img = IMG_DIR.to_string().add("/").add(C_BG);
    if let Some(scripts) = Path::new(*POST_SCRIPT_DIR).get_exec_script() {
        scripts.iter().enumerate().for_each(|(index, fd)| {
            let _n_img = IMG_DIR.to_string().add("/").add(index.to_string().as_str()).add("_").add(Path::new(_c_img.as_str()).file_name().unwrap().to_str().unwrap());
            format!("执行处理脚本 [{:?}] [{}] [{}]",fd.file_name(),_c_img,_n_img).as_str().to_log();
            if let Ok(mut child) = Command::new(fd.path())
                .arg(_c_img.as_str())
                .arg(_n_img.as_str()).spawn()
            {
                if let Ok(code) = child.wait() {
                    if code.success() {
                        _c_img.clear();
                        if Path::new(_n_img.as_str()).exists() {
                            _c_img.push_str(_n_img.as_str());
                            format!("执行处理脚本 [{:?}] [{}] [{}] 成功; 脚本正常退出; 输出文件存在",fd.file_name(),_c_img,_n_img).as_str().to_log();
                        } else {
                            _c_img.push_str(IMG_DIR.to_string().add("/").add(C_BG).as_str());
                            format!("执行处理脚本 [{:?}] [{}] [{}] 失败; 脚本正常退出; 输出文件不在",fd.file_name(),_c_img,_n_img).as_str().to_log();
                        }
                        return;
                    }
                }
            }
            format!("执行处理脚本 [{:?}] [{}] [{}] 失败; 脚本异常退出",fd.file_name(),_c_img,_n_img).as_str().to_log();
        });
    }
    return _c_img;
}

fn set_background(img_path: String) {
    if let Ok(mut child) = Command::new("feh").arg("--bg-scale").arg(img_path.as_str()).spawn() {
        if let Ok(state) = child.wait() {
            if state.success(){
                format!("设置壁纸 [{}] 成功",img_path).as_str().to_log();
                return;
            }
        }
    }
    format!("设置壁纸 [{}] 失败",img_path).as_str().to_log();
}

fn change_background() {
    set_background(IMG_DIR.to_string().add("/").add(N_BG));
    // process next image 
    thread::spawn(|| {
        let post_img = process_image();
        std::fs::copy(post_img.as_str(), IMG_DIR.to_string().add("/").add(N_BG)).expect("change image -> std::fs::copy erro");
        format!("图片处理完成 {} -> {}",post_img,IMG_DIR.to_string().add("/").add(N_BG)).as_str().to_log();
    });
}

static C_BG: &str = "bgwall_next";
static N_BG: &str = "bgwall_current";

lazy_static::lazy_static! {
    static ref IMG_DIR:&'static str = "/tmp/hawk";
    static ref FETCH_SCRIPT_DIR:&'static str = "/home/hawk/Sync/bin/bgwall/fetch_script";
    static ref POST_SCRIPT_DIR:&'static str = "/home/hawk/Sync/bin/bgwall/post_script";
    static ref DURA:u64 = 5 * 60;
    static ref VALS: RwLock<HashMap<String,String>> =  { 
        let mut mp = HashMap::new();
        mp.insert("FETCH_COUNT".to_string(), "0".to_string());
        mp.insert("FETCH_INDEX".to_string(), "0".to_string());
        return RwLock::new(mp);
    };
}


fn main() {
    let mut _download = std::thread::spawn(move || {
        loop {
            let download_img = download_image();
            if !Path::new(download_img.as_str()).exists() {
                format!("图片不存在 [{}]; 5 秒后重试 ",download_img).as_str().to_log();
                std::thread::sleep(Duration::from_secs(5));
                continue;
            }

            std::fs::copy(download_img.as_str(), IMG_DIR.to_string().add("/").add(C_BG)).expect("download image -> std::fs::copy erro");
            format!("图片下载完成 {} -> {}",download_img,IMG_DIR.to_string().add("/").add(C_BG)).as_str().to_log();
            std::thread::sleep(Duration::from_secs(*DURA));
        }
    });

    let mut _change = std::thread::spawn(|| {
        loop {
            if Path::new(IMG_DIR.to_string().add("/").add(C_BG).as_str()).exists() {
                change_background();
            } else {
                let download_img = download_image();
                set_background(download_img)
            }
            let _sec = SystemTime::now().get_current_sec() as u64;
            format!("图片更新完成 [{}]",IMG_DIR.to_string().add("/").add(C_BG)).as_str().to_log();
            std::thread::sleep(Duration::from_secs(60 - _sec));
        }
    });

    loop {
        std::thread::sleep(Duration::from_secs(60 ));
    }
}

trait TimeExt {
    fn get_current_sec(&self) -> u8;
    fn get_current_min(&self) -> u8;
}

trait PathExt {
    fn get_exec_script(&self) -> Option<Vec<DirEntry>>;
}

trait StrExt {
    fn to_log(&self);
}

impl StrExt for &str {
    fn to_log(&self) {
        let n = SystemTime::now();
        eprintln!("<{}:{}> [LOG]: {}", n.get_current_min(), n.get_current_sec(), self);
    }
}

impl PathExt for Path {
    fn get_exec_script(&self) -> Option<Vec<DirEntry>> {
        if let Ok(dir) = self.read_dir() {
            let mut scripts = dir.filter(|fs| {
                if let Ok(sct) = fs {
                    if sct.path().is_file() {
                        if let Ok(md) = sct.metadata() {
                            if (md.mode() & 0o111) != 0 {
                                return true;
                            }
                        }
                    }
                }
                return false;
            }).map(|v| v.unwrap()).collect::<Vec<DirEntry>>();

            scripts.sort_by(|v1, v2| {
                return v1.file_name().cmp(&v2.file_name());
            });
            return Some(scripts);
        }
        return None;
    }
}

impl TimeExt for SystemTime {
    fn get_current_sec(&self) -> u8 {
        let total_sec = self.duration_since(UNIX_EPOCH).unwrap().as_secs();
        return (total_sec % (60 * 24) % 60) as u8;
    }

    fn get_current_min(&self) -> u8 {
        let total_sec = self.duration_since(UNIX_EPOCH).unwrap().as_secs();
        return (total_sec % (60 * 24) % (60 * 60) / 60) as u8;
    }
}
