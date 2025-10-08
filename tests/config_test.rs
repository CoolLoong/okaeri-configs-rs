use okaeri_configs::prelude::*;
use std::fs;
use std::path::PathBuf;
use temp_env::with_vars;

#[derive(Config, Serialize, Deserialize, Default, Debug, PartialEq)]
#[comment(r#"Test Configuration Header"#)]
#[comment(r#"Second line of header"#)]
struct TestConfig {
    #[comment("String field")]
    name: String,
    #[comment("Number field")]
    age: u32,
    #[comment("Boolean field")]
    enabled: bool,
}

#[derive(Config, Serialize, Deserialize, Default, Debug, PartialEq)]
struct ConfigWithEnv {
    #[env("TEST_HOST")]
    #[comment("Host from environment")]
    host: String,
    #[env("TEST_PORT")]
    #[comment("Port from environment")]
    port: u16,
    #[comment("Regular field")]
    name: String,
}

#[derive(Config, Serialize, Deserialize, Default, Debug, PartialEq)]
struct ConfigWithCustomKey {
    #[key("customName")]
    #[serde(rename = "customName")]
    #[comment("Field with custom key")]
    name: String,
    #[key("myAge")]
    #[serde(rename = "myAge")]
    #[comment("Age with custom key")]
    age: u32,
}

#[derive(Config, Serialize, Deserialize, Default, Debug, PartialEq)]
struct NestedConfig {
    #[comment("Main configuration")]
    main: MainConfig,
    #[comment("Database configuration")]
    database: DatabaseConfig,
}

#[derive(Config, Serialize, Deserialize, Default, Debug, PartialEq)]
struct MainConfig {
    #[comment("Application name")]
    app_name: String,
    #[comment("Version")]
    version: String,
}

#[derive(Config, Serialize, Deserialize, Default, Debug, PartialEq)]
struct DatabaseConfig {
    #[comment("Database host")]
    host: String,
    #[comment("Database port")]
    port: u16,
    #[comment("Database name")]
    name: String,
}

#[derive(Config, Serialize, Deserialize, Default, Debug, PartialEq)]
struct ConfigWithSkip {
    #[comment("Public field")]
    public_field: String,
    #[serde(skip)]
    internal_field: String,
    #[comment("Another public field")]
    another_field: i32,
}

#[derive(Config, Serialize, Deserialize, Default, Debug, PartialEq)]
struct ConfigV1 {
    #[comment("Field 1")]
    field1: String,
    #[comment("Field 2")]
    field2: i32,
}

#[derive(Config, Serialize, Deserialize, Default, Debug, PartialEq)]
struct ConfigV2 {
    #[comment("Field 1")]
    field1: String,
    #[comment("Field 2")]
    field2: i32,
    #[comment("New field 3")]
    field3: bool,
    #[comment("New field 4")]
    field4: String,
}

#[ctor::ctor]
fn setup() {
    let test_dir = PathBuf::from("test_configs");
    if test_dir.exists() && test_dir.is_file() {
        fs::remove_file(&test_dir).expect("Failed to remove test_configs file");
    }
    if !test_dir.exists() {
        fs::create_dir(&test_dir).expect("Failed to create test_configs directory");
    }
}

#[ctor::dtor]
fn teardown() {
    let test_dir = PathBuf::from("test_configs");
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir).ok();
    }
}

fn test_file_path(name: &str) -> PathBuf {
    PathBuf::from("test_configs").join(name)
}

#[test]
fn test_serialization() {
    let path_toml = test_file_path("test_serialization.toml");
    let mut manager = ConfigManager::<TestConfig>::create()
        .with_path(&path_toml)
        .build()
        .expect("Failed to create TOML manager");

    manager.get_mut().name = "Alice".to_string();
    manager.get_mut().age = 30;
    manager.get_mut().enabled = true;

    manager.save().expect("Failed to save TOML config");

    assert!(path_toml.exists());

    let content = fs::read_to_string(&path_toml).expect("Failed to read TOML file");
    assert!(content.contains("Alice"));
    assert!(content.contains("30"));
    assert!(content.contains("true"));

    #[cfg(feature = "json")]
    {
        let path_json = test_file_path("test_serialization.json");
        let mut manager = ConfigManager::<TestConfig>::create()
            .with_path(&path_json)
            .build()
            .expect("Failed to create JSON manager");

        manager.get_mut().name = "Charlie".to_string();
        manager.get_mut().age = 35;
        manager.get_mut().enabled = true;

        manager.save().expect("Failed to save JSON config");

        assert!(path_json.exists());

        let content = fs::read_to_string(&path_json).expect("Failed to read JSON file");
        assert!(content.contains("Charlie"));
        assert!(content.contains("35"));
        assert!(content.contains("true"));
    }

    #[cfg(feature = "yaml")]
    {
        let path_yaml = test_file_path("test_serialization.yaml");
        let mut manager = ConfigManager::<TestConfig>::create()
            .with_path(&path_yaml)
            .build()
            .expect("Failed to create YAML manager");

        manager.get_mut().name = "Eve".to_string();
        manager.get_mut().age = 28;
        manager.get_mut().enabled = true;

        manager.save().expect("Failed to save YAML config");

        assert!(path_yaml.exists());

        let content = fs::read_to_string(&path_yaml).expect("Failed to read YAML file");
        assert!(content.contains("Eve"));
        assert!(content.contains("28"));
        assert!(content.contains("true"));
    }
}

#[test]
fn test_deserialization() {
    let path_toml = test_file_path("test_deserialization.toml");
    let mut manager = ConfigManager::<TestConfig>::create()
        .with_path(&path_toml)
        .build()
        .expect("Failed to create TOML manager");

    manager.get_mut().name = "Bob".to_string();
    manager.get_mut().age = 25;
    manager.get_mut().enabled = false;

    manager.save().expect("Failed to save TOML config");

    let loaded_manager = ConfigManager::<TestConfig>::create()
        .with_path(&path_toml)
        .build()
        .expect("Failed to load TOML config");

    let loaded_config = loaded_manager.get();
    assert_eq!(loaded_config.name, "Bob");
    assert_eq!(loaded_config.age, 25);
    assert_eq!(loaded_config.enabled, false);

    #[cfg(feature = "json")]
    {
        let path_json = test_file_path("test_deserialization.json");
        let mut manager = ConfigManager::<TestConfig>::create()
            .with_path(&path_json)
            .build()
            .expect("Failed to create JSON manager");

        manager.get_mut().name = "David".to_string();
        manager.get_mut().age = 40;
        manager.get_mut().enabled = false;

        manager.save().expect("Failed to save JSON config");

        let loaded_manager = ConfigManager::<TestConfig>::create()
            .with_path(&path_json)
            .build()
            .expect("Failed to load JSON config");

        let loaded_config = loaded_manager.get();
        assert_eq!(loaded_config.name, "David");
        assert_eq!(loaded_config.age, 40);
        assert_eq!(loaded_config.enabled, false);
    }

    #[cfg(feature = "yaml")]
    {
        let path_yaml = test_file_path("test_deserialization.yaml");
        let mut manager = ConfigManager::<TestConfig>::create()
            .with_path(&path_yaml)
            .build()
            .expect("Failed to create YAML manager");

        manager.get_mut().name = "Frank".to_string();
        manager.get_mut().age = 45;
        manager.get_mut().enabled = false;

        manager.save().expect("Failed to save YAML config");

        let loaded_manager = ConfigManager::<TestConfig>::create()
            .with_path(&path_yaml)
            .build()
            .expect("Failed to load YAML config");

        let loaded_config = loaded_manager.get();
        assert_eq!(loaded_config.name, "Frank");
        assert_eq!(loaded_config.age, 45);
        assert_eq!(loaded_config.enabled, false);
    }
}

#[test]
fn test_env() {
    with_vars(
        [
            ("TEST_HOST", Some("localhost")),
            ("TEST_PORT", Some("8080")),
        ],
        || {
            let path_toml = test_file_path("test_env.toml");
            let manager = ConfigManager::<ConfigWithEnv>::create()
                .with_path(&path_toml)
                .build()
                .expect("Failed to create manager");
            assert_eq!(manager.get().host, "localhost");
            assert_eq!(manager.get().port, 8080);

            #[cfg(feature = "json")]
            {
                let path_json = test_file_path("test_env.json");
                let manager = ConfigManager::<ConfigWithEnv>::create()
                    .with_path(&path_json)
                    .build()
                    .expect("Failed to create JSON manager");
                assert_eq!(manager.get().host, "localhost");
                assert_eq!(manager.get().port, 8080);
            }

            #[cfg(feature = "yaml")]
            {
                let path_yaml = test_file_path("test_env.yaml");
                let manager = ConfigManager::<ConfigWithEnv>::create()
                    .with_path(&path_yaml)
                    .build()
                    .expect("Failed to create YAML manager");
                assert_eq!(manager.get().host, "localhost");
                assert_eq!(manager.get().port, 8080);
            }
        },
    );
}

#[test]
fn test_env_missing() {
    let path = test_file_path("test_env_missing.toml");
    let mut manager = ConfigManager::<ConfigWithEnv>::create()
        .with_path(&path)
        .build()
        .expect("Failed to create manager");

    manager.get_mut().host = "default_host".to_string();
    manager.get_mut().port = 3000;
    manager.get_mut().name = "test".to_string();

    assert_eq!(manager.get().host, "default_host");
    assert_eq!(manager.get().port, 3000);
    assert_eq!(manager.get().name, "test");
}

#[test]
fn test_custom_keys() {
    let path = test_file_path("test_custom_keys.toml");

    let mut manager = ConfigManager::<ConfigWithCustomKey>::create()
        .with_path(&path)
        .build()
        .expect("Failed to create manager");

    manager.get_mut().name = "Test".to_string();
    manager.get_mut().age = 25;

    manager.save().expect("Failed to save config");

    let content = fs::read_to_string(&path).expect("Failed to read file");
    assert!(content.contains("customName"));
    assert!(content.contains("myAge"));
    assert!(!content.contains("\"name\""));
    assert!(!content.contains("\"age\""));
}

#[test]
fn test_nested_config() {
    let path = test_file_path("test_nested.toml");

    let mut manager = ConfigManager::<NestedConfig>::create()
        .with_path(&path)
        .build()
        .expect("Failed to create manager");

    manager.get_mut().main.app_name = "MyApp".to_string();
    manager.get_mut().main.version = "1.0.0".to_string();
    manager.get_mut().database.host = "db.example.com".to_string();
    manager.get_mut().database.port = 5432;
    manager.get_mut().database.name = "mydb".to_string();

    manager.save().expect("Failed to save config");

    let loaded_manager = ConfigManager::<NestedConfig>::create()
        .with_path(&path)
        .build()
        .expect("Failed to load config");

    let loaded = loaded_manager.get();
    assert_eq!(loaded.main.app_name, "MyApp");
    assert_eq!(loaded.main.version, "1.0.0");
    assert_eq!(loaded.database.host, "db.example.com");
    assert_eq!(loaded.database.port, 5432);
    assert_eq!(loaded.database.name, "mydb");
}

#[test]
fn test_skip_field() {
    let path = test_file_path("test_skip.toml");

    let mut manager = ConfigManager::<ConfigWithSkip>::create()
        .with_path(&path)
        .build()
        .expect("Failed to create manager");

    manager.get_mut().public_field = "public".to_string();
    manager.get_mut().internal_field = "internal".to_string();
    manager.get_mut().another_field = 42;

    manager.save().expect("Failed to save config");

    let content = fs::read_to_string(&path).expect("Failed to read file");
    assert!(content.contains("public"));
    assert!(!content.contains("internal"));
    assert!(content.contains("42"));

    let loaded_manager = ConfigManager::<ConfigWithSkip>::create()
        .with_path(&path)
        .build()
        .expect("Failed to load config");

    let loaded = loaded_manager.get();
    assert_eq!(loaded.public_field, "public");
    assert_eq!(loaded.internal_field, "");
    assert_eq!(loaded.another_field, 42);
}

#[test]
fn test_config_merging() {
    let path_toml = test_file_path("test_merge.toml");

    let mut manager_v1 = ConfigManager::<ConfigV1>::create()
        .with_path(&path_toml)
        .build()
        .expect("Failed to create v1 manager");
    manager_v1.get_mut().field1 = "value1".to_string();
    manager_v1.get_mut().field2 = 100;
    manager_v1.save().expect("Failed to save v1 config");

    let manager_v2 = ConfigManager::<ConfigV2>::create()
        .with_path(&path_toml)
        .build()
        .expect("Failed to load as v2");
    let loaded = manager_v2.get();
    assert_eq!(loaded.field1, "value1");
    assert_eq!(loaded.field2, 100);
    assert_eq!(loaded.field3, false);
    assert_eq!(loaded.field4, "");

    manager_v2.save().expect("Failed to save merged config");
    let content = fs::read_to_string(&path_toml).expect("Failed to read file");
    assert!(content.contains("field1"));
    assert!(content.contains("field2"));
    assert!(content.contains("field3"));
    assert!(content.contains("field4"));

    #[cfg(feature = "json")]
    {
        let path_json = test_file_path("test_merge.json");
        let mut manager_v1 = ConfigManager::<ConfigV1>::create()
            .with_path(&path_json)
            .build()
            .expect("Failed to create JSON v1 manager");
        manager_v1.get_mut().field1 = "json_value1".to_string();
        manager_v1.get_mut().field2 = 200;
        manager_v1.save().expect("Failed to save JSON v1 config");

        let manager_v2 = ConfigManager::<ConfigV2>::create()
            .with_path(&path_json)
            .build()
            .expect("Failed to load JSON as v2");
        let loaded = manager_v2.get();
        assert_eq!(loaded.field1, "json_value1");
        assert_eq!(loaded.field2, 200);
        assert_eq!(loaded.field3, false);
        assert_eq!(loaded.field4, "");

        manager_v2
            .save()
            .expect("Failed to save merged JSON config");
        let content = fs::read_to_string(&path_json).expect("Failed to read JSON file");
        assert!(content.contains("json_value1"));
        assert!(content.contains("field3"));
        assert!(content.contains("field4"));
    }

    #[cfg(feature = "yaml")]
    {
        let path_yaml = test_file_path("test_merge.yaml");

        let mut manager_v1 = ConfigManager::<ConfigV1>::create()
            .with_path(&path_yaml)
            .build()
            .expect("Failed to create YAML v1 manager");
        manager_v1.get_mut().field1 = "yaml_value1".to_string();
        manager_v1.get_mut().field2 = 300;
        manager_v1.save().expect("Failed to save YAML v1 config");

        let manager_v2 = ConfigManager::<ConfigV2>::create()
            .with_path(&path_yaml)
            .build()
            .expect("Failed to load YAML as v2");
        let loaded = manager_v2.get();
        assert_eq!(loaded.field1, "yaml_value1");
        assert_eq!(loaded.field2, 300);
        assert_eq!(loaded.field3, false);
        assert_eq!(loaded.field4, "");
        manager_v2
            .save()
            .expect("Failed to save merged YAML config");

        let content = fs::read_to_string(&path_yaml).expect("Failed to read YAML file");
        assert!(content.contains("yaml_value1"));
        assert!(content.contains("field3"));
        assert!(content.contains("field4"));
    }
}

#[test]
fn test_remove_orphans() {
    let path_toml = test_file_path("test_orphans.toml");

    let mut manager_v2 = ConfigManager::<ConfigV2>::create()
        .with_path(&path_toml)
        .build()
        .expect("Failed to create v2 manager");

    manager_v2.get_mut().field1 = "value1".to_string();
    manager_v2.get_mut().field2 = 100;
    manager_v2.get_mut().field3 = true;
    manager_v2.get_mut().field4 = "extra".to_string();
    manager_v2.save().expect("Failed to save v2 config");

    let manager_v1 = ConfigManager::<ConfigV1>::create()
        .with_path(&path_toml)
        .with_remove_orphans(true)
        .build()
        .expect("Failed to load as v1");

    manager_v1.save().expect("Failed to save config");

    let content = fs::read_to_string(&path_toml).expect("Failed to read file");
    assert!(content.contains("field1"));
    assert!(content.contains("field2"));
    assert!(!content.contains("field3"));
    assert!(!content.contains("field4"));

    #[cfg(feature = "json")]
    {
        let path_json = test_file_path("test_orphans.json");

        let mut manager_v2 = ConfigManager::<ConfigV2>::create()
            .with_path(&path_json)
            .build()
            .expect("Failed to create JSON v2 manager");

        manager_v2.get_mut().field1 = "json_value1".to_string();
        manager_v2.get_mut().field2 = 200;
        manager_v2.get_mut().field3 = true;
        manager_v2.get_mut().field4 = "json_extra".to_string();
        manager_v2.save().expect("Failed to save JSON v2 config");

        let manager_v1 = ConfigManager::<ConfigV1>::create()
            .with_path(&path_json)
            .with_remove_orphans(true)
            .build()
            .expect("Failed to load JSON as v1");

        manager_v1.save().expect("Failed to save JSON config");

        let content = fs::read_to_string(&path_json).expect("Failed to read JSON file");
        assert!(content.contains("json_value1"));
        assert!(!content.contains("field3"));
        assert!(!content.contains("json_extra"));
    }

    #[cfg(feature = "yaml")]
    {
        let path_yaml = test_file_path("test_orphans.yaml");

        let mut manager_v2 = ConfigManager::<ConfigV2>::create()
            .with_path(&path_yaml)
            .build()
            .expect("Failed to create YAML v2 manager");

        manager_v2.get_mut().field1 = "yaml_value1".to_string();
        manager_v2.get_mut().field2 = 300;
        manager_v2.get_mut().field3 = false;
        manager_v2.get_mut().field4 = "yaml_extra".to_string();
        manager_v2.save().expect("Failed to save YAML v2 config");

        let manager_v1 = ConfigManager::<ConfigV1>::create()
            .with_path(&path_yaml)
            .with_remove_orphans(true)
            .build()
            .expect("Failed to load YAML as v1");

        manager_v1.save().expect("Failed to save YAML config");

        let content = fs::read_to_string(&path_yaml).expect("Failed to read YAML file");
        assert!(content.contains("yaml_value1"));
        assert!(!content.contains("field3"));
        assert!(!content.contains("yaml_extra"));
    }
}

#[test]
fn test_update_and_save() {
    let path = test_file_path("test_update.toml");

    let mut manager = ConfigManager::<TestConfig>::create()
        .with_path(&path)
        .build()
        .expect("Failed to create manager");

    manager
        .update(|cfg| {
            cfg.name = "Updated".to_string();
            cfg.age = 30;
            cfg.enabled = true;
        })
        .expect("Failed to update config");

    let reloaded = ConfigManager::<TestConfig>::create()
        .with_path(&path)
        .build()
        .expect("Failed to reload");

    assert_eq!(reloaded.get().name, "Updated");
    assert_eq!(reloaded.get().age, 30);
    assert_eq!(reloaded.get().enabled, true);
}

#[test]
fn test_reload() {
    let path = test_file_path("test_reload.toml");

    let mut manager = ConfigManager::<TestConfig>::create()
        .with_path(&path)
        .build()
        .expect("Failed to create manager");

    manager.get_mut().name = "Original".to_string();
    manager.get_mut().age = 25;
    manager.get_mut().enabled = true;

    manager.save().expect("Failed to save");

    let modified_content = r#"
name = "Modified"
age = 35
enabled = false
"#;
    fs::write(&path, modified_content).expect("Failed to write modified content");

    manager.reload().expect("Failed to reload");

    assert_eq!(manager.get().name, "Modified");
    assert_eq!(manager.get().age, 35);
    assert_eq!(manager.get().enabled, false);
}

#[test]
fn test_comments_in_output() {
    let path_toml = test_file_path("test_comments.toml");

    let mut manager = ConfigManager::<TestConfig>::create()
        .with_path(&path_toml)
        .build()
        .expect("Failed to create manager");

    manager.get_mut().name = "Test".to_string();
    manager.get_mut().age = 25;
    manager.get_mut().enabled = true;

    manager.save().expect("Failed to save config");

    let content = fs::read_to_string(&path_toml).expect("Failed to read file");

    assert!(content.contains("Test Configuration Header"));
    assert!(content.contains("Second line of header"));
    assert!(content.contains("String field"));
    assert!(content.contains("Number field"));
    assert!(content.contains("Boolean field"));

    #[cfg(feature = "json")]
    {
        let path_json = test_file_path("test_comments.json");

        let mut manager = ConfigManager::<TestConfig>::create()
            .with_path(&path_json)
            .build()
            .expect("Failed to create JSON manager");

        manager.get_mut().name = "JsonTest".to_string();
        manager.get_mut().age = 30;
        manager.get_mut().enabled = false;

        manager.save().expect("Failed to save JSON config");

        let content = fs::read_to_string(&path_json).expect("Failed to read JSON file");

        assert!(content.contains("Test Configuration Header"));
        assert!(content.contains("Second line of header"));
        assert!(content.contains("String field"));
        assert!(content.contains("Number field"));
        assert!(content.contains("Boolean field"));
    }

    #[cfg(feature = "yaml")]
    {
        let path_yaml = test_file_path("test_comments.yaml");

        let mut manager = ConfigManager::<TestConfig>::create()
            .with_path(&path_yaml)
            .build()
            .expect("Failed to create YAML manager");

        manager.get_mut().name = "YamlTest".to_string();
        manager.get_mut().age = 35;
        manager.get_mut().enabled = true;

        manager.save().expect("Failed to save YAML config");

        let content = fs::read_to_string(&path_yaml).expect("Failed to read YAML file");

        assert!(content.contains("Test Configuration Header"));
        assert!(content.contains("Second line of header"));
        assert!(content.contains("String field"));
        assert!(content.contains("Number field"));
        assert!(content.contains("Boolean field"));
    }
}

#[test]
fn test_env_merge() {
    with_vars(
        [("TEST_HOST", Some("env-host")), ("TEST_PORT", Some("9000"))],
        || {
            let path = test_file_path("test_env_merge.toml");

            #[derive(Config, Serialize, Deserialize, Default, Debug, PartialEq)]
            struct ConfigV1WithEnv {
                #[env("TEST_HOST")]
                #[comment("Host from env")]
                host: String,
                #[comment("Regular field")]
                name: String,
            }

            #[derive(Config, Serialize, Deserialize, Default, Debug, PartialEq)]
            struct ConfigV2WithEnv {
                #[env("TEST_HOST")]
                #[comment("Host from env")]
                host: String,
                #[comment("Regular field")]
                name: String,
                #[env("TEST_PORT")]
                #[comment("New port field with env")]
                port: u16,
                #[comment("New regular field")]
                extra: String,
            }

            let mut manager_v1 = ConfigManager::<ConfigV1WithEnv>::create()
                .with_path(&path)
                .build()
                .expect("Failed to create v1 manager");

            manager_v1.get_mut().host = "old-host".to_string();
            manager_v1.get_mut().name = "test-name".to_string();
            manager_v1.save().expect("Failed to save v1 config");

            let manager_v2 = ConfigManager::<ConfigV2WithEnv>::create()
                .with_path(&path)
                .build()
                .expect("Failed to load as v2");

            assert_eq!(manager_v2.get().host, "env-host");
            assert_eq!(manager_v2.get().name, "test-name");
            assert_eq!(manager_v2.get().port, 9000);
            assert_eq!(manager_v2.get().extra, "");

            manager_v2.save().expect("Failed to save merged config");

            let reloaded = ConfigManager::<ConfigV2WithEnv>::create()
                .with_path(&path)
                .build()
                .expect("Failed to reload");

            assert_eq!(reloaded.get().host, "env-host");
            assert_eq!(reloaded.get().name, "test-name");
            assert_eq!(reloaded.get().port, 9000);
        },
    );
}
