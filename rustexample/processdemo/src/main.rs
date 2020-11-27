use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, prelude::*};
use std::path::Path;
use std::process::Command;

fn main() {
    processdemo();
    filedemo();

    let z = Complex { re: -1., im: 0. };
    let z_sqrt = unsafe { csqrtf(z) };

    println!("the square root of {:?} is {:?}", z, z_sqrt);
    println!("cos({:?}) ={:?}", z, cos(z));
}

#[link(name = "m")]
extern "C" {
    fn csqrtf(z: Complex) -> Complex;
    fn ccosf(z: Complex) -> Complex;
}
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}
#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}
impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
fn filedemo() {
    println!("mkdir a ");
    match fs::create_dir("a") {
        Err(e) => println!("!{:?}", e.kind()),
        Ok(_) => {}
    }
    println!("echo heelo >a/b.txt");
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|e| {
        println!("!{:?}", e.kind());
    });
    println!("nkdir -p a/c/d");
    fs::create_dir_all("a/b/d").unwrap_or_else(|e| {
        println!("!{:?}", e.kind());
    });
    println!("touch a/c/e.txt");
    touch(&Path::new("a/c/e.tx")).unwrap_or_else(|e| {
        println!("!touch {:?}", e.kind());
    });
    println!("`cat a/c/b.txt`");
    match cat(&Path::new("a/c/b.txt")) {
        Err(why) => println!("!cat {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }

    match fs::read_dir("a") {
        Err(e) => println!("!{:?}", e.kind()),
        Ok(paths) => {
            for path in paths {
                println!(">{:?}", path.unwrap().path());
            }
        }
    }
    fs::remove_file("a/c/e.txt").unwrap_or_else(|e| {
        println!("!remove_file {:?}", e.kind());
    });
    fs::remove_dir("a/b/d").unwrap_or_else(|e| {
        println!("!remove_dir {:?}", e.kind());
    });
}

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn processdemo() {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("failed to exec process:{}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("output is: \n{}\n", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("output is: \n{}\n", s);
    }
}
