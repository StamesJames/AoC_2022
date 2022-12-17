use dotenv::dotenv;
use reqwest::{cookie::Jar, Url};
use std::io::prelude::*;
use std::{
    env,
    error::Error,
    fmt::Display,
    fs::{self, OpenOptions},
    path::PathBuf,
    sync::Arc,
};

pub type GenDynResult<R> = Result<R, Box<dyn std::error::Error>>;

pub fn fetch_res_and_save_to_file(day: &str) -> Result<(), Box<dyn std::error::Error>> {
    let day_num: usize = day.parse()?;
    if day_num > 0 && day_num <= 24 {
        let resp_text = fetch_res_cont(day)?;
        let day_str = if day_num < 10 {
            format!("0{day}")
        } else {
            day.to_string()
        };
        let day_str = format!("day_{}", day_str);
        let path_string = format!("./res/{}/", day_str);
        let mut path_buf = PathBuf::from(path_string);
        fs::create_dir_all(path_buf.as_path())?;
        path_buf.push(format!("{}.csv", day_str));
        fs::write(path_buf.as_path(), resp_text)?;
        path_buf.pop();
        path_buf.push(format!("{}_test.csv", day_str));
        fs::write(path_buf.as_path(), "")?;
        path_buf.pop();
        path_buf.pop();
        path_buf.pop();

        path_buf.push("Cargo.toml");
        {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(path_buf.as_path())?;
            let cont = format!(
                "[[bin]]\nname = \"{}\"\npath = \"./src/{}.rs\"",
                day_str, day_str
            );
            writeln!(file, "{}", cont)?;
        }
        path_buf.pop();

        path_buf.push("src");
        path_buf.push(format!("{}.rs", day_str));
        fs::write(path_buf.as_path(), format!("\nuse std::path::Path;\n\nfn main() -> Result<(), Box<dyn std::error::Error>> {{\n\tlet path = Path::new(r\"./res/{}/{}_test.csv\");\n\n\tOk(())\n}}\n", day_str, day_str))?;
        path_buf.pop();
        path_buf.push("aoc_lib");
        path_buf.push("mod.rs");
        {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(path_buf.as_path())?;
            let mod_line = format!("pub mod {};", day_str);
            writeln!(file, "{}", mod_line)?;
        }
        path_buf.pop();
        path_buf.push(format!("{}", day_str));
        fs::create_dir_all(path_buf.as_path())?;
        path_buf.push("mod.rs");
        fs::write(path_buf.as_path(), "")?;
    } else {
        return Err(Box::new(WrongDayNumberError {}));
    }

    Ok(())
}

pub fn fetch_res_cont(day: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/2022/day/{}/input", day.trim()).parse::<Url>()?;
    dotenv().ok();
    let jar = Jar::default();
    jar.add_cookie_str(&format!("session={}", env::var("SESSION")?), &url);
    let client = reqwest::blocking::ClientBuilder::new()
        .cookie_store(true)
        .cookie_provider(Arc::new(jar))
        .build()?;
    let resp = client.get(url).send()?;
    Ok(resp.text()?)
}

#[derive(Debug)]
pub struct WrongDayNumberError {}
impl Display for WrongDayNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "day must be between 1 and 24")
    }
}
impl Error for WrongDayNumberError {}


#[derive(Debug)]
pub struct EmptyOptionError;
impl Display for EmptyOptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Option was empty")
    }
}
impl Error for EmptyOptionError {}
