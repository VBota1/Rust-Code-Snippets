pub trait ToVecString
{
        fn to_vec_string (&self) -> Vec<String>;
}

impl ToVecString for str
{
        fn to_vec_string (&self) -> Vec<String>
        {
                self.split_whitespace().map ( |word| word.to_string() ).collect()         
        }
}
