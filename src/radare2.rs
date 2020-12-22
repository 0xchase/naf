use serde_json;

use r2pipe::R2Pipe;
use r2pipe::R2PipeSpawnOptions;

fn test_trim() {
    let mut ns = R2Pipe::spawn("/bin/ls".to_owned(), None).unwrap();
    println!("(({}))", ns.cmd("\n\n?e hello world\n\n").unwrap());
    println!("(({}))", ns.cmd("\n\n?e hello world\n\n").unwrap());
    println!("(({}))", ns.cmd("\n\n?e hello world\n\n").unwrap());
    ns.close();
    //    process::exit(0);
}

pub struct R2Debug {
    r2p: r2pipe::R2Pipe
}

impl R2Debug {
    pub fn new() -> Self {
        return R2Debug {
            r2p: match R2Pipe::in_session() {
                Some(_) => R2Pipe::open().expect("Failed to open r2pipe"),
                None => R2Pipe::spawn("/home/oem/github/ninja-analysis-framework/binaries/lockpicksim".to_owned(), 
                    Some(R2PipeSpawnOptions {
                        exepath: "radare2".to_owned(),
                        ..Default::default()
                    })
                ).expect("Failed to start r2pipe"),
            }
        };
    }

    pub fn cmd(&mut self, command: &str) {
        info!("{}", self.r2p.cmd(command).unwrap());
    }
}

pub fn test() {
    //test_trim();

    // let mut r2p = open_pipe!().unwrap();
    let opts = R2PipeSpawnOptions {
        exepath: "radare2".to_owned(),
        ..Default::default()
    };

    let mut r2p = match R2Pipe::in_session() {
        Some(_) => R2Pipe::open(),
        None => R2Pipe::spawn("/home/oem/github/ninja-analysis-framework/binaries/lockpicksim".to_owned(), Some(opts)),
    }
    .unwrap();

    println!("{}", r2p.cmd("ood").unwrap());
    println!("{}", r2p.cmd("dcu main").unwrap());
/*
    let json = r2p.cmdj("ij").unwrap();
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
    println!("ARCH {}", json["bin"]["arch"]);
    println!("BITS {}", json["bin"]["bits"]);
    println!("Disasm:\n{}", r2p.cmd("pd 20").unwrap());
    println!("Hexdump:\n{}", r2p.cmd("px 64").unwrap());
    r2p.close();
    */
}
