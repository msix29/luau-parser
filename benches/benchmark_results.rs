use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Read},
    ops::{Deref, DerefMut},
    path::PathBuf,
    time::Duration,
};

const SAVE_PATH: &str = "bench-results";

#[derive(Debug)]
pub struct BenchMarkResult {
    pub lowest: Duration,
    pub highest: Duration,
    pub total: Duration,
}

#[inline]
fn serialize_duration(duration: &Duration) -> String {
    format!("{},{}", duration.as_secs(), duration.subsec_nanos())
}

#[inline]
fn deserialize_duration(serialized: &str) -> Option<Duration> {
    let mut splits = serialized.split(',');

    let secs = splits.next().map(str::parse)?.ok()?;
    let nanos = splits.next().map(str::parse)?.ok()?;

    Some(Duration::new(secs, nanos))
}

impl BenchMarkResult {
    #[inline]
    pub fn serialize(&self) -> String {
        format!(
            "{}-{}-{}",
            serialize_duration(&self.lowest),
            serialize_duration(&self.total),
            serialize_duration(&self.highest)
        )
    }

    #[inline]
    pub fn deserialize(serialized: &str) -> Option<Self> {
        let mut splits = serialized.split('-');

        Some(Self {
            lowest: deserialize_duration(splits.next()?)?,
            total: deserialize_duration(splits.next()?)?,
            highest: deserialize_duration(splits.next()?)?,
        })
    }
}

type BenchMarkResultsInner = HashMap<PathBuf, BenchMarkResult>;

#[derive(Default, Debug)]
pub struct BenchMarkResults(BenchMarkResultsInner);

macro_rules! load_error {
    () => {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Malformed benchmark results file at '{}'", SAVE_PATH),
        )
    };
}

impl BenchMarkResults {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn save(&self) -> io::Result<()>  {
        let mut str = String::new();

        for (path, result) in self.iter() {
            str.push_str(&path.to_string_lossy());
            str.push('=');
            str.push_str(&result.serialize());
        }

        fs::write(SAVE_PATH, str)
    }

    pub fn load() -> io::Result<Self> {
        let mut file = File::open(SAVE_PATH)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let mut results = Self::new();

        for line in content.lines() {
            let mut splits = line.split('=');

            let path = splits
                .next()
                .map(PathBuf::from)
                .ok_or_else(|| load_error!())?;

            let result = BenchMarkResult::deserialize(splits.next().ok_or_else(|| load_error!())?)
                .ok_or_else(|| load_error!())?;

            results.insert(path, result);
        }

        Ok(results)
    }
}

impl Deref for BenchMarkResults {
    type Target = BenchMarkResultsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for BenchMarkResults {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
