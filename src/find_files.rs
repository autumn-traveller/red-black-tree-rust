use nix::sys::stat::SFlag;
use nix::sys::stat::lstat;
use nix::sys::stat::FileStat;

use std::fs;
use std::fmt::Write;

pub fn find_files(start: &str,fill : &mut String) -> std::io::Result<()> {

    let mut v = Vec::new();
    v.push(String::from(start));
    while let Some(e) = v.pop(){
        for entry in fs::read_dir(e)? {
    //for entry in fs::read_dir("/home/nic/")? {
            let dir = entry?;
            if let Some(s1)= dir.path().to_str() {
                let s1s = lstat(s1).unwrap();
                //println!("File {} is a {} {}",s1,if is_dir(s1s) {"directory"} else {"file"} ,if !is_dir(s1s) {String::from("of size ") + &s1s.st_size.to_string()} else {String::from("")});
                let mut s = String::new();
                write!(&mut s,"File {} is a {} {}\n",s1,if is_dir(s1s) {"directory"} else {"file"} ,if !is_dir(s1s) {String::from("of size ") + &s1s.st_size.to_string()} else {String::from("")});
                *fill+= &s;
                if is_dir(s1s) {
                    v.push(String::from(s1));
                }
            }
        }
    }

    Ok(())
}

fn is_dir(f: FileStat) -> bool {
    f.st_mode & SFlag::S_IFMT.bits() == SFlag::S_IFDIR.bits()
}
