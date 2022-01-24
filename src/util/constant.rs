use lazy_static::lazy_static;
use crate::load_config::models::GlobalConfig;
use crate::load_config::init_load_config::load_conf;


lazy_static! {
    ///
    /// 全局配置
    ///
    pub static ref GLOBAL_CONFIG: GlobalConfig = load_conf().unwrap();
}
