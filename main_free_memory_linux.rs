// main.rs
//
// 2023-09-11
//
// test: OK
// environment: $ uname -a --> Linux ... 6.2.0-32-generic #32~22.04.1-Ubuntu SMP PREEMPT_DYNAMIC Fri Aug 18 10:40:13 UTC 2 x86_64 x86_64 x86_64 GNU/Linux:
//
//
// access data from:$ free -t  # <-- kB
//
// 
// Cargo.toml --> [dependencies]: subprocess = "0.2.9" <== will automatically install this library
// build:  $ ~/scripts/Rust/linux_memory$ cargo build
// run:    $ ~/scripts/Rust/linux_memory$ ./target/debug/linux_memory
//
// subprocess: Execution of child processes and pipelines, inspired by Python's subprocess module
// https://crates.io/crates/subprocess
//
//  

use subprocess::Exec;  // Exec::shell()

fn main() {
    let mem = {Exec::shell("free -t")}.capture().expect("nothing captured!").stdout_str();  // type: alloc::string::String
    
    println!("{}", &mem);
    /*
    $ ./target/debug/linux_memory
                   total        used        free      shared  buff/cache   available
    Mem:        16158284     1564216    10983260      441012     3610808    13807388
    Swap:        2097148           0     2097148
    Total:      18255432     1564216    13080408
    */
    
    let mem_v = mem.split_whitespace().collect::<Vec<_>>();
    
    // dbg!(&mem_v);
    /*
    [src/main.rs:47] &mem_v = [
        "total",
        "used",
        "free",
        "shared",
        "buff/cache",
        "available",
        "Mem:",
        "16158284",
        "1567400",
        "10953876", <<<<<<<<<<<<<<<<<<<<<<<<<<<
        "441012",
        "3637008",
        "13804204",
        "Swap:",
        "2097148",
        "0",
        "2097148",
        "Total:",
        "18255432",
        "1567400",
        "13051024",
    ]
    */
    
    let mem_free = mem_v[9];
    
    println!("mem_free [kilobytes] = {}", &mem_free);
}


// alternative: use std::process::Command;
//   https://stackoverflow.com/questions/21011330/how-do-i-invoke-a-system-command-and-capture-its-output


/* original Python solution:
import os

mem = os.popen('free -t').readlines()[1].split()[1:]
mem_free = mem[2]

print(mem_free)
*/
