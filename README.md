# rust使用yml配置文件

# 引言

​      在接触rust之前一直是使用java，已经习惯了springboot那一套东西，所以对使用.env或者.toml做配置文件都不习惯。就想着复刻springboot的配置习惯，花了点时间做好了，就不知道自己做的怎么样，请诸位大佬指教。



## 第一步：创建项目

```  shell
cargo new yml_rust
```

## 第二步：创建配置文件

``` shell
cd yml_rust
mkdir resources  #经过实践我发现rust打包的时候不会包含配置文件，所以我建议配置文件和src文件夹平级这样方便在发布时复制配置文件
cd resources 
touch application.yml application-dev.yml application-test.yml application-prod.yml #创建配置配置文件 
# application.yml 环境配置文件 可以通过配置dev、test、prod切换配置
# application-dev.yml  开发环境配置
# application-test.yml 测试环境配置
# application-prod.yml 生产环境配置
```

编辑 application.yml 添加一下内容：

```yaml
#切换配置文件
profiles:
  active: dev
```

编辑 application-dev.yml 添加一下内容：

```
# mysql 配置
mysql:
  host: 127.0.0.1
  port: 3306
  user: root
  password: jishuzhai
  db: rust_book
  #最小连接数
  pool_min_idle: 8
  #最大连接数
  pool_max_open: 32
  #连接超时时间单位秒
  timeout_seconds: 15
```

编辑 application-prod.yml 添加一下内容：

```
# mysql 配置
mysql:
  host: 127.0.0.2
  port: 3306
  user: root
  password: jishuzhai
  db: rust_book
  #最小连接数
  pool_min_idle: 8
  #最大连接数
  pool_max_open: 32
  #连接超时时间单位秒
  timeout_seconds: 15
```

编辑 application-test.yml 添加一下内容：

```
# mysql 配置
mysql:
  host: 127.0.0.3
  port: 3306
  user: root
  password: jishuzhai
  db: rust_book
  #最小连接数
  pool_min_idle: 8
  #最大连接数
  pool_max_open: 32
  #连接超时时间单位秒
  timeout_seconds: 15
```

> 注意dev和prod、test的差异只有ip地址不一样 这是为了后面测试切换功能

## 第三步：添加配置

在Cargo.toml文件中**[dependencies]**标签下添加一下配置：

```toml
serde = { version = "1.0.125", features = ["derive"] }
serde_json = "1.0.75"
lazy_static = "1.4.0"
serde_yaml = "0.8.23"
schemars = "0.8.8"
```

## 第四步：编写读取配置代码

``` shell
cd /Users/fuping/Desktop/yml_rust/src 
mkdir load_config  #加载配置文件的代码模块
touch load_config/mod.rs load_config/models.rs load_config/init_load_config.rs
mkdir util         #全局变量
touch util/mod.rs util/constant.rs
```

编辑 load_config/mod.rs文件添加一下代码：

```rust
/****
 * 加载系统配置
 */
pub mod models;
pub mod init_load_config;
```

编辑load_config/models.rs 添加一下代码

```
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GlobalConfig {
    pub mysql: Mysql,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Mysql {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub db: String,
    pub pool_min_idle: u64,
    pub pool_max_open: u64,
    pub timeout_seconds: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Profiles {
    pub active: String,
}

#[derive(Serialize, Deserialize)]
pub struct EnvConfig {
    pub profiles: Profiles,
}
```

编辑load_config/init_load_config.rs文件添加一下代码

```rust
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
```

## 第五步：测试

运行test_load_env_conf_mysql 测试看看运行结果，正常情况下会打印一下结果：

``` Load profile
Load profile
mysqlConfigs:{"host":"127.0.0.1","port":3306,"user":"root","password":"jishuzhai","db":"rust_book","pool_min_idle":8,"pool_max_open":32,"timeout_seconds":15}
```

然后编辑application.yml 更改配置查看配置切换是否正常。

## 在项目中使用：

编辑util/mod.rs 文件添加以下内容：

```
pub mod constant;
```

编辑util/constant.rs文件添加以下内容：

```rust
use lazy_static::lazy_static;
use crate::load_config::models::GlobalConfig;
use crate::load_config::init_load_config::load_conf;

lazy_static! {
    ///
    /// 全局配置
    ///
    pub static ref GLOBAL_CONFIG: GlobalConfig = load_conf().unwrap();
}
```

编辑main.rs 添加以下内容

```rust
mod load_config;
mod util;

use util::constant::GLOBAL_CONFIG;

fn main() {
    let config = &GLOBAL_CONFIG;
    println!("{:?}", config.mysql);
}
```

运行项目

``` shell
cargo run
```

运行结果如下：

Load profile

Mysql { host: "127.0.0.3", port: 3306, user: "root", password: "jishuzhai", db: "rust_book", pool_min_idle: 8, pool_max_open: 32, timeout_seconds: 15 }



> 