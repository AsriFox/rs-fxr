use crate::serde::Description;
use validator::Validate;

pub fn parse(jfxr: serde_json::Value) -> Result<Description, String> {
    match serde_json::from_value::<Description>(jfxr) {
        Ok(description) => {
            if let Err(errors) = description.validate() {
                Err(format!("{:?}", errors))
            } else {
                Ok(description)
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn parse_str(s: &str) -> Result<Description, String> {
    match serde_json::from_str::<Description>(s) {
        Ok(description) => {
            if let Err(errors) = description.validate() {
                Err(format!("{:?}", errors))
            } else {
                Ok(description)
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn parse_reader<R>(rdr: R) -> Result<Description, String>
where
    R: std::io::Read,
{
    match serde_json::from_reader::<R, Description>(rdr) {
        Ok(description) => {
            if let Err(errors) = description.validate() {
                Err(format!("{:?}", errors))
            } else {
                Ok(description)
            }
        }
        Err(err) => Err(err.to_string()),
    }
}
