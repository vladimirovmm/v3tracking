use crate::system::config::ServerConfigs;

pub mod config;

pub struct System{
    pub configs:ServerConfigs
}
impl System{
    pub fn inic()->Result<System,String>{
        let conf = ServerConfigs::load();
        if conf.is_err() {
            return Err( "Неудалось загрузить настройки".to_string()
                    + ". "
                    + conf.err().unwrap().as_str()
                );
        }

        Ok(
            System{
                configs:conf.unwrap()
            }
        )
    }
}