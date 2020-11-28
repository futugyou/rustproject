use anyhow::{Context, Result};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    pattern: String,
    //#[structopt(short = "o", long = "output")]
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let args = Cli {
        pattern,
        path: std::path::PathBuf::from(path),
    };
    println!("args:{:?} ", args);
    let args = Cli::from_args();
    println!("args:{:?} ", args);
    let content =
        std::fs::read_to_string(&args.path).with_context(|| format!("could not read file ",))?;
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    let pb = indicatif::ProgressBar::new(10);
    for i in 0..10 {
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
    Ok(())
}
