pub const MIN_ARGS_LENGTH: u8 = 3;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn file_path(&self) -> &String {
        &self.file_path
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }
}

pub struct ConfigBuiler {
    query: Option<String>,
    file_path: Option<String>,
    ignore_case: bool,
}

impl ConfigBuiler {
    pub fn new() -> Self {
        ConfigBuiler {
            query: None,
            file_path: None,
            ignore_case: false,
        }
    }

    pub fn query(mut self, query: Option<String>) -> Self {
        self.query = query;
        self
    }

    pub fn file_path(mut self, file_path: Option<String>) -> Self {
        self.file_path = file_path;
        self
    }

    pub fn ignore_case(mut self, ignore_case: bool) -> Self {
        self.ignore_case = ignore_case;
        self
    }

    pub fn build(self) -> Result<Config, &'static str> {
        Ok(Config {
            query: self.query.ok_or("query argument is not provided")?,
            file_path: self.file_path.ok_or("file path argument is not provided")?,
            ignore_case: self.ignore_case,
        })
    }
}
