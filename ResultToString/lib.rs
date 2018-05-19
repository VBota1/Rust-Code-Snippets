pub trait ToString
{
    fn to_string (&self) -> String;
}

impl ToString for Result<String,String>
{
    fn to_string (&self) -> String
    {
        match self {
            Ok(message) => message.clone(),
            Err(message) => message.clone(),
        }
    }
}
