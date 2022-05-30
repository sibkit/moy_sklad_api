mod ms_data_set;
mod ms_configuration;
mod ms_mapping;

#[cfg(test)]
mod tests
{
    use crate::ms_data_set::MsDataSet;

    #[test]
    fn it_works()
    {
        let mut ds = MsDataSet::new();
        let result = 2 + 2;
        assert_eq!(result, 4);
        println!("ok")
    }
}
