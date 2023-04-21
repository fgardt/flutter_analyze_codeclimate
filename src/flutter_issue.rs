use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FlutterIssue {
    pub severity: Severity,
    pub message: String,
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub raw: String,
}

#[derive(Debug)]
pub enum ConvertError {
    InvalidFormatError,
    RegexError,
}

impl TryFrom<String> for FlutterIssue {
    type Error = ConvertError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = Regex::new(r"\[(error|warning|info)\]\s(.*(?:\n{2}(?:\n|.+[:.,!?;'])+\n{2})?)\s\((.*):(\d+):(\d+)\)").unwrap();

        if !re.is_match(value.as_str()) {
            return Err(ConvertError::InvalidFormatError);
        }

        let groups = re.captures(value.as_str()).unwrap();

        let severity = match groups.get(1).unwrap().as_str() {
            "error" => Severity::Error,
            "warning" => Severity::Warning,
            "info" => Severity::Info,
            _ => return Err(ConvertError::RegexError),
        };

        let message = groups.get(2).unwrap().as_str().to_owned();
        let file = groups.get(3).unwrap().as_str().to_owned();
        let line = groups.get(4).unwrap().as_str().parse::<u32>().unwrap();
        let column = groups.get(5).unwrap().as_str().parse::<u32>().unwrap();

        Ok(FlutterIssue {
            severity,
            message,
            file,
            line,
            column,
            raw: value,
        })
    }
}
