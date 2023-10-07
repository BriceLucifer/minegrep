// 大概的命令为
// cargo run -- searching file_path

//1.先读取命令参数

/*  main.rs 包括：解析命令行参数;
    初始化其他配置;
    调用lib.rs的run函数;
    如果run一个错误，需要处理错误;
*/

use std::env;
use std::process;

use minegrep::Config;

fn main(){

    let args: Vec<String> = env::args().collect();
    //通过env包然后使用args方法
    //dbg!(args);
    //let (query,file_path) = parse_config(&args);
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    /*
    println!("searching for {}",config.query);
    println!("In file {}",config.file_path);
    */
    if let Err(e) = minegrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}


