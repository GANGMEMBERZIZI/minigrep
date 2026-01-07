use std::env;//读取命令行参数
use std::process;
use minigrep::Config;
//不产生panic

fn main() {
    let args:Vec<String>=env::args().collect();
    // dbg!(&args);会显示路径和内容，会打印到stderr
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("problem parsing arguments {}",err);//打印错误信息 stderr错误信息即使 >output.txt 也显示终端上 用于区分信息
        process::exit(1);//停止程序 1是错误退出

    });
    println!("searching {}",config.query);
    println!("in file {}",config.file_path);
    if let Err(e)=minigrep::run(config){
        eprintln!("{}",e);//输出给stderr
        process::exit(1);
    }
}



