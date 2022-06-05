mod ms_data_set;
mod ms_configuration;
mod ms_mapping;

#[cfg(test)]
mod tests
{
    use std::any::{Any, TypeId};
    use std::{mem, thread};
    use std::time::Duration;
    use crate::ms_data_set::MsDataSet;



    #[test]
    fn it_works()
    {
        let mut ds = MsDataSet::new();
        for i in 0..100
        {
            ds.add_entity(Box::new(format!("a-{}",i)));
        }

        for i in 0..100
        {
            let e = ds.get_entity::<String>(i);
            println!("e: {}", *e);
        }


        let result = 2 + 2;
        assert_eq!(result, 4);
        println!("ok");
        drop(ds);
        //thread::sleep(Duration::from_secs(5));
    }
}
