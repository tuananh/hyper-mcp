package main

import (
	pdk "github.com/extism/go-pdk"
)

//export call_tool
func _CallTool() int32 {
	var err error
	_ = err
	pdk.Log(pdk.LogDebug, "CallTool: getting JSON input")
	var input CallToolRequest
	err = pdk.InputJSON(&input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "CallTool: calling implementation function")
	output, err := CallTool(input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	if output == nil {
		pdk.SetErrorString("CallTool: output is nil")
		return -1
	}

	pdk.Log(pdk.LogDebug, "CallTool: setting JSON output")
	err = pdk.OutputJSON(output)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "CallTool: returning")
	return 0
}

//export complete
func _Complete() int32 {
	var err error
	_ = err
	pdk.Log(pdk.LogDebug, "Complete: getting JSON input")
	var input CompleteRequest
	err = pdk.InputJSON(&input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "Complete: calling implementation function")
	output, err := Complete(input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	if output == nil {
		pdk.SetErrorString("Complete: output is nil")
		return -1
	}

	pdk.Log(pdk.LogDebug, "Complete: setting JSON output")
	err = pdk.OutputJSON(output)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "Complete: returning")
	return 0
}

//export get_prompt
func _GetPrompt() int32 {
	var err error
	_ = err
	pdk.Log(pdk.LogDebug, "GetPrompt: getting JSON input")
	var input GetPromptRequest
	err = pdk.InputJSON(&input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "GetPrompt: calling implementation function")
	output, err := GetPrompt(input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	if output == nil {
		pdk.SetErrorString("GetPrompt: output is nil")
		return -1
	}

	pdk.Log(pdk.LogDebug, "GetPrompt: setting JSON output")
	err = pdk.OutputJSON(output)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "GetPrompt: returning")
	return 0
}

//export list_prompts
func _ListPrompts() int32 {
	var err error
	_ = err
	pdk.Log(pdk.LogDebug, "ListPrompts: getting JSON input")
	var input ListPromptsRequest
	err = pdk.InputJSON(&input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "ListPrompts: calling implementation function")
	output, err := ListPrompts(input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	if output == nil {
		pdk.SetErrorString("ListPrompts: output is nil")
		return -1
	}

	pdk.Log(pdk.LogDebug, "ListPrompts: setting JSON output")
	err = pdk.OutputJSON(output)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "ListPrompts: returning")
	return 0
}

//export list_resource_templates
func _ListResourceTemplates() int32 {
	var err error
	_ = err
	pdk.Log(pdk.LogDebug, "ListResourceTemplates: getting JSON input")
	var input ListResourceTemplatesRequest
	err = pdk.InputJSON(&input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "ListResourceTemplates: calling implementation function")
	output, err := ListResourceTemplates(input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	if output == nil {
		pdk.SetErrorString("ListResourceTemplates: output is nil")
		return -1
	}

	pdk.Log(pdk.LogDebug, "ListResourceTemplates: setting JSON output")
	err = pdk.OutputJSON(output)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "ListResourceTemplates: returning")
	return 0
}

//export list_resources
func _ListResources() int32 {
	var err error
	_ = err
	pdk.Log(pdk.LogDebug, "ListResources: getting JSON input")
	var input ListResourcesRequest
	err = pdk.InputJSON(&input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "ListResources: calling implementation function")
	output, err := ListResources(input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	if output == nil {
		pdk.SetErrorString("ListResources: output is nil")
		return -1
	}

	pdk.Log(pdk.LogDebug, "ListResources: setting JSON output")
	err = pdk.OutputJSON(output)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "ListResources: returning")
	return 0
}

//export list_tools
func _ListTools() int32 {
	var err error
	_ = err
	pdk.Log(pdk.LogDebug, "ListTools: getting JSON input")
	var input ListToolsRequest
	err = pdk.InputJSON(&input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "ListTools: calling implementation function")
	output, err := ListTools(input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	if output == nil {
		pdk.SetErrorString("ListTools: output is nil")
		return -1
	}

	pdk.Log(pdk.LogDebug, "ListTools: setting JSON output")
	err = pdk.OutputJSON(output)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "ListTools: returning")
	return 0
}

//export on_roots_list_changed
func _OnRootsListChanged() int32 {
	var err error
	_ = err
	pdk.Log(pdk.LogDebug, "OnRootsListChanged: getting JSON input")
	var input PluginNotificationContext
	err = pdk.InputJSON(&input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "OnRootsListChanged: calling implementation function")
	err = OnRootsListChanged(input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "OnRootsListChanged: returning")
	return 0
}

//export read_resource
func _ReadResource() int32 {
	var err error
	_ = err
	pdk.Log(pdk.LogDebug, "ReadResource: getting JSON input")
	var input ReadResourceRequest
	err = pdk.InputJSON(&input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "ReadResource: calling implementation function")
	output, err := ReadResource(input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	if output == nil {
		pdk.SetErrorString("ReadResource: output is nil")
		return -1
	}

	pdk.Log(pdk.LogDebug, "ReadResource: setting JSON output")
	err = pdk.OutputJSON(output)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "ReadResource: returning")
	return 0
}
