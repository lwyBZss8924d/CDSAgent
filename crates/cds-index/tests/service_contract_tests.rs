//! Service Contract Tests for JSON-RPC Schema Validation
//!
//! These tests ensure that CDS-Index Service responses conform to the
//! JSON-RPC schema defined in docs/api/jsonrpc-schema.json
//!
//! Test Strategy:
//! 1. Schema validation - responses match expected structure
//! 2. Error format validation - errors follow JSON-RPC 2.0 spec
//! 3. Type safety - TypeScript bindings match Rust serialization
//! 4. Backward compatibility - older clients can parse new responses

use jsonschema::{Draft, JSONSchema};
use serde_json::{json, Value};
use std::sync::OnceLock;

/// Load the JSON-RPC schema from docs/api/jsonrpc-schema.json
/// Schema is embedded at compile time via include_str!
fn load_jsonrpc_schema() -> &'static Value {
    static SCHEMA: OnceLock<Value> = OnceLock::new();
    SCHEMA.get_or_init(|| {
        // Embed schema at compile time
        let schema_content = include_str!("../../../docs/api/jsonrpc-schema.json");
        serde_json::from_str(schema_content)
            .expect("Failed to parse embedded jsonrpc-schema.json as valid JSON")
    })
}

/// Validate an entity against the entity schema definition
/// Creates an inline schema that includes the entity definition with resolved references
fn validate_entity_schema(entity: &Value) -> Result<(), String> {
    // Instead of using $ref, create an inline schema for entity validation
    let entity_schema = json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "required": ["id", "name", "entity_type", "file_path", "line_range", "score"],
        "properties": {
            "id": {
                "type": "string",
                "description": "Unique entity identifier"
            },
            "name": {
                "type": "string",
                "description": "Entity name"
            },
            "entity_type": {
                "type": "string",
                "enum": ["directory", "file", "class", "function"]
            },
            "file_path": {
                "type": "string"
            },
            "line_range": {
                "type": "array",
                "items": {
                    "type": "integer",
                    "minimum": 1
                },
                "minItems": 2,
                "maxItems": 2
            },
            "score": {
                "type": "number",
                "minimum": 0,
                "maximum": 1
            },
            "snippet": {
                "type": "object",
                "required": ["fold"],
                "properties": {
                    "fold": {
                        "type": "string"
                    },
                    "preview": {
                        "type": "string"
                    },
                    "full": {
                        "type": "string"
                    }
                }
            }
        }
    });

    let compiled = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&entity_schema)
        .map_err(|e| format!("Failed to compile entity schema: {}", e))?;

    let validation_result = compiled.validate(entity);
    match validation_result {
        Ok(()) => Ok(()),
        Err(errors) => {
            let error_msgs: Vec<String> = errors
                .map(|e| format!("{} at {}", e, e.instance_path))
                .collect();
            Err(format!("Entity validation failed:\n  {}", error_msgs.join("\n  ")))
        }
    }
}

/// Test helper to validate JSON-RPC response format
fn validate_jsonrpc_response(response: &Value) -> Result<(), String> {
    // Check required fields
    if !response.is_object() {
        return Err("Response must be an object".into());
    }

    let obj = response.as_object().unwrap();

    // Validate jsonrpc field
    match obj.get("jsonrpc") {
        Some(Value::String(v)) if v == "2.0" => {}
        _ => return Err("Missing or invalid 'jsonrpc' field (must be '2.0')".into()),
    }

    // Validate id field (must be string, number, or null)
    match obj.get("id") {
        Some(Value::String(_)) | Some(Value::Number(_)) | Some(Value::Null) => {}
        None => return Err("Missing 'id' field".into()),
        _ => return Err("Invalid 'id' field type".into()),
    }

    // Must have either 'result' or 'error', but not both
    let has_result = obj.contains_key("result");
    let has_error = obj.contains_key("error");

    match (has_result, has_error) {
        (true, false) => Ok(()),
        (false, true) => validate_error_format(obj.get("error").unwrap()),
        (true, true) => Err("Response cannot have both 'result' and 'error'".into()),
        (false, false) => Err("Response must have either 'result' or 'error'".into()),
    }
}

/// Validate JSON-RPC error format
fn validate_error_format(error: &Value) -> Result<(), String> {
    if !error.is_object() {
        return Err("Error must be an object".into());
    }

    let obj = error.as_object().unwrap();

    // Validate code field (must be integer)
    match obj.get("code") {
        Some(Value::Number(n)) if n.is_i64() => {}
        _ => return Err("Error must have 'code' field (integer)".into()),
    }

    // Validate message field (must be string)
    match obj.get("message") {
        Some(Value::String(_)) => {}
        _ => return Err("Error must have 'message' field (string)".into()),
    }

    // data field is optional, no validation needed

    Ok(())
}

#[cfg(test)]
mod schema_validation_tests {
    use super::*;

    #[test]
    fn test_valid_success_response() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "entities": [],
                "total_count": 0
            }
        });

        assert!(validate_jsonrpc_response(&response).is_ok());
    }

    #[test]
    fn test_valid_error_response() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32001,
                "message": "Index not found",
                "data": {
                    "index_path": "/path/to/index"
                }
            }
        });

        assert!(validate_jsonrpc_response(&response).is_ok());
    }

    #[test]
    fn test_missing_jsonrpc_field() {
        let response = json!({
            "id": 1,
            "result": {}
        });

        assert!(validate_jsonrpc_response(&response).is_err());
    }

    #[test]
    fn test_wrong_jsonrpc_version() {
        let response = json!({
            "jsonrpc": "1.0",
            "id": 1,
            "result": {}
        });

        assert!(validate_jsonrpc_response(&response).is_err());
    }

    #[test]
    fn test_missing_id_field() {
        let response = json!({
            "jsonrpc": "2.0",
            "result": {}
        });

        assert!(validate_jsonrpc_response(&response).is_err());
    }

    #[test]
    fn test_both_result_and_error() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {},
            "error": {
                "code": -32001,
                "message": "Error"
            }
        });

        assert!(validate_jsonrpc_response(&response).is_err());
    }

    #[test]
    fn test_error_missing_code() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "message": "Error"
            }
        });

        assert!(validate_jsonrpc_response(&response).is_err());
    }

    #[test]
    fn test_error_missing_message() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32001
            }
        });

        assert!(validate_jsonrpc_response(&response).is_err());
    }
}

#[cfg(test)]
mod entity_schema_tests {
    use super::*;

    #[test]
    fn test_entity_structure() {
        let entity = json!({
            "id": "test_entity_123",
            "name": "test_function",
            "entity_type": "function",
            "file_path": "src/test.py",
            "line_range": [10, 20],
            "score": 0.95,
            "snippet": {
                "fold": "def test_function():",
                "preview": "def test_function():\n    return True",
                "full": "def test_function():\n    \"\"\"Test function\"\"\"\n    return True"
            }
        });

        // Validate required fields
        assert!(entity.get("id").is_some());
        assert!(entity.get("name").is_some());
        assert!(entity.get("entity_type").is_some());
        assert!(entity.get("file_path").is_some());
        assert!(entity.get("line_range").is_some());
        assert!(entity.get("score").is_some());

        // Validate entity_type enum
        let entity_type = entity.get("entity_type").unwrap().as_str().unwrap();
        assert!(["directory", "file", "class", "function"].contains(&entity_type));

        // Validate line_range is array of 2 integers
        let line_range = entity.get("line_range").unwrap().as_array().unwrap();
        assert_eq!(line_range.len(), 2);
        assert!(line_range[0].is_number());
        assert!(line_range[1].is_number());

        // Validate score is between 0 and 1
        let score = entity.get("score").unwrap().as_f64().unwrap();
        assert!(score >= 0.0 && score <= 1.0);

        // Validate snippet structure
        let snippet = entity.get("snippet").unwrap();
        assert!(snippet.get("fold").is_some());
        assert!(snippet.get("preview").is_some());
        assert!(snippet.get("full").is_some());
    }

    #[test]
    fn test_entity_type_enum_values() {
        let valid_types = vec!["directory", "file", "class", "function"];

        for entity_type in valid_types {
            let entity = json!({
                "id": "test",
                "name": "test",
                "entity_type": entity_type,
                "file_path": "test.py",
                "line_range": [1, 10],
                "score": 1.0,
                "snippet": {
                    "fold": "test"
                }
            });

            assert_eq!(
                entity.get("entity_type").unwrap().as_str().unwrap(),
                entity_type
            );

            // Validate against JSON schema
            assert!(
                validate_entity_schema(&entity).is_ok(),
                "Entity type {} failed schema validation",
                entity_type
            );
        }
    }

    #[test]
    fn test_entity_validates_against_schema() {
        // Full entity with all fields
        let entity = json!({
            "id": "src.utils::sanitize",
            "name": "sanitize",
            "entity_type": "function",
            "file_path": "src/utils.py",
            "line_range": [10, 20],
            "score": 0.95,
            "snippet": {
                "fold": "def sanitize(text):",
                "preview": "def sanitize(text):\n    return clean(text)",
                "full": "def sanitize(text):\n    \"\"\"Clean input\"\"\"\n    return clean(text)"
            }
        });

        assert!(
            validate_entity_schema(&entity).is_ok(),
            "Full entity should validate against schema"
        );
    }

    #[test]
    fn test_entity_with_fold_only_snippet() {
        // Entity with only fold in snippet (snippet_mode='fold')
        let entity = json!({
            "id": "test_id",
            "name": "test_func",
            "entity_type": "function",
            "file_path": "test.py",
            "line_range": [1, 5],
            "score": 0.8,
            "snippet": {
                "fold": "def test_func():"
            }
        });

        match validate_entity_schema(&entity) {
            Ok(()) => {}
            Err(e) => panic!("Entity with fold-only snippet should validate. Error: {}", e),
        }
    }

    #[test]
    fn test_entity_with_fold_and_preview_snippet() {
        // Entity with fold+preview (snippet_mode='preview')
        let entity = json!({
            "id": "test_id",
            "name": "test_func",
            "entity_type": "function",
            "file_path": "test.py",
            "line_range": [1, 5],
            "score": 0.8,
            "snippet": {
                "fold": "def test_func():",
                "preview": "def test_func():\n    pass"
            }
        });

        assert!(
            validate_entity_schema(&entity).is_ok(),
            "Entity with fold+preview snippet should validate"
        );
    }

    #[test]
    fn test_entity_missing_fold_fails() {
        // Entity missing required fold field
        let entity = json!({
            "id": "test_id",
            "name": "test_func",
            "entity_type": "function",
            "file_path": "test.py",
            "line_range": [1, 5],
            "score": 0.8,
            "snippet": {
                "preview": "def test_func():\n    pass"
            }
        });

        assert!(
            validate_entity_schema(&entity).is_err(),
            "Entity without fold should fail validation"
        );
    }
}

#[cfg(test)]
mod search_entities_contract_tests {
    use super::*;

    #[test]
    fn test_search_entities_response_structure() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "entities": [
                    {
                        "id": "entity_1",
                        "name": "sanitize_input",
                        "entity_type": "function",
                        "file_path": "src/utils.py",
                        "line_range": [10, 25],
                        "score": 0.95,
                        "snippet": {
                            "fold": "def sanitize_input(text):",
                            "preview": "def sanitize_input(text):\n    return clean(text)",
                            "full": "def sanitize_input(text):\n    \"\"\"Clean user input\"\"\"\n    return clean(text)"
                        }
                    }
                ],
                "total_count": 1,
                "query_metadata": {
                    "used_upper_index": true,
                    "used_bm25": false,
                    "execution_time_ms": 120
                }
            }
        });

        // Validate JSON-RPC structure
        assert!(validate_jsonrpc_response(&response).is_ok());

        // Validate result structure
        let result = response.get("result").unwrap();
        assert!(result.get("entities").is_some());
        assert!(result.get("total_count").is_some());
        assert!(result.get("query_metadata").is_some());

        // Validate query_metadata structure
        let metadata = result.get("query_metadata").unwrap();
        assert!(metadata.get("used_upper_index").unwrap().is_boolean());
        assert!(metadata.get("used_bm25").unwrap().is_boolean());
        assert!(metadata.get("execution_time_ms").unwrap().is_number());
    }

    #[test]
    fn test_search_entities_empty_results() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "entities": [],
                "total_count": 0,
                "query_metadata": {
                    "used_upper_index": false,
                    "used_bm25": true,
                    "execution_time_ms": 50
                }
            }
        });

        assert!(validate_jsonrpc_response(&response).is_ok());

        let result = response.get("result").unwrap();
        let entities = result.get("entities").unwrap().as_array().unwrap();
        assert_eq!(entities.len(), 0);
        assert_eq!(result.get("total_count").unwrap().as_i64().unwrap(), 0);
    }
}

#[cfg(test)]
mod traverse_graph_contract_tests {
    use super::*;

    #[test]
    fn test_traverse_graph_response_structure() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "result": {
                "subgraph": {
                    "nodes": [
                        {
                            "id": "node_1",
                            "name": "function_a",
                            "entity_type": "function",
                            "file_path": "src/a.py",
                            "line_range": [10, 20],
                            "depth": 0
                        }
                    ],
                    "edges": [
                        {
                            "source": "node_1",
                            "target": "node_2",
                            "relation": "invoke"
                        }
                    ]
                },
                "metadata": {
                    "total_nodes": 1,
                    "total_edges": 1,
                    "max_depth_reached": 1,
                    "execution_time_ms": 200
                }
            }
        });

        assert!(validate_jsonrpc_response(&response).is_ok());

        let result = response.get("result").unwrap();
        assert!(result.get("subgraph").is_some());
        assert!(result.get("metadata").is_some());

        // Validate subgraph structure
        let subgraph = result.get("subgraph").unwrap();
        assert!(subgraph.get("nodes").unwrap().is_array());
        assert!(subgraph.get("edges").unwrap().is_array());

        // Validate relation enum
        let edge = subgraph.get("edges").unwrap().as_array().unwrap()[0].clone();
        let relation = edge.get("relation").unwrap().as_str().unwrap();
        assert!(["contain", "import", "invoke", "inherit"].contains(&relation));
    }
}

#[cfg(test)]
mod retrieve_entity_contract_tests {
    use super::*;

    #[test]
    fn test_retrieve_entity_response_structure() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 3,
            "result": {
                "entities": [
                    {
                        "id": "entity_1",
                        "name": "MyClass",
                        "entity_type": "class",
                        "file_path": "src/models.py",
                        "line_range": [10, 50],
                        "code": "class MyClass:\n    def __init__(self):\n        pass",
                        "metadata": {
                            "parameters": ["self"],
                            "docstring": "My class documentation",
                            "decorators": ["dataclass"]
                        }
                    }
                ]
            }
        });

        assert!(validate_jsonrpc_response(&response).is_ok());

        let result = response.get("result").unwrap();
        let entities = result.get("entities").unwrap().as_array().unwrap();
        assert!(entities.len() > 0);

        let entity = &entities[0];
        assert!(entity.get("code").is_some());
        assert!(entity.get("metadata").is_some());
    }
}

#[cfg(test)]
mod error_contract_tests {
    use super::*;

    #[test]
    fn test_index_not_found_error() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32001,
                "message": "Index not found",
                "data": {
                    "index_path": "/path/to/missing/index",
                    "suggestion": "Run 'cds init <repo>' to create an index"
                }
            }
        });

        assert!(validate_jsonrpc_response(&response).is_ok());

        let error = response.get("error").unwrap();
        assert_eq!(error.get("code").unwrap().as_i64().unwrap(), -32001);
        assert_eq!(
            error.get("message").unwrap().as_str().unwrap(),
            "Index not found"
        );
        assert!(error.get("data").is_some());
    }

    #[test]
    fn test_entity_not_found_error() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "error": {
                "code": -32002,
                "message": "Entity not found",
                "data": {
                    "entity_id": "nonexistent_id"
                }
            }
        });

        assert!(validate_jsonrpc_response(&response).is_ok());
        assert_eq!(
            response.get("error").unwrap().get("code").unwrap().as_i64().unwrap(),
            -32002
        );
    }

    #[test]
    fn test_parse_error_code() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 3,
            "error": {
                "code": -32003,
                "message": "Parse error",
                "data": {
                    "file_path": "src/malformed.py",
                    "line": 42,
                    "error_message": "Unexpected token"
                }
            }
        });

        assert!(validate_jsonrpc_response(&response).is_ok());
    }

    #[test]
    fn test_query_timeout_error() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 4,
            "error": {
                "code": -32004,
                "message": "Query timeout",
                "data": {
                    "timeout_ms": 5000,
                    "query": "complex search"
                }
            }
        });

        assert!(validate_jsonrpc_response(&response).is_ok());
    }

    #[test]
    fn test_standard_jsonrpc_errors() {
        let error_codes = vec![
            (-32700, "Parse error"),
            (-32600, "Invalid Request"),
            (-32601, "Method not found"),
            (-32602, "Invalid params"),
            (-32603, "Internal error"),
        ];

        for (code, message) in error_codes {
            let response = json!({
                "jsonrpc": "2.0",
                "id": 1,
                "error": {
                    "code": code,
                    "message": message
                }
            });

            assert!(
                validate_jsonrpc_response(&response).is_ok(),
                "Failed for error code {}",
                code
            );
        }
    }
}

#[cfg(test)]
mod backward_compatibility_tests {
    use super::*;

    #[test]
    fn test_old_client_ignores_new_fields() {
        // v0.2.0 response with new optional field
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "entities": [],
                "total_count": 0,
                "query_metadata": {
                    "used_upper_index": false,
                    "used_bm25": true,
                    "execution_time_ms": 100
                },
                // NEW field in v0.2.0
                "cache_hit": true
            }
        });

        // v0.1.0 client should still be able to parse this
        assert!(validate_jsonrpc_response(&response).is_ok());

        // Old client reads only known fields
        let result = response.get("result").unwrap();
        assert!(result.get("entities").is_some());
        assert!(result.get("total_count").is_some());
        // New field is present but can be safely ignored
        assert!(result.get("cache_hit").is_some());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_cli_output_is_valid_json() {
        // Simulate CLI output that should be parseable by jq
        let cli_output = json!({
            "query": "sanitize",
            "total_results": 1,
            "results": [
                {
                    "entity_id": "entity_1",
                    "name": "sanitize_input",
                    "type": "function",
                    "file_path": "src/utils.py",
                    "line_range": [10, 20],
                    "score": 0.95
                }
            ]
        });

        // Should be parseable by jq
        let json_str = serde_json::to_string_pretty(&cli_output).unwrap();
        assert!(serde_json::from_str::<Value>(&json_str).is_ok());
    }
}

#[cfg(test)]
mod embedded_schema_validation_tests {
    use super::*;

    /// Compile schema validator for a specific method result by inlining all definitions
    /// This avoids $ref resolution issues by creating a self-contained schema
    fn compile_method_result_validator(method_name: &str) -> JSONSchema {
        let schema = load_jsonrpc_schema();
        let method_result = schema
            .get("methods")
            .and_then(|m| m.get(method_name))
            .and_then(|m| m.get("result"))
            .expect(&format!("{} result schema not found", method_name));

        // Get all definitions to inline
        let definitions = schema.get("definitions").expect("definitions not found");

        // Create a new schema with the method result as root and all definitions
        let inline_schema = json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "definitions": definitions,
            "type": method_result.get("type"),
            "required": method_result.get("required"),
            "properties": method_result.get("properties")
        });

        JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&inline_schema)
            .expect(&format!("Failed to compile {} result schema", method_name))
    }

    fn compile_search_result_validator() -> JSONSchema {
        compile_method_result_validator("search_entities")
    }

    fn compile_traverse_result_validator() -> JSONSchema {
        compile_method_result_validator("traverse_graph")
    }

    fn compile_retrieve_result_validator() -> JSONSchema {
        compile_method_result_validator("retrieve_entity")
    }

    #[test]
    fn test_search_entities_fixture_validates() {
        // Load actual fixture (embedded at compile time)
        let fixture = include_str!("../../../tests/fixtures/api/search-response.json");
        let response: Value = serde_json::from_str(fixture)
            .expect("Failed to parse search-response.json");

        // Validate JSON-RPC wrapper
        assert!(validate_jsonrpc_response(&response).is_ok());

        // Compile schema and validate result against actual schema
        let validator = compile_search_result_validator();
        let result = response.get("result").unwrap();

        let validation_result = validator.validate(result);
        if let Err(errors) = validation_result {
            let error_messages: Vec<String> = errors
                .map(|e| format!("{} at {}", e, e.instance_path))
                .collect();
            panic!(
                "search-response.json failed schema validation:\n{}",
                error_messages.join("\n")
            );
        }

        // Additionally validate each entity against inline entity schema
        // (this catches entity-level regressions)
        let entities = result.get("entities").unwrap().as_array().unwrap();
        for entity in entities {
            assert!(
                validate_entity_schema(entity).is_ok(),
                "Entity in search response failed inline entity schema validation"
            );
        }
    }

    #[test]
    fn test_traverse_graph_fixture_validates() {
        let fixture = include_str!("../../../tests/fixtures/api/traverse-response.json");
        let response: Value = serde_json::from_str(fixture)
            .expect("Failed to parse traverse-response.json");

        // Validate JSON-RPC wrapper
        assert!(validate_jsonrpc_response(&response).is_ok());

        // Compile schema and validate result against actual schema
        let validator = compile_traverse_result_validator();
        let result = response.get("result").unwrap();

        let validation_result = validator.validate(result);
        if let Err(errors) = validation_result {
            let error_messages: Vec<String> = errors
                .map(|e| format!("{} at {}", e, e.instance_path))
                .collect();
            panic!(
                "traverse-response.json failed schema validation:\n{}",
                error_messages.join("\n")
            );
        }
    }

    #[test]
    fn test_retrieve_entity_fixture_validates() {
        // Create a synthetic retrieve_entity response for validation
        let response = json!({
            "jsonrpc": "2.0",
            "id": 3,
            "result": {
                "entities": [
                    {
                        "id": "entity_abc",
                        "name": "sanitize_html",
                        "entity_type": "function",
                        "file_path": "src/utils.py",
                        "line_range": [15, 32],
                        "code": "def sanitize_html(input):\n    return input.strip()",
                        "metadata": {
                            "docstring": "Sanitize HTML input",
                            "parameters": ["input"],
                            "return_type": "str"
                        }
                    }
                ]
            }
        });

        // Validate JSON-RPC wrapper
        assert!(validate_jsonrpc_response(&response).is_ok());

        // Compile schema and validate result against actual schema
        let validator = compile_retrieve_result_validator();
        let result = response.get("result").unwrap();

        let validation_result = validator.validate(result);
        if let Err(errors) = validation_result {
            let error_messages: Vec<String> = errors
                .map(|e| format!("{} at {}", e, e.instance_path))
                .collect();
            panic!(
                "retrieve_entity response failed schema validation:\n{}",
                error_messages.join("\n")
            );
        }
    }

    #[test]
    fn test_error_response_fixture_validates() {
        let fixture = include_str!("../../../tests/fixtures/api/error-index-not-found.json");
        let response: Value = serde_json::from_str(fixture)
            .expect("Failed to parse error-index-not-found.json");

        // Validate JSON-RPC error format
        assert!(validate_jsonrpc_response(&response).is_ok());

        // Verify error structure matches schema
        let error = response.get("error").unwrap();

        // Create inline error schema (JSON-RPC 2.0 standard error object)
        let error_schema = json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "required": ["code", "message"],
            "properties": {
                "code": {
                    "type": "integer",
                    "description": "Error code (standard or custom)"
                },
                "message": {
                    "type": "string",
                    "description": "Human-readable error message"
                },
                "data": {
                    "type": "object",
                    "description": "Additional error data (optional)"
                }
            }
        });

        let validator = JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&error_schema)
            .expect("Failed to compile error schema");

        let validation_result = validator.validate(error);
        if let Err(errors) = validation_result {
            let error_messages: Vec<String> = errors
                .map(|e| format!("{} at {}", e, e.instance_path))
                .collect();
            panic!(
                "error-index-not-found.json failed schema validation:\n{}",
                error_messages.join("\n")
            );
        }

        // Verify error code matches documented custom error
        assert_eq!(error.get("code").unwrap().as_i64().unwrap(), -32001);
        assert_eq!(
            error.get("message").unwrap().as_str().unwrap(),
            "Index not found"
        );
    }

    #[test]
    fn test_schema_drift_detection() {
        // This test ensures the embedded schema is parseable and has expected structure
        let schema = load_jsonrpc_schema();

        // Verify top-level schema structure
        assert!(schema.get("$schema").is_some());
        assert!(schema.get("definitions").is_some());
        assert!(schema.get("methods").is_some());
        assert!(schema.get("errors").is_some());

        // Verify all 4 methods are defined
        let methods = schema.get("methods").unwrap();
        assert!(methods.get("search_entities").is_some());
        assert!(methods.get("traverse_graph").is_some());
        assert!(methods.get("retrieve_entity").is_some());
        assert!(methods.get("rebuild_index").is_some());

        // Verify entity definition exists
        let entity_def = schema
            .get("definitions")
            .and_then(|d| d.get("entity"));
        assert!(entity_def.is_some(), "Entity definition missing from schema");

        // Verify snippet is properly defined with fold as required
        let snippet_def = entity_def
            .unwrap()
            .get("properties")
            .and_then(|p| p.get("snippet"));
        assert!(snippet_def.is_some());

        let snippet_required = snippet_def
            .unwrap()
            .get("required")
            .and_then(|r| r.as_array());
        assert!(snippet_required.is_some());
        assert_eq!(
            snippet_required.unwrap().len(),
            1,
            "Snippet should only require 'fold' field"
        );
    }
}
