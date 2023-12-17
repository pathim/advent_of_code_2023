use chrono::TimeZone;
use ex::Wrapper;
use std::io::BufRead;

#[derive(Debug)]
pub enum Error {
    Http(http_req::error::Error),
    Download(http_req::response::StatusCode),
    Io(ex::io::Error),
}

impl From<http_req::error::Error> for Error {
    fn from(e: http_req::error::Error) -> Self {
        Self::Http(e)
    }
}
impl From<ex::io::Error> for Error {
    fn from(e: ex::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<http_req::response::StatusCode> for Error {
    fn from(e: http_req::response::StatusCode) -> Self {
        Self::Download(e)
    }
}
fn generate_path(year: i32, day: u32) -> std::path::PathBuf {
    ["input", &year.to_string(), &(day.to_string() + ".txt")]
        .iter()
        .collect()
}
fn download_input(year: i32, day: u32) -> Result<(), Error> {
    let session = ex::fs::read_to_string("cookie")?;
    let cookie = format!("session={}", session);
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let url_str: &str = &url;
    let uri = http_req::uri::Uri::try_from(url_str).unwrap();

    let path = generate_path(year, day);
    ex::fs::create_dir_all(path.parent().unwrap())?;
    let mut file = ex::fs::File::create(&path)?;
    let resp = http_req::request::Request::new(&uri)
        .header("Cookie", &cookie)
        .send(&mut file)?;
    let status = resp.status_code();
    if !status.is_success() {
        drop(file);
        ex::fs::remove_file(&path)?;
        return Err(status.into());
    }
    Ok(())
}
fn get_input(year: i32, day: u32) -> Result<std::fs::File, Error> {
    let path = generate_path(year, day);
    let file_in = ex::fs::File::open(&path);
    if file_in.is_ok() {
        Ok(file_in?.into_inner())
    } else {
        download_input(year, day)?;
        Ok(ex::fs::File::open(path)?.into_inner())
    }
}

pub fn get_all_inputs(year: i32) -> impl Iterator<Item = Result<AocInput, Error>> {
    let aoc_today = chrono::Utc::now().with_timezone(&chrono_tz::EST);
    let aoc_start = chrono_tz::EST
        .with_ymd_and_hms(year, 12, 1, 0, 0, 0)
        .unwrap();
    if aoc_today < aoc_start {
        let delta = aoc_start - aoc_today;
        let h = delta.num_hours();
        let m = delta.num_minutes() - 60 * h;
        let s = delta.num_seconds() - 60 * (60 * h + m);
        println!(
            "Advent of Code {} has not yet started. Start in {}h {}m {}s.",
            year, h, m, s
        );
    }
    let available_days = (1..=25).filter(move |d| {
        chrono_tz::EST
            .with_ymd_and_hms(year, 12, *d, 0, 0, 0)
            .unwrap()
            <= aoc_today
    });

    available_days.map(move |d| AocInput::try_new(year, d))
}

pub struct AocInput {
    file: std::io::BufReader<std::fs::File>,
}

impl AocInput {
    pub fn try_new(year: i32, day: u32) -> Result<Self, Error> {
        let file = std::io::BufReader::new(get_input(year, day)?);
        Ok(Self { file })
    }
    pub fn lines(self) -> impl Iterator<Item = std::io::Result<String>> {
        self.file.lines()
    }
    pub fn to_2d_array(self) -> Vec<Vec<char>> {
        self.to_2d_array_finding(|_| false).0
    }
    pub fn to_2d_array_finding(self, p: fn(char) -> bool) -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
        let mut found = Vec::new();
        let mut res = Vec::new();
        for (y, l) in self.lines().enumerate() {
            let mut res_line = Vec::new();
            for (x, c) in l.unwrap().chars().enumerate() {
                if p(c) {
                    found.push((x, y));
                }
                res_line.push(c);
            }
            res.push(res_line)
        }
        (res, found)
    }
    pub fn to_2d_array_mapped<T>(self, map: fn(char) -> T) -> Vec<Vec<T>> {
        let mut res = Vec::new();
        for l in self.lines() {
            res.push(l.unwrap().chars().map(map).collect())
        }
        res
    }
}
