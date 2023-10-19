use serde::{Serialize, Deserialize};
use serde_json::json;
use openai_function_definitions::generate_function_schema; // replace `your_macro_crate` with the name of your macro crate

#[generate_function_schema]
fn get_current_weather(location: &str, unit: Option<&str>) -> serde_json::Value {
    json!({
        "location": location,
        "temperature": "70",
        "unit": unit.unwrap_or("fahrenheit"),
        "forecast": ["sunny", "windy"],
    })
}

#[test]
fn test_generate_function_schema() {
    print!("{}", get_current_weather::SCHEMA);

    let functions = vec![
        ChatCompletionFunctionDefinition {
            name: "get_current_weather".into(),
            description: Some("Get the current weather in a given location".into()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA",
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"]
                    },
                },
                "required": ["location"],
            })),
        },
    ];

    // This would be the main part of your test where you validate the output
    // For example, if your macro adds the `functions` definition to the module:
    assert_eq!(functions[0].name, "get_current_weather".to_string());
    // ... Add more assertions as necessary
}

// Assuming ChatCompletionFunctionDefinition looks something like this:
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ChatCompletionFunctionDefinition {
    name: String,
    description: Option<String>,
    parameters: Option<serde_json::Value>,
}
