use serde_json::json;
use std::{collections::HashMap, ffi::OsString};

use crate::config_file::ArgumentConfig;

/// Export arguments to environment variables.
pub fn to_env_record(
    args: &Vec<ArgumentConfig>,
    matches: &clap::ArgMatches,
) -> HashMap<OsString, OsString> {
    let mut map = HashMap::new();

    for arg in args {
        if let Some(value) = matches.get_one::<String>(&arg.id) {
            map.insert(arg.id.clone().into(), value.clone().into());
        }
    }

    map
}

/// Export arguments to JSON.
#[allow(dead_code)]
fn to_json_record(args: &Vec<ArgumentConfig>, matches: &clap::ArgMatches) -> serde_json::Value {
    let mut map = serde_json::Map::new();

    for arg in args {
        // TODO: Add support for types other than `String`.
        if let Some(value) = matches.get_one::<String>(&arg.id) {
            map.insert(arg.id.clone(), value.clone().into());
        }
    }

    json!(map)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::config_file::CommandConfig;

    #[test]
    fn test_env_record_exporter() {
        let config = CommandConfig {
            args: Some(vec![
                ArgumentConfig {
                    id: "arg1".into(),
                    ..Default::default()
                },
                ArgumentConfig {
                    id: "arg2".into(),
                    ..Default::default()
                },
            ]),
            ..CommandConfig::default()
        };

        let command: clap::Command = config.clone().into();
        let matches = command.get_matches_from(vec!["test", "value1", "value2"]);
        let env = to_env_record(&config.args.unwrap_or_default(), &matches);

        assert_eq!(
            env,
            HashMap::from_iter(vec![
                ("arg1".into(), "value1".into()),
                ("arg2".into(), "value2".into()),
            ])
        );
    }

    #[test]
    fn test_json_record_exporter() {
        let config = CommandConfig {
            args: Some(vec![
                ArgumentConfig {
                    id: "arg1".into(),
                    ..Default::default()
                },
                ArgumentConfig {
                    id: "arg2".into(),
                    ..Default::default()
                },
            ]),
            ..CommandConfig::default()
        };

        let command: clap::Command = config.clone().into();
        let matches = command.get_matches_from(vec!["test", "value1", "value2"]);
        let payload = to_json_record(&config.args.unwrap_or_default(), &matches);

        assert_eq!(
            payload,
            json!({
                "arg1": "value1",
                "arg2": "value2",
            })
        );
    }
}
