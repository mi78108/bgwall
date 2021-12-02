use std::{time::{Duration, SystemTime, UNIX_EPOCH}, thread, process::{Command, ExitStatus}, ops::Add, fs::{ReadDir, DirEntry}, os::unix::prelude::MetadataExt, path::Path, fmt::Error};


fn download_image() -> String {
    eprintln!(">>> download background at {}:{}",SystemTime::now().get_current_min(),SystemTime::now().get_current_sec());
    return String::new()
}
fn process_image() -> String{
    let mut _c_img = imgDir.to_string().add("/").add(cBg);
    if let Ok(dir) = std::fs::read_dir(postScriptDir) {
        let mut scripts = dir.filter(|fs|{
            if let Ok(sct) = fs {
                if sct.path().is_file() {
                    if let Ok(md) = sct.metadata(){
                        if ( md.mode() & 0o111 ) != 0 {
                            return true
                        }
                    }
                }
            }
            return false
        }).map(|v| v.unwrap()).collect::<Vec<DirEntry>>();

        scripts.sort_by(|v1,v2|{
            return v1.file_name().cmp(&v2.file_name())
        });

        scripts.iter().enumerate().for_each(|(index,fs)|{
            let _n_img = imgDir.to_string().add("/").add(index.to_string().as_str()).add("_").add(Path::new(_c_img.as_str()).file_name().unwrap().to_str().unwrap());
            println!(">>> find script {:?} [{}] [{}]",fs,_c_img,_n_img );
            if let Ok(mut child) = Command::new(fs.path())
                .arg(_c_img.as_str())
                .arg(_n_img.as_str()).spawn() 
            {
                if let Ok(code) = child.wait(){
                    if code.success() {
                        _c_img.clear();
                        _c_img.push_str(_n_img.as_str());
                        eprintln!(">>> process background {:?} ok at {}:{}\n\n",fs.file_name(),SystemTime::now().get_current_min(),SystemTime::now().get_current_sec());
                        return;
                    }
                }
            }
            eprintln!(">>> process background {:?} erro at {}:{}\n\n",fs.file_name(),SystemTime::now().get_current_min(),SystemTime::now().get_current_sec());
        });
    }
    return _c_img
}

fn set_background(){
    if let Ok(child)= Command::new("feh").arg("--bg-scale").arg(imgDir.to_string().add("/").add(cBg)).spawn(){
        eprintln!(">>> set background at {}:{} ok\n\n",SystemTime::now().get_current_min(),SystemTime::now().get_current_sec());
    }else{
        eprintln!(">>> set background at {}:{} erro\n\n",SystemTime::now().get_current_min(),SystemTime::now().get_current_sec());
    }
}

fn change_background(){
    eprintln!(">>> change background at {}:{}\n\n",SystemTime::now().get_current_min(),SystemTime::now().get_current_sec());
    set_background();
    // process next image 
    thread::spawn(|| {
        process_image()
    });
}

static imgDir:&str = "/tmp/hawk";
static fetchScriptDir:&str = "/home/hawk/Sync/bin/bgwall/fetch_script";
static postScriptDir:&str = "/home/hawk/Sync/bin/bgwall/post_script";
static cBg:&str = "bgwall_next";
static nbg:&str = "bgwall_current";


fn main() {
    let duration = Duration::from_secs(60);

    let mut _download = std::thread::spawn(move ||{
        loop {
            download_image();
            std::thread::sleep(duration);
        }
    });

    let mut _change = std::thread::spawn(||{
        loop {
            change_background();
            let _sec =  SystemTime::now().get_current_sec() as u64;
            std::thread::sleep(Duration::from_secs(60 - _sec));
        }
    });

    loop {

    }
}

trait TimeExt {
    fn get_current_sec(&self) -> u8;
    fn get_current_min(&self) -> u8;
}

impl TimeExt for SystemTime {
    fn get_current_sec(&self) -> u8 {
        let total_sec = self.duration_since(UNIX_EPOCH).unwrap().as_secs();
        return  ( total_sec % ( 60 * 24 ) % 60 ) as u8;
    }

    fn get_current_min(&self) -> u8{
        let total_sec = self.duration_since(UNIX_EPOCH).unwrap().as_secs();
        return  ( total_sec % ( 60 * 24 ) % ( 60 * 60 ) / 60 ) as u8;
    }
}
