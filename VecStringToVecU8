pub trait ToVecU8
{
        fn to_vec_u8 (self) -> Vec<u8>;
}

impl ToVecU8 for Vec<String>
{
        fn to_vec_u8 ( self ) -> Vec<u8>
        {      
                let string_vector: Vec<String> = self.into_iter().map( |a| a.to_string() ).collect();       
                let one_string = string_vector.as_slice().join(" ");
                one_string.into_bytes()      
         }
}
