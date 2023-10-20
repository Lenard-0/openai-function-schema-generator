# openai-function-schema-generator
A Rust library that provides simple macros to put above functions that automatically generates the schema definition for Open AI GPT function calling.


Example:

From This

#[generate_function_schema]

 fn get_current_weather(location: &str, unit: Option<&str>) -> serde_json::Value {
 
     json!({
         "location": location,
         "temperature": "70",
         "unit": unit.unwrap_or("fahrenheit"),
         "forecast": ["sunny", "windy"],
     })
 })

 Generate This Boilerplate Behind The Scenes
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
