use config::Config;
use serde::Deserialize;


#[derive(Debug,Deserialize)]
pub struct Setting{
    port:u32,
}

impl Setting{
    pub fn new()->Setting{
        let settings = Config::builder()
            .add_source(config::File::with_name("config").format(config::FileFormat::Yaml))
            .build()
            .unwrap();
        let setting = settings.get::<u32>("port").unwrap();
        Setting{
            port:setting,
        }
    }

    pub fn get_port(&self)->u32{
        self.port
    }
}