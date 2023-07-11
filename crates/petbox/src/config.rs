use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    //pub config_dir: PathBuf,
    pub data_dir: PathBuf,
}

impl Config {
    pub fn build() -> Self {
        let data_dir;
        match std::env::var("PETBOX_DATA_DIR") {
            Ok(dir) => data_dir = PathBuf::from(dir),
            Err(_) => {
                let xdg_dirs = xdg::BaseDirectories::with_prefix("petbox").unwrap();
                //let config_dir = xdg_dirs.get_config_home().into();
                data_dir = xdg_dirs.get_data_home();
            }
        }
        let s = Self {
            //config_dir,
            data_dir,
        };
        debug!("{:#?}", s);
        s
    }
    pub fn get_container_dir(self, name: &str) -> PathBuf {
        self.data_dir.join("containers").join(name)
    }
    pub fn get_container_rootfs(self, name: &str) -> PathBuf {
        self.get_container_dir(name).join("rootfs")
    }
}
