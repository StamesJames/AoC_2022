use dotenv::dotenv;
use reqwest::{cookie::Jar, Url};
use std::{env, error::Error, fmt::Display, fs, path::PathBuf, sync::Arc};

pub fn fetch_res_and_save_to_file(day: &str) -> Result<(), Box<dyn std::error::Error>> {
    let day_num: usize = day.parse()?;
    if day_num > 0 && day_num <= 24 {
        let resp_text = fetch_res_cont(day)?;
        let day_str = if day_num < 10 {
            format!("0{day}")
        } else {
            day.to_string()
        };
        let path_string = format!("./res/day_{}/", day_str);
        let mut path_buf = PathBuf::from(path_string);
        fs::create_dir_all(path_buf.as_path())?;

        path_buf.push(format!("day_{}.csv", day_str));
        fs::write(path_buf.as_path(), resp_text)?;
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
