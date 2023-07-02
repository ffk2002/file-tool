use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::Metadata;
use chrono::offset::Utc;
use chrono::DateTime;

struct Config {
    flag: String,
    target: String,
    params: Vec<String>,
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();
    // let cmd = &args[1];
    let configs = parse_args(&args);

    if configs.flag=="-i"{ //object info
        let _ = get_object_info(&configs.target);
    } else if configs.flag=="-f"{ //find in file
        let _ = find_in_file(&configs.target, &configs.params);
    }

}

fn parse_args(args: &Vec<String>) -> Config {
    let param_args : Vec<String>;
    if args.len() > 3{
        param_args = args[4..].to_owned();
    } else {
        param_args = Vec::new();
    }
    Config { flag: args[2].clone(), target: args[3].clone(), params: param_args}
}

fn get_object_info(target: &String) ->std::io::Result<()>{
    println!("\nget object info of file {:?}\n", target);
    let object_data : Metadata = fs::metadata("src/files/".to_owned()+target)?;
    let mut object_details : HashMap<String, String> = HashMap::new();
    let object_type: String;

    if object_data.is_file() {
        object_type = String::from("file");
    } else if object_data.is_dir() {
        object_type = String::from("directory");
    } else {
        object_type = String::from("symbolic link");
    };


    let create_time : DateTime<Utc> = object_data.created().unwrap().into();
    let access_time : DateTime<Utc> = object_data.accessed().unwrap().into();
    let mod_time : DateTime<Utc> = object_data.modified().unwrap().into();

    object_details.insert(String::from("object type:\t\t\t\t"), object_type);
    object_details.insert(String::from("object size:\t\t\t\t"), object_data.len().to_string().to_owned()+" bytes");
    object_details.insert(String::from("read only:\t\t\t\t"), object_data.permissions().readonly().to_string());
    object_details.insert(String::from("created date (UTC):\t\t\t"), create_time.format("%Y/%m/%d %T").to_string());
    object_details.insert(String::from("last modified (UTC):\t\t\t"), mod_time.format("%Y/%m/%d %T").to_string());
    object_details.insert(String::from("last accessed (UTC):\t\t\t"), access_time.format("%Y/%m/%d %T").to_string());

    for desc in object_details{
        println!("{} {}", desc.0, desc.1);
    }
    Ok(())
}

fn find_in_file(target: &String, params: &Vec<String>){
    println!("filename: {}", target);
    println!("find: {:?}", params);
}