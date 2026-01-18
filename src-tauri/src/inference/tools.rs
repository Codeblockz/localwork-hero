use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::files::operations;
use crate::files::PermissionStore;

/// A tool call parsed from LLM output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

/// Tool definition for the LLM
#[derive(Debug, Clone, Serialize)]
pub struct ToolDefinition {
    pub name: &'static str,
    pub description: &'static str,
    pub parameters: Value,
}

/// Get all available file operation tools
pub fn get_file_tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "list_files",
            description: "List files and directories in a given path",
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Absolute path to the directory to list"
                    }
                },
                "required": ["path"]
            }),
        },
        ToolDefinition {
            name: "read_file",
            description: "Read the contents of a text file",
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Absolute path to the file to read"
                    }
                },
                "required": ["path"]
            }),
        },
        ToolDefinition {
            name: "write_file",
            description: "Write content to an existing file (overwrites)",
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Absolute path to the file to write"
                    },
                    "content": {
                        "type": "string",
                        "description": "Content to write to the file"
                    }
                },
                "required": ["path", "content"]
            }),
        },
        ToolDefinition {
            name: "create_file",
            description: "Create a new file with content (fails if file already exists)",
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Absolute path for the new file"
                    },
                    "content": {
                        "type": "string",
                        "description": "Content for the new file"
                    }
                },
                "required": ["path", "content"]
            }),
        },
        ToolDefinition {
            name: "delete_file",
            description: "Delete a file",
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Absolute path to the file to delete"
                    }
                },
                "required": ["path"]
            }),
        },
        ToolDefinition {
            name: "move_file",
            description: "Move or rename a file",
            parameters: json!({
                "type": "object",
                "properties": {
                    "src": {
                        "type": "string",
                        "description": "Absolute path to the source file"
                    },
                    "dest": {
                        "type": "string",
                        "description": "Absolute path for the destination"
                    }
                },
                "required": ["src", "dest"]
            }),
        },
    ]
}

/// Format tool definitions as a string for the system prompt
pub fn format_tools_for_prompt() -> String {
    let tools = get_file_tools();
    let tools_json: Vec<Value> = tools
        .iter()
        .map(|t| {
            json!({
                "name": t.name,
                "description": t.description,
                "parameters": t.parameters
            })
        })
        .collect();

    format!(
        r#"You have access to the following tools to help users with file operations:

{}

To use a tool, respond with a tool call in this exact format:
<tool_call>{{"name": "tool_name", "arguments": {{"arg1": "value1"}}}}</tool_call>

You can use multiple tool calls in a single response. After each tool call, you will receive the result.
Only use tools when the user asks for file operations. Always provide a natural language response along with your tool calls."#,
        serde_json::to_string_pretty(&tools_json).unwrap_or_default()
    )
}

/// Execute a tool call and return the result
pub fn execute_tool(store: &PermissionStore, tool_call: &ToolCall) -> String {
    match tool_call.name.as_str() {
        "list_files" => {
            let path = tool_call.arguments.get("path").and_then(|v| v.as_str());
            match path {
                Some(p) => match operations::list_directory(store, p) {
                    Ok(files) => {
                        let file_list: Vec<String> = files
                            .iter()
                            .map(|f| {
                                let type_indicator = if f.is_directory { "[DIR]" } else { "[FILE]" };
                                format!("{} {} ({})", type_indicator, f.name, f.path)
                            })
                            .collect();
                        if file_list.is_empty() {
                            "Directory is empty".to_string()
                        } else {
                            file_list.join("\n")
                        }
                    }
                    Err(e) => format!("Error: {}", e),
                },
                None => "Error: Missing 'path' argument".to_string(),
            }
        }
        "read_file" => {
            let path = tool_call.arguments.get("path").and_then(|v| v.as_str());
            match path {
                Some(p) => match operations::read_file(store, p) {
                    Ok(content) => content,
                    Err(e) => format!("Error: {}", e),
                },
                None => "Error: Missing 'path' argument".to_string(),
            }
        }
        "write_file" => {
            let path = tool_call.arguments.get("path").and_then(|v| v.as_str());
            let content = tool_call.arguments.get("content").and_then(|v| v.as_str());
            match (path, content) {
                (Some(p), Some(c)) => match operations::write_file(store, p, c) {
                    Ok(()) => format!("Successfully wrote to {}", p),
                    Err(e) => format!("Error: {}", e),
                },
                _ => "Error: Missing 'path' or 'content' argument".to_string(),
            }
        }
        "create_file" => {
            let path = tool_call.arguments.get("path").and_then(|v| v.as_str());
            let content = tool_call.arguments.get("content").and_then(|v| v.as_str());
            match (path, content) {
                (Some(p), Some(c)) => match operations::create_file(store, p, c) {
                    Ok(()) => format!("Successfully created {}", p),
                    Err(e) => format!("Error: {}", e),
                },
                _ => "Error: Missing 'path' or 'content' argument".to_string(),
            }
        }
        "delete_file" => {
            let path = tool_call.arguments.get("path").and_then(|v| v.as_str());
            match path {
                Some(p) => match operations::delete_file(store, p) {
                    Ok(()) => format!("Successfully deleted {}", p),
                    Err(e) => format!("Error: {}", e),
                },
                None => "Error: Missing 'path' argument".to_string(),
            }
        }
        "move_file" => {
            let src = tool_call.arguments.get("src").and_then(|v| v.as_str());
            let dest = tool_call.arguments.get("dest").and_then(|v| v.as_str());
            match (src, dest) {
                (Some(s), Some(d)) => match operations::move_file(store, s, d) {
                    Ok(()) => format!("Successfully moved {} to {}", s, d),
                    Err(e) => format!("Error: {}", e),
                },
                _ => "Error: Missing 'src' or 'dest' argument".to_string(),
            }
        }
        _ => format!("Error: Unknown tool '{}'", tool_call.name),
    }
}

/// Parse tool calls from LLM output
pub fn parse_tool_calls(text: &str) -> Vec<ToolCall> {
    let mut tool_calls = Vec::new();
    let mut id_counter = 0;

    // Find all <tool_call>...</tool_call> blocks
    let mut remaining = text;
    while let Some(start) = remaining.find("<tool_call>") {
        let after_start = &remaining[start + 11..];
        if let Some(end) = after_start.find("</tool_call>") {
            let json_str = &after_start[..end].trim();

            // Parse the JSON inside the tags
            if let Ok(parsed) = serde_json::from_str::<Value>(json_str) {
                if let (Some(name), Some(arguments)) = (
                    parsed.get("name").and_then(|v| v.as_str()),
                    parsed.get("arguments"),
                ) {
                    tool_calls.push(ToolCall {
                        id: format!("call_{}", id_counter),
                        name: name.to_string(),
                        arguments: arguments.clone(),
                        result: None,
                    });
                    id_counter += 1;
                }
            }
            remaining = &after_start[end + 12..];
        } else {
            break;
        }
    }

    tool_calls
}

/// Extract the text content from LLM output, excluding tool calls
pub fn extract_text_content(text: &str) -> String {
    let mut result = text.to_string();

    // Remove all <tool_call>...</tool_call> blocks
    while let Some(start) = result.find("<tool_call>") {
        if let Some(end) = result[start..].find("</tool_call>") {
            result = format!("{}{}", &result[..start], &result[start + end + 12..]);
        } else {
            break;
        }
    }

    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_tool_call() {
        let text = r#"Let me list the files for you.
<tool_call>{"name": "list_files", "arguments": {"path": "/tmp/test"}}</tool_call>"#;

        let calls = parse_tool_calls(text);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].name, "list_files");
        assert_eq!(calls[0].arguments.get("path").unwrap(), "/tmp/test");
    }

    #[test]
    fn test_parse_multiple_tool_calls() {
        let text = r#"I'll read both files.
<tool_call>{"name": "read_file", "arguments": {"path": "/tmp/a.txt"}}</tool_call>
<tool_call>{"name": "read_file", "arguments": {"path": "/tmp/b.txt"}}</tool_call>"#;

        let calls = parse_tool_calls(text);
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].arguments.get("path").unwrap(), "/tmp/a.txt");
        assert_eq!(calls[1].arguments.get("path").unwrap(), "/tmp/b.txt");
    }

    #[test]
    fn test_parse_no_tool_calls() {
        let text = "This is just a regular response with no tools.";
        let calls = parse_tool_calls(text);
        assert_eq!(calls.len(), 0);
    }

    #[test]
    fn test_extract_text_content() {
        let text = r#"Let me list those files.
<tool_call>{"name": "list_files", "arguments": {"path": "/tmp"}}</tool_call>
Here are the results."#;

        let content = extract_text_content(text);
        assert_eq!(content, "Let me list those files.\n\nHere are the results.");
    }

    #[test]
    fn test_format_tools_for_prompt() {
        let prompt = format_tools_for_prompt();
        assert!(prompt.contains("list_files"));
        assert!(prompt.contains("read_file"));
        assert!(prompt.contains("write_file"));
        assert!(prompt.contains("<tool_call>"));
    }
}
