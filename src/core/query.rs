use crate::core::errors::Error;
use jmespath::Expression;
use serde_json::Value;

type Result<T> = std::result::Result<T, Error>;

pub struct QueryEngine {
    expression: Expression<'static>,
}

impl QueryEngine {
    pub fn new(query: &str) -> Result<Self> {
        let expression = jmespath::compile(query)
            .map_err(|e| Error::other(format!("Invalid query syntax: {e}")))?;

        Ok(Self { expression })
    }

    pub fn search(&self, data: &Value) -> Result<Value> {
        let result = self
            .expression
            .search(data)
            .map_err(|e| Error::other(format!("Query execution failed: {e}")))?;

        variable_to_json(result.as_ref())
    }

    pub fn compile(query: &str) -> Result<Expression<'static>> {
        jmespath::compile(query).map_err(|e| Error::other(format!("Query compilation failed: {e}")))
    }

    pub fn apply(query: &str, data: &Value) -> Result<Value> {
        let expr = Self::compile(query)?;
        let result = expr
            .search(data)
            .map_err(|e| Error::other(format!("Query failed: {e}")))?;

        variable_to_json(result.as_ref())
    }
}

fn variable_to_json(var: &jmespath::Variable) -> Result<Value> {
    match var {
        jmespath::Variable::Null => Ok(Value::Null),
        jmespath::Variable::Bool(b) => Ok(Value::Bool(*b)),
        jmespath::Variable::Number(n) => serde_json::to_value(n)
            .map_err(|e| Error::other(format!("Failed to convert number: {e}"))),
        jmespath::Variable::String(s) => Ok(Value::String(s.clone())),
        jmespath::Variable::Array(arr) => {
            let values: Result<Vec<Value>> = arr.iter().map(|v| variable_to_json(v)).collect();
            Ok(Value::Array(values?))
        }
        jmespath::Variable::Object(obj) => {
            let map: Result<serde_json::Map<String, Value>> = obj
                .iter()
                .map(|(k, v)| variable_to_json(v).map(|val| (k.clone(), val)))
                .collect();
            Ok(Value::Object(map?))
        }
        jmespath::Variable::Expref(_) => Err(Error::other(
            "Expression references not supported in output".to_string(),
        )),
    }
}

pub fn validate_query(query: &str) -> Result<()> {
    jmespath::compile(query)
        .map(|_| ())
        .map_err(|e| Error::other(format!("Invalid query: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_simple_query() {
        let data = json!({
            "name": "test",
            "value": 42
        });

        let result = QueryEngine::apply("name", &data).unwrap();
        assert_eq!(result, json!("test"));
    }

    #[test]
    fn test_projection() {
        let data = json!({
            "items": [
                {"name": "a", "value": 1},
                {"name": "b", "value": 2}
            ]
        });

        let result = QueryEngine::apply("items[*].name", &data).unwrap();
        assert_eq!(result, json!(["a", "b"]));
    }

    #[test]
    fn test_invalid_query() {
        let result = validate_query("invalid[[query");
        assert!(result.is_err());
    }
}
