use std::fs::{self,*};
use chrono::*;

pub fn create_datedfile(path:&str) -> File{
    let time_now = Local::now();
    let path = path.replace("$D",&format!("{}",time_now.format("%Y-%m-%d_%H-%M-%S")));
    let v:Vec<_> = path.match_indices(&['\\','/']).collect();
    let (dir,_file) = match v.last(){
        Some(x) => path.split_at(x.0),
        None => panic!{"Wrong path!!!"}
    };
    fs::create_dir_all(&dir).expect("cannot create directory");
    File::create(path).expect("cannot create file")
}

#[macro_export]
macro_rules! dout {
    ($file:expr;$($x:expr),*) => {{
        use std::io::*;
        let string:String = format!($($x),*);
        print!("{}",string);
        ($file).write_all(string.as_bytes()).unwrap();
    }}
}

#[macro_export]
macro_rules! doutln {
    ($file:expr;$($x:expr),*) => {{
        let string:String = format!($($x),*) + "\n";
        dout!($file;"{}",string)
    }}
}

pub fn serialize_string<T>(t:&T) -> String where T: serde::Serialize{
    bytes_to_string(&bincode::serialize(t).unwrap())
}

pub fn bytes_to_string(bytes:&[u8]) -> String{
    let temp:Vec<String> = bytes.iter().map(|i|{format!{"{}",i}}).collect();
    temp.join(",")
}

pub fn string_to_bytes(string:&str) -> Vec<u8>{
    let temp = string.split(",");
    temp.map(|o|{o.parse().unwrap()}).collect()
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use super::*;
    #[test]
    fn it_works() {
        let mut temp = File::create("./test.txt").unwrap();
        doutln!(temp;"{},{:>10},{}", 0, 0.0, "aiueo");
        doutln!(temp;"{},{:>10},{}", 0, 0.0, "aiueo");
    }
    #[test]
    fn bytes_test() {
        let string = bytes_to_string(&[8, 1, 0, 0, 0, 1, 0, 0, 0, 192, 63, 0, 1, 1, 6, 0, 0, 0, 0, 0, 0, 0]);
        println!("{}",string);
        let temp = string_to_bytes(&string);
        println!("{:?}",temp);
    }
}
