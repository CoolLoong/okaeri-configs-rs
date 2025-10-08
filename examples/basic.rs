use okaeri_configs::prelude::*;

fn main() -> ConfigResult<()> {
    let _default_config = AppConfig {
        api_key: "secret_key_here".to_string(),
        my_list: vec!["value1".to_string(), "value2".to_string()],
        server: ServerConfig {
            host: "localhost".to_string(),
            port: 8080,
        },
        _internal_field: "internal".to_string(),
    };

    println!("Creating TOML config...");
    let mut manager = ConfigManager::<AppConfig>::create()
        .with_path("example_config.toml")
        .build()?;
    manager.update(|config| {
        config.server.port = 9000;
    })?;
    println!("Config saved to example_config.toml");

    println!("\nCreating JSON config...");
    let _ = ConfigManager::<AppConfig>::create()
        .with_path("example_config.json")
        .build()?;
    println!("Config saved to example_config.json");

    println!("\nCreating YAML config...");
    let _ = ConfigManager::<AppConfig>::create()
        .with_path("example_config.yaml")
        .build()?;
    println!("Config saved to example_config.yaml");
    Ok(())
}

#[derive(Config, Serialize, Deserialize, Default, Debug)]
#[comment(
    r#"
################################################################
#                                                              #
#    okaeri-configs test                                       #
#                                                              #
#    Trouble using? Check out the documentation!               #
#    https://github.com/CoolLoong/okaeri-configs-rs            #
#                                                              #
################################################################"#
)]
struct AppConfig {
    #[comment("API secret key")]
    #[comment("Keep this secure!")]
    #[env("API_KEY")]
    api_key: String,

    #[key("myCustomList")]
    #[serde(rename = "myCustomList")]
    #[comment("List of allowed values")]
    my_list: Vec<String>,

    #[comment("Server configuration")]
    server: ServerConfig,

    #[serde(skip)]
    _internal_field: String,
}

#[derive(Config, Serialize, Deserialize, Default, Debug)]
struct ServerConfig {
    #[env("SERVER_HOST")]
    #[comment("Server host address")]
    host: String,

    #[env("SERVER_PORT")]
    #[comment("Server port number")]
    port: u16,
}
