use serde::Serialize;

use crate::flutter_issue::FlutterIssue;
use crate::flutter_issue::Severity as FlutterIssueSeverity;

#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[allow(dead_code)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Blocker,
    Critical,
    Major,
    Minor,
    Info,
}

#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Location {
    pub path: String,
    pub positions: Position,
}

#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Position {
    pub begin: PositionInfo,
    //pub end: PositionInfo,
}

#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PositionInfo {
    pub line: u32,
    pub column: u32,
}

#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CodeClimateIssue {
    pub severity: Severity,
    pub description: String,
    pub fingerprint: String,
    pub location: Location,
}

impl From<FlutterIssue> for CodeClimateIssue {
    fn from(source: FlutterIssue) -> Self {
        let severity = match source.severity {
            FlutterIssueSeverity::Error => Severity::Blocker,
            FlutterIssueSeverity::Warning => Severity::Major,
            FlutterIssueSeverity::Info => Severity::Info,
        };

        let description = source.message;
        let fingerprint = format!("{:x}", md5::compute(source.raw));
        let path = source.file;
        let line = source.line;
        let column = source.column;

        CodeClimateIssue {
            severity,
            description,
            fingerprint,
            location: Location {
                path,
                positions: Position {
                    begin: PositionInfo { line, column },
                },
            },
        }
    }
}
