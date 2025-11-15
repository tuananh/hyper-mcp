package main

import pdk "github.com/extism/go-pdk"

// CreateElicitation Request user input through the client's elicitation interface.
//
// Plugins can use this to ask users for input, decisions, or confirmations. This is useful for interactive plugins that need user guidance during tool execution. Returns the user's response with action and optional form data.
// It takes input of CreateElicitationRequestParamWithTimeout ()
// And it returns an output *CreateElicitationResult ()
func CreateElicitation(input ElicitRequestParamWithTimeout) (*ElicitResult, error) {
	var err error
	_ = err
	mem, err := pdk.AllocateJSON(&input)
	if err != nil {
		return nil, err
	}

	offs := _CreateElicitation(mem.Offset())

	var out ElicitResult
	err = pdk.JSONFrom(offs, &out)
	if err != nil {
		return nil, err
	}
	return &out, nil

}

// CreateMessage Request message creation through the client's sampling interface.
//
// Plugins can use this to have the client create messages, typically with AI assistance. This is used when plugins need intelligent text generation or analysis. Returns the generated message with model information.
// It takes input of CreateMessageRequestParam ()
// And it returns an output *CreateMessageResult ()
func CreateMessage(input CreateMessageRequestParam) (*CreateMessageResult, error) {
	var err error
	_ = err
	mem, err := pdk.AllocateJSON(&input)
	if err != nil {
		return nil, err
	}

	offs := _CreateMessage(mem.Offset())

	var out CreateMessageResult
	err = pdk.JSONFrom(offs, &out)
	if err != nil {
		return nil, err
	}
	return &out, nil

}

// ListRoots List the client's root directories or resources.
//
// Plugins can query this to discover what root resources (typically file system roots) are available on the client side. This helps plugins understand the scope of resources they can access.
// And it returns an output *ListRootsResult ()
func ListRoots() (*ListRootsResult, error) {
	var err error
	_ = err
	offs := _ListRoots()

	var out ListRootsResult
	err = pdk.JSONFrom(offs, &out)
	if err != nil {
		return nil, err
	}
	return &out, nil

}

// NotifyLoggingMessage Send a logging message to the client.
//
// Plugins use this to report diagnostic, informational, warning, or error messages. The client's logging level determines which messages are processed.
// It takes input of LoggingMessageNotificationParam ()
func NotifyLoggingMessage(input LoggingMessageNotificationParam) error {
	var err error
	_ = err
	mem, err := pdk.AllocateJSON(&input)
	if err != nil {
		return err
	}

	_NotifyLoggingMessage(mem.Offset())

	return nil

}

// NotifyProgress Send a progress notification to the client.
//
// Plugins use this to report progress during long-running operations. This allows clients to display progress bars or status information to users.
// It takes input of ProgressNotificationParam ()
func NotifyProgress(input ProgressNotificationParam) error {
	var err error
	_ = err
	mem, err := pdk.AllocateJSON(&input)
	if err != nil {
		return err
	}

	_NotifyProgress(mem.Offset())

	return nil

}

// NotifyPromptListChanged Notify the client that the list of available prompts has changed.
//
// Plugins should call this when they add, remove, or modify their available prompts. The client will typically refresh its prompt list in response.
func NotifyPromptListChanged() error {
	var err error
	_ = err
	_NotifyPromptListChanged()

	return nil

}

// NotifyResourceListChanged Notify the client that the list of available resources has changed.
//
// Plugins should call this when they add, remove, or modify their available resources. The client will typically refresh its resource list in response.
func NotifyResourceListChanged() error {
	var err error
	_ = err
	_NotifyResourceListChanged()

	return nil

}

// NotifyResourceUpdated Notify the client that a specific resource has been updated.
//
// Plugins should call this when they modify the contents of a resource. The client can use this to invalidate caches and refresh resource displays.
// It takes input of ResourceUpdatedNotificationParam ()
func NotifyResourceUpdated(input ResourceUpdatedNotificationParam) error {
	var err error
	_ = err
	mem, err := pdk.AllocateJSON(&input)
	if err != nil {
		return err
	}

	_NotifyResourceUpdated(mem.Offset())

	return nil

}

// NotifyToolListChanged Notify the client that the list of available tools has changed.
//
// Plugins should call this when they add, remove, or modify their available tools. The client will typically refresh its tool list in response.
func NotifyToolListChanged() error {
	var err error
	_ = err
	_NotifyToolListChanged()

	return nil

}

//go:wasmimport extism:host/user create_elicitation
func _CreateElicitation(uint64) uint64

//go:wasmimport extism:host/user create_message
func _CreateMessage(uint64) uint64

//go:wasmimport extism:host/user list_roots
func _ListRoots() uint64

//go:wasmimport extism:host/user notify_logging_message
func _NotifyLoggingMessage(uint64)

//go:wasmimport extism:host/user notify_progress
func _NotifyProgress(uint64)

//go:wasmimport extism:host/user notify_prompt_list_changed
func _NotifyPromptListChanged()

//go:wasmimport extism:host/user notify_resource_list_changed
func _NotifyResourceListChanged()

//go:wasmimport extism:host/user notify_resource_updated
func _NotifyResourceUpdated(uint64)

//go:wasmimport extism:host/user notify_tool_list_changed
func _NotifyToolListChanged()
