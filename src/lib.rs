use std::error::Error;
use std::{env, fs};//读取文件



pub fn run(config: Config) ->Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).expect("something went wrong in the file"); //将file的内容读取成String expect错误显示
    // let contents = fs::read_to_string(config.file_path)?; 如果是对的，就把值给我；如果是错的，就立刻从当前函数返回这个错误
    let results=if !config.ignore_case{
        search(&config.query,&contents)
    }else{
        search_case_insensitive(&config.query,&contents)
    };
    for line in results{
        println!("{line}");
    }
    Ok(())
}

pub struct Config{
    pub query:String,
    pub file_path:String,
    pub ignore_case:bool,//是否忽略大小写
}
impl Config{//实现Config的方法
    pub fn build(args:&[String])->Result<Config,&'static str>{//返回result枚举
        if args.len()<3{
            return Err("not enough arguments")//return 提前跳出
        }
        let query = args[1].clone(); //0是exe 1是搜索的string 2是路径
        let file_path =args[2].clone();//这个Config 需要所有权 而 query 和file_path 都是借用 所以要clone
        let ignore_case=env::var("IGNORE_CASE").is_ok();//检查用户是否设置了名为 IGNORE_CASE 的环境变量，并根据结果返回一个布尔值
        Ok(Config{//最后Ok就是返回值
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{//有多个输入引用，且返回一个引用 要标注声明周期参数(返回跟谁相关) query是关键字 不需要标注 标注content 和返回Vec
    let mut results=Vec::new();//关键字可能很多行都有 创建一个集合
    for line in contents.lines(){//line 方法 把每一行返回回来
        if line.contains(query){
            results.push(line)
        }
    }
    results
}
pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{//无大小写的识别
    let query=query.to_lowercase();//把关键字变成小写的
    let mut results=Vec::new();//关键字可能很多行都有 创建一个集合
    for line in contents.lines(){//line 方法 把每一行返回回来
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn case_sensitive(){
        let query="duct";
        let content="\
Rust:
safe,fast,productive.
Pick three.";// \可以输出 输入的形式
        assert_eq!(vec!["safe,fast,productive."],search(query,content));
    }
    #[test]
    fn case_insensitive(){
        let query="rUst";
        let content="\
Rust:
safe,fast,productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:","Trust me."],search_case_insensitive(query,content));
    }
}