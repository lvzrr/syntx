use crossbeam::channel::unbounded;
use memmap2::Mmap;
use procfs::process::Process;
use std::env;
use std::fs::{File, create_dir_all};
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;
use syntx::codegen::codegen::*;
use syntx::engine::lexer::*;
use syntx::engine::semantic_traits::Walker;
use syntx::langs::java::tokenset::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        run_stdin();
        return;
    }

    let mode = &args[1];
    let filename = &args[2];

    match mode.as_str() {
        "--bench" => run_benchmark(filename),
        "--tokens" => print_tokens(filename),
        "--build" => generate_code(filename),
        _ => {
            eprintln!("Unknown mode: {}", mode);
            std::process::exit(1);
        }
    }
}

fn generate_code(filename: &str) {
    let f = File::open(filename).expect("Failed to open file");
    let mmap = unsafe { Mmap::map(&f).expect("Failed to mmap file") };
    let source_code = std::str::from_utf8(&mmap).expect("File is not valid UTF-8");
    create_dir_all(PathBuf::from("langs").join(PathBuf::from(filename.trim_end_matches(".stx"))))
        .unwrap();
    codegen(source_code);
}

fn run_benchmark(filename: &str) {
    let f = File::open(filename).expect("Failed to open file");
    let mmap = unsafe { Mmap::map(&f).expect("Failed to mmap file") };
    let source_code = std::str::from_utf8(&mmap).expect("File is not valid UTF-8");

    let source_for_lexer = source_code.to_owned();
    let pid = std::process::id() as i32;
    let start_mem = Process::new(pid).unwrap().statm().unwrap().resident;
    let page_size = procfs::page_size();

    let (sender, receiver) = crossbeam::channel::unbounded::<Vec<JavaToken>>();

    let start = Instant::now();

    std::thread::spawn(move || {
        let mut lexer = Lexer::<JavaTokenSet>::new(&source_for_lexer, sender);
        lexer.tokenize();
    });

    let mut token_count = 0usize;
    for batch in receiver.iter() {
        token_count += batch.len();
    }

    let elapsed = start.elapsed();
    let end_mem = Process::new(pid).unwrap().statm().unwrap().resident;
    let used_pages = end_mem.saturating_sub(start_mem);
    let used_mb = (used_pages * page_size as u64) as f64 / (1024.0 * 1024.0);
    let line_count = source_code.lines().count();

    println!("========== Benchmark Results for {} ==========", filename);
    println!("Lines        : {}", line_count);
    println!("Tokens       : {}", token_count);
    println!("Time         : {:.4} s", elapsed.as_secs_f64());
    println!("Memory       : {:.2} MB", used_mb);
    println!(
        "Lines/sec    : {}",
        (line_count as f64 / elapsed.as_secs_f64()) as usize
    );
    println!(
        "Tokens/sec   : {}",
        (token_count as f64 / elapsed.as_secs_f64()) as usize
    );
}

fn print_tokens(filename: &str) {
    let f = File::open(filename).expect("Failed to open file");
    let mmap = unsafe { Mmap::map(&f).expect("Failed to mmap file") };
    let source_code = std::str::from_utf8(&mmap).expect("File is not valid UTF-8");
    let (sender, receiver) = unbounded::<Vec<JavaToken>>();
    let source_code = source_code.to_owned();

    std::thread::spawn(move || {
        let mut lexer = Lexer::<JavaTokenSet>::new(&source_code, sender);
        lexer.tokenize();
    });

    for batch in receiver.iter() {
        for token in batch {
            eprintln!("{:?}", token);
        }
    }
}

fn run_stdin() {
    let mut src: String = String::new();
    std::io::stdin().read_to_string(&mut src).unwrap();
    let (sender, receiver) = unbounded::<Vec<JavaToken>>();
    std::thread::spawn(move || {
        let mut lexer = Lexer::<JavaTokenSet>::new(&src, sender);
        lexer.tokenize();
    });
    for batch in receiver.iter() {
        for token in batch {
            eprintln!("{:?}", token);
        }
    }
}
