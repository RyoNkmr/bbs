use rocket::http::RawStr;
use rocket::request::FromFormValue;
use tripcode::*;

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
        let decoded = form_value.url_decode().or(Err(form_value))?;
        let replaced = decoded.replace("◆", "◇").replace("★", "☆");
        let sharp_point = replaced.find('#');

        if sharp_point.is_none() {
            return Ok(UserName(replaced));
        }

        let (name, key_with_sharp) = replaced.split_at(sharp_point.unwrap());
        let trip = '◆'.to_string() + &Mona::generate(&key_with_sharp[1..key_with_sharp.len()]);
        Ok(UserName(name.to_string() + &trip))
    }
}
