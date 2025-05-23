package main

import (
	"errors"

	pdk "github.com/extism/go-pdk"
)

//export call
func _Call() int32 {
	var err error
	_ = err
	pdk.Log(pdk.LogDebug, "Call: getting JSON input")
	var input CallToolRequest
	err = pdk.InputJSON(&input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "Call: calling implementation function")
	output, err := Call(input)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "Call: setting JSON output")
	err = pdk.OutputJSON(output)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "Call: returning")
	return 0
}

//export describe
func _Describe() int32 {
	var err error
	_ = err
	output, err := Describe()
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "Describe: setting JSON output")
	err = pdk.OutputJSON(output)
	if err != nil {
		pdk.SetError(err)
		return -1
	}

	pdk.Log(pdk.LogDebug, "Describe: returning")
	return 0
}

type BlobResourceContents struct {
	// A base64-encoded string representing the binary data of the item.
	Blob string `json:"blob"`
	// The MIME type of this resource, if known.
	MimeType *string `json:"mimeType,omitempty"`
	// The URI of this resource.
	Uri string `json:"uri"`
}

// Used by the client to invoke a tool provided by the server.
type CallToolRequest struct {
	Method *string `json:"method,omitempty"`
	Params Params  `json:"params"`
}

// The server's response to a tool call.
//
// Any errors that originate from the tool SHOULD be reported inside the result
// object, with `isError` set to true, _not_ as an MCP protocol-level error
// response. Otherwise, the LLM would not be able to see that an error occurred
// and self-correct.
//
// However, any errors in _finding_ the tool, an error indicating that the
// server does not support tool calls, or any other exceptional conditions,
// should be reported as an MCP error response.
type CallToolResult struct {
	Content []Content `json:"content"`
	// Whether the tool call ended in an error.
	//
	// If not set, this is assumed to be false (the call was successful).
	IsError *bool `json:"isError,omitempty"`
}

// A content response.
// For text content set type to ContentType.Text and set the `text` property
// For image content set type to ContentType.Image and set the `data` and `mimeType` properties
type Content struct {
	Annotations *TextAnnotation `json:"annotations,omitempty"`
	// The base64-encoded image data.
	Data *string `json:"data,omitempty"`
	// The MIME type of the image. Different providers may support different image types.
	MimeType *string `json:"mimeType,omitempty"`
	// The text content of the message.
	Text *string     `json:"text,omitempty"`
	Type ContentType `json:"type"`
}

type ContentType string

const (
	ContentTypeText     ContentType = "text"
	ContentTypeImage    ContentType = "image"
	ContentTypeResource ContentType = "resource"
)

func (v ContentType) String() string {
	switch v {
	case ContentTypeText:
		return `text`
	case ContentTypeImage:
		return `image`
	case ContentTypeResource:
		return `resource`
	default:
		return ""
	}
}

func stringToContentType(s string) (ContentType, error) {
	switch s {
	case `text`:
		return ContentTypeText, nil
	case `image`:
		return ContentTypeImage, nil
	case `resource`:
		return ContentTypeResource, nil
	default:
		return ContentType(""), errors.New("unable to convert string to ContentType")
	}
}

// Provides one or more descriptions of the tools available in this servlet.
type ListToolsResult struct {
	// The list of ToolDescription objects provided by this servlet.
	Tools []ToolDescription `json:"tools"`
}

type Params struct {
	Arguments interface{} `json:"arguments,omitempty"`
	Name      string      `json:"name"`
}

// The sender or recipient of messages and data in a conversation.
type Role string

const (
	RoleAssistant Role = "assistant"
	RoleUser      Role = "user"
)

func (v Role) String() string {
	switch v {
	case RoleAssistant:
		return `assistant`
	case RoleUser:
		return `user`
	default:
		return ""
	}
}

func stringToRole(s string) (Role, error) {
	switch s {
	case `assistant`:
		return RoleAssistant, nil
	case `user`:
		return RoleUser, nil
	default:
		return Role(""), errors.New("unable to convert string to Role")
	}
}

// A text annotation
type TextAnnotation struct {
	// Describes who the intended customer of this object or data is.
	//
	// It can include multiple entries to indicate content useful for multiple audiences (e.g., `["user", "assistant"]`).
	Audience []Role `json:"audience,omitempty"`
	// Describes how important this data is for operating the server.
	//
	// A value of 1 means "most important," and indicates that the data is
	// effectively required, while 0 means "least important," and indicates that
	// the data is entirely optional.
	Priority float32 `json:"priority,omitempty"`
}

type TextResourceContents struct {
	// The MIME type of this resource, if known.
	MimeType *string `json:"mimeType,omitempty"`
	// The text of the item. This must only be set if the item can actually be represented as text (not binary data).
	Text string `json:"text"`
	// The URI of this resource.
	Uri string `json:"uri"`
}

// Describes the capabilities and expected paramters of the tool function
type ToolDescription struct {
	// A description of the tool
	Description string `json:"description"`
	// The JSON schema describing the argument input
	InputSchema interface{} `json:"inputSchema"`
	// The name of the tool. It should match the plugin / binding name.
	Name string `json:"name"`
}

// Note: leave this in place, as the Go compiler will find the `export` function as the entrypoint.
func main() {}
