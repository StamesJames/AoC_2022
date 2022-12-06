use dotenv::dotenv;
use reqwest::{cookie::Jar, Url};
use std::{env, error::Error, fmt::Display, sync::Arc, fs::{File, self}, path::Path, io::BufWriter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];

    let day_num: usize = day.parse()?;
    if day_num > 0 && day_num <= 24 {
        let url = format!("https://adventofcode.com/2022/day/{}/input", day.trim()).parse::<Url>()?;
        dotenv().ok();
        let jar = Jar::default();
        jar.add_cookie_str(&format!("session={}",env::var("SESSION")?), &url);
        let client = reqwest::blocking::ClientBuilder::new().cookie_store(true).cookie_provider(Arc::new(jar)).build()?;
        let resp = client.get(url).send()?;
        // println!("{}", resp.text().unwrap());
        let day_str = if day_num < 10 {format!("0{day}")} else {day.to_string()};
        let path = format!("./res/day_{}/day_{}.csv", day_str, day_str);
        let path = Path::new(&path);
        fs::write(path, resp.text().unwrap())?;
        
    } else {
        return Err(Box::new(WrongDayNumberError {}));
    }

    Ok(())
}

#[derive(Debug)]
pub struct WrongDayNumberError {}
impl Display for WrongDayNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "day must be between 1 and 24")
    }
}
impl Error for WrongDayNumberError {}
