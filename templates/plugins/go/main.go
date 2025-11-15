package main

import (
	"fmt"
)

// Execute a tool call. This is the primary entry point for tool execution in plugins.
//
// The plugin receives a tool call request with the tool name and arguments, along with request context information. The plugin should execute the requested tool and return the result with content blocks and optional structured output.
// It takes CallToolRequest as input ()
// And returns CallToolResult ()
func CallTool(input CallToolRequest) (*CallToolResult, error) {
	return nil, fmt.Errorf("CallTool not implemented.")
}

// Provide completion suggestions for a partially-typed input.
//
// This function is called when the user requests autocompletion. The plugin should analyze the partial input and return matching completion suggestions based on the reference (prompt or resource) and argument context.
// It takes CompleteRequest as input ()
// And returns CompleteResult ()
func Complete(input CompleteRequest) (*CompleteResult, error) {
	return &CompleteResult{}, nil
}

// Retrieve a specific prompt by name.
//
// This function is called when the user requests a specific prompt. The plugin should return the prompt details including messages and optional description.
// It takes GetPromptRequest as input ()
// And returns GetPromptResult ()
func GetPrompt(input GetPromptRequest) (*GetPromptResult, error) {
	// TODO: fill out your implementation here
	return nil, fmt.Errorf("GetPrompt not implemented.")
}

// List all available prompts.
//
// This function should return a list of prompts that the plugin provides. Each prompt should include its name and a brief description of what it does. Supports pagination via cursor.
// It takes ListPromptsRequest as input ()
// And returns ListPromptsResult ()
func ListPrompts(input ListPromptsRequest) (*ListPromptsResult, error) {
	// TODO: fill out your implementation here
	return &ListPromptsResult{}, nil
}

// List all available resource templates.
//
// This function should return a list of resource templates that the plugin provides. Templates are URI patterns that can match multiple resources. Supports pagination via cursor.
// It takes ListResourceTemplatesRequest as input ()
// And returns ListResourceTemplatesResult ()
func ListResourceTemplates(input ListResourceTemplatesRequest) (*ListResourceTemplatesResult, error) {
	// TODO: fill out your implementation here
	return &ListResourceTemplatesResult{}, nil
}

// List all available resources.
//
// This function should return a list of resources that the plugin provides. Resources are URI-based references to files, data, or services. Supports pagination via cursor.
// It takes ListResourcesRequest as input ()
// And returns ListResourcesResult ()
func ListResources(input ListResourcesRequest) (*ListResourcesResult, error) {
	// TODO: fill out your implementation here
	return &ListResourcesResult{}, nil
}

// List all available tools.
//
// This function should return a list of all tools that the plugin provides. Each tool should include its name, description, and input schema. Supports pagination via cursor.
// It takes ListToolsRequest as input ()
// And returns ListToolsResult ()
func ListTools(input ListToolsRequest) (*ListToolsResult, error) {
	// TODO: fill out your implementation here
	return &ListToolsResult{}, nil
}

// Notification that the list of roots has changed.
//
// This is an optional notification handler. If implemented, the plugin will be notified whenever the roots list changes on the client side. This allows plugins to react to changes in the file system roots or other root resources.
// It takes PluginNotificationContext as input ()
func OnRootsListChanged(input PluginNotificationContext) error {
	// TODO: fill out your implementation here
	return nil
}

// Read the contents of a resource by its URI.
//
// This function is called when the user wants to read the contents of a specific resource. The plugin should retrieve and return the resource data with appropriate MIME type information.
// It takes ReadResourceRequest as input ()
// And returns ReadResourceResult ()
func ReadResource(input ReadResourceRequest) (*ReadResourceResult, error) {
	// TODO: fill out your implementation here
	return nil, fmt.Errorf("ReadResource not implemented.")
}

// Note: leave this in place, as the Go compiler will find the `export` function as the entrypoint.
func main() {}
