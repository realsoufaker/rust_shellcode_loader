mod loader;
mod description;

use clap::{Arg, Command};
use std::error::Error;
use loader::load;
use description::method;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Command::new("rust_shell_code")
        .version("1.0")
        .about("An example application for loading shellcode")
        .arg(
            Arg::new("list")
                .help("所有加载方式的列表")
                .short('l')
                .action(clap::ArgAction::SetTrue) // 设定为布尔标志
        )
        .subcommand(
            Command::new("shellcode")
                .about("Rust shellcode加载器")
                .arg(
                    Arg::new("bin_path")
                        .help("bin文件的路径，请使用绝对路径，基于转义的情况使用正斜杠，例如 D:/src/shell.bin")
                        .short('b')
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::new("method")
                        .help("使用哪种加载方式，例如asm")
                        .short('m')
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::new("file")
                        .help("输出的exe的名字，例如shellcode")
                        .short('f')
                        .required(true)
                        .takes_value(true)
                )        
        );
    let matches = cli.get_matches();

    match matches.subcommand() {
        Some(("shellcode", sub_matches)) => {
            if let Some(bin_path) = sub_matches.value_of("bin_path") {
                println!("Bin file location: {}", bin_path);
                if let Some(method) = sub_matches.value_of("method") {
                    match method {
                        "asm" => {
                            if let Some(file) = sub_matches.value_of("file") {
                                let _ = load(bin_path, file, method);
                                println!("ASM 方法已使用");
                            } else {
                                eprintln!("必须提供文件名");
                            }
                        },
                        "create_fiber" => {
                            if let Some(file) = sub_matches.value_of("file") {
                                let _ = load(bin_path, file, method);
                                println!("create_fiber 方法已使用");
                            } else {
                                eprintln!("必须提供文件名");
                            }
                        },
                        "create_process" => {
                            if let Some(file) = sub_matches.value_of("file") {
                                let _ = load(bin_path, file, method);
                                println!("create_process 方法已使用");
                            } else {
                                eprintln!("必须提供文件名");
                            }
                        },
                        "create_remote_thread" => {
                            if let Some(file) = sub_matches.value_of("file") {
                                let _ = load(bin_path, file, method);
                                println!("create_remote_thread 方法已使用");
                            } else {
                                eprintln!("必须提供文件名");
                            }
                        },
                        "create_remote_thread_native" => {
                            if let Some(file) = sub_matches.value_of("file") {
                                let _ = load(bin_path, file, method);
                                println!("create_remote_thread_native 方法已使用");
                            } else {
                                eprintln!("必须提供文件名");
                            }
                        },
                        "create_thread" => {
                            if let Some(file) = sub_matches.value_of("file") {
                                let _ = load(bin_path, file, method);
                                println!("create_thread 方法已使用");
                            } else {
                                eprintln!("必须提供文件名");
                            }
                        },
                        "create_thread_native" => {
                            if let Some(file) = sub_matches.value_of("file") {
                                let _ = load(bin_path, file, method);
                                println!("create_thread_native 方法已使用");
                            } else {
                                eprintln!("必须提供文件名");
                            }
                        },
                        "early_bird" => {
                            if let Some(file) = sub_matches.value_of("file") {
                                let _ = load(bin_path, file, method);
                                println!("early_bird 方法已使用");
                            } else {
                                eprintln!("必须提供文件名");
                            }
                        },
                        "etwp_create_etw_thread" => {
                            if let Some(file) = sub_matches.value_of("file") {
                                let _ = load(bin_path, file, method);
                                println!("etwp_create_etw_thread 方法已使用");
                            } else {
                                eprintln!("必须提供文件名");
                            }
                        },
                        "memmap2_transmute" => {
                            if let Some(file) = sub_matches.value_of("file") {
                                let _ = load(bin_path, file, method);
                                println!("memmap2_transmute 方法已使用");
                            } else {
                                eprintln!("必须提供文件名");
                            }
                        },
                        "module_stomping" => {
                            if let Some(file) = sub_matches.value_of("file") {
                                let _ = load(bin_path, file, method);
                                println!("module_stomping 方法已使用");
                            } else {
                                eprintln!("必须提供文件名");
                            }
                        },
                        "nt_queue_apc_thread_ex_local" => {
                            if let Some(file) = sub_matches.value_of("file") {
                                let _ = load(bin_path, file, method);
                                println!("nt_queue_apc_thread_ex_local 方法已使用");
                            } else {
                                eprintln!("必须提供文件名");
                            }
                        },
                        "rtl_create_user_thread" => {
                            if let Some(file) = sub_matches.value_of("file") {
                                let _ = load(bin_path, file, method);
                                println!("rtl_create_user_thread 方法已使用");
                            } else {
                                eprintln!("必须提供文件名");
                            }
                        },
                        _ => {
                            eprintln!("提供了无效的加载方式");
                        }
                    }
                }
            } else {
                eprintln!("必须提供文件路径");
            }
        },
        None if matches.get_flag("list") => {
            method();
        }
        _ => {
            eprintln!("没有提供有效的命令");
        }
    }

    Ok(())
}
