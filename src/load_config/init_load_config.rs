use crate::load_config::models::{GlobalConfig, EnvConfig};
use schemars::schema::RootSchema;
use serde_yaml::from_str as yaml_from_str;
use serde_json::{from_str as json_from_str, to_string_pretty};
use std::fs::read_to_string;

///
/// 加载环境配置
///
///
fn load_env_conf() -> Option<EnvConfig> {
    let schema = yaml_from_str::<RootSchema>(
        &read_to_string("resources/application.yml").expect("Error loading configuration file resources/application.yml, please check the configuration!"),
    );
    return match schema {
        Ok(json) => {
            let data = to_string_pretty(&json).expect("resources/application.yml file data error！");
            let p: EnvConfig = json_from_str(&*data).expect("Failed to transfer JSON data to EnvConfig object！");
            return Some(p);
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    };
}

///
/// 根据环境配置加载全局配置
/// action  dev 开始环境 test 测试环境 prod 生产环境
///
fn load_global_config(action: String) -> Option<GlobalConfig> {
    let path = format!("resources/application-{}.yml", &action);
    let schema = yaml_from_str::<RootSchema>(
        &read_to_string(&path).unwrap_or_else(|_| panic!("Error loading configuration file {}, please check the configuration!", &path)),
    );
    return match schema {
        Ok(json) => {
            let data = to_string_pretty(&json).unwrap_or_else(|_| panic!("{} file data error！, please check the configuration!", path));
            let p = json_from_str(&*data).expect("Failed to transfer JSON data to BriefProConfig object！");
            return Some(p);
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    };
}

///
/// 先加载环境配置 在根据当前加载的环境 去加载相应的信息
///
pub fn load_conf() -> Option<GlobalConfig> {
    println!("{}", "Load profile");
    if let Some(init) = load_env_conf() {
        return load_global_config(init.profiles.active);
    }
    None
}


#[test]
fn test_load_env_conf_mysql() {
    let pro = load_conf();
    pro.as_ref().map(|a| {
        println!("mysqlConfig:{}", serde_json::to_string(&a.mysql).unwrap());
    });
}

