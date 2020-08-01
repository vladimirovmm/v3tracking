use serde::{Serialize, Deserialize};
use toml;
use std::io::{Read, Write};

#[derive(Debug,Clone, Default, Serialize, Deserialize)]
pub struct ServerConfigs{
    #[serde(rename="Server")]
    pub listen:ListenServerConfigs,
    #[serde(rename="DB")]
    pub postgres:PostgresServerConfigs
}
impl ServerConfigs{
    pub fn new()->ServerConfigs{
        ServerConfigs{
            listen:ListenServerConfigs::new(),
            postgres:PostgresServerConfigs::new(),
        }
    }
    ///
    /// Загрузить настройки из configs/config.toml
    ///
    pub fn load()->Result<ServerConfigs,String>{
        use std::fs::File;

        let file = File::open("configs/config.toml");
        if file.is_err() { return Err("Файл не найден".to_string()); }

        let mut content = String::new();
        if file.unwrap().read_to_string(&mut content).is_err(){ return Err("Неудалость прочесть файл".to_string()); }

        let configs = toml::from_str::<ServerConfigs>(content.as_str());
        if configs.is_err() { return Err("Не верный формат toml".to_string()); }
        Ok(configs.unwrap())
    }
    ///
    /// Сохранить настройки сервера
    ///
    pub fn _save(&self)->Result<(),String>{
        let string = toml::to_string(self);
        if string.is_err() { return Err("Неудалось преобразовать в строку".to_string()); }
        let string = string.unwrap();

        use std::fs::File;
        let file = File::create("configs/config.toml");
        if file.is_err() { return Err("configs/config.toml не найден и не удалось создать".to_string()); }
        if file.unwrap().write_all(string.as_bytes()).is_err(){ return Err("configs/config.toml неудалось записать".to_string()); }

        Ok(())
    }
}
#[derive(Debug,Clone, Default, Serialize, Deserialize)]
pub struct ListenServerConfigs{
    // IP прослушивания
    pub ip:Option<String>,
    // Порт прослушивания
    pub port:Option<i16>,
    // Количество потоков
    pub workers:Option<i8>,
    // Количество одноврименных соединений.
    // если поставить 1 то только один пользователь сможет пользоваться сайтом
    // пока его соединение не закроется
    pub maxconn:Option<i16>,
    pub maxconnrate:Option<i8>,
    // Время жизни соединения
    pub keep_alive:Option<i32>,
    // Время ожидания соединения
    pub client_timeout:Option<i32>,
    // Врем выполнения
    pub client_shutdown:Option<i32>
}
impl ListenServerConfigs{
    pub fn new()->ListenServerConfigs{
        ListenServerConfigs{
            ip: Some("127.0.0.1".to_string()),
            port: Some(8080_i16),
            workers:Some(1),
            maxconn:Some(30),
            maxconnrate:Some(1),
            keep_alive:Some(500),
            client_timeout:Some(30),
            client_shutdown:Some(30)
        }
    }
    /// Получить адрес прослушивания сервера
    pub fn get_addr(&self)->String{
        let addr = match &self.ip {
            Some(addr) => addr.clone(),
            None => "127.0.0.1".to_string()
        };
        let port = match &self.port {
            Some(port) => port.to_string(),
            None => "8080".to_string()
        };
        addr + ":" + port.as_str()
    }
    /// Количество потоков
    pub fn get_workers(&self)->usize{
        match &self.workers {
            Some(v) => v.clone() as usize,
            None => 1_usize
        }
    }
    /// Количество одноврименных соединений.
    /// если поставить 1 то только один пользователь сможет пользоваться сайтом
    /// пока его соединение не закроется
    pub fn get_maxconn(&self)->usize{
        match &self.maxconn {
            Some(v) => v.clone() as usize,
            None => 30_usize
        }
    }
    pub fn get_maxconnrate(&self)->usize{
        match &self.maxconnrate {
            Some(v) => v.clone() as usize,
            None => 1_usize
        }
    }
    /// Время жизни соединения
    pub fn get_keep_alive(&self)->usize{
        match &self.keep_alive {
            Some(v) => v.clone() as usize,
            None => 500_usize
        }
    }
    /// Время ожидания соединения
    pub fn get_client_timeout(&self)->u64{
        match &self.client_timeout {
            Some(v) => v.clone() as u64,
            None => 30_u64
        }
    }
    /// Врем выполнения
    pub fn get_client_shutdown(&self)->u64{
        match &self.client_shutdown {
            Some(v) => v.clone() as u64,
            None => 30_u64
        }
    }
}
#[derive(Debug,Clone, Default, Serialize, Deserialize)]
pub struct PostgresServerConfigs{
    pub ip:Option<String>,
    pub port:Option<i16>,
    pub user:Option<String>,
    pub password:Option<String>,
    pub dbname:Option<String>
}
impl PostgresServerConfigs{
    pub fn new()->PostgresServerConfigs{
        PostgresServerConfigs{
            ip:Some("127.0.0.1".to_string()),
            port:Some(5432_i16),
            user:Some("demo".to_string()),
            password:Some("demo".to_string()),
            dbname:Some("demo".to_string())
        }
    }
}

