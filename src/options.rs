use serde::{Deserialize, Serialize};
use serde_json::Value;

pub trait TemplateOptions
where
    for<'de> Self: Serialize + Deserialize<'de> + Default,
{
    fn parse(s: &str) -> anyhow::Result<Self> {
        let mut map: serde_json::Map<String, serde_json::Value> =
            serde_json::from_value(serde_json::to_value(Self::default())?)?;

        for entry in s.split(",") {
            let (key, value) = entry
                .split_once("=")
                .ok_or_else(|| anyhow::anyhow!("expected `key = value` found {entry}"))?;

            let key = key.trim();
            let value = value.trim();

            let existing = map
                .get(key)
                .ok_or_else(|| anyhow::anyhow!("Invalid option: {key} in {entry}."))?;

            let parsed = coerce_value(value, existing)?;
            map.insert(key.to_string(), parsed);
        }

        Ok(serde_json::from_value(serde_json::to_value(map)?)?)
    }
}

pub fn coerce_value(raw: &str, existing: &Value) -> anyhow::Result<Value> {
    match existing {
        Value::Bool(_) => raw
            .parse::<bool>()
            .map(Value::Bool)
            .map_err(|_| anyhow::anyhow!("expected a boolean, found `{raw}`")),

        Value::Number(_) => raw
            .parse::<f64>()
            .ok()
            .and_then(serde_json::Number::from_f64)
            .map(Value::Number)
            .ok_or_else(|| anyhow::anyhow!("expected a number, found `{raw}`")),

        Value::String(_) => Ok(Value::String(raw.to_string())),

        _ => serde_json::from_str(raw).or_else(|_| Ok(Value::String(raw.to_string()))),
    }
}
