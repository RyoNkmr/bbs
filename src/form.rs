use rocket::http::RawStr;
use rocket::request::FromFormValue;

pub struct UserName(String);

impl UserName {
    pub fn as_str<'a>(&'a self) -> &'a str {
        let UserName(name) = self;
        name
    }
}

impl<'v> FromFormValue<'v> for UserName {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<UserName, &'v RawStr> {
        if form_value.len() == 0 {
            return Ok(UserName("名無しさん".to_string()));
        }
        match form_value.url_decode() {
            Ok(decoded) => Ok(UserName(decoded)),
            _ => Err(form_value),
        }
    }
}
