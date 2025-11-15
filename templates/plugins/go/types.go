package main

import (
	"encoding/json"
	"fmt"
	"time"
)

// Annotations represents metadata annotations for resources and content
type Annotations struct {
	Audience     []Role     `json:"audience,omitempty"`
	LastModified *time.Time `json:"lastModified,omitempty"`
	Priority     float32    `json:"priority,omitempty"`
}

// AudioContent represents audio content in a message
type AudioContent struct {
	Meta        Meta         `json:"_meta,omitempty"`
	Annotations *Annotations `json:"annotations,omitempty"`
	Data        string       `json:"data"`
	MimeType    string       `json:"mimeType"`
}

func (a AudioContent) MarshalJSON() ([]byte, error) {
	type alias AudioContent
	return json.Marshal(&struct {
		Type string `json:"type"`
		alias
	}{
		Type:  "audio",
		alias: (alias)(a),
	})
}

func (a *AudioContent) UnmarshalJSON(data []byte) error {
	type alias AudioContent
	aux := struct {
		Type string `json:"type"`
		alias
	}{}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	// Optional: validate `type`
	if aux.Type != "audio" && aux.Type != "" { // allow empty if missing
		return fmt.Errorf("invalid type %q, expected \"audio\"", aux.Type)
	}

	*a = AudioContent(aux.alias)
	return nil
}

// BlobResourceContents represents binary resource contents
type BlobResourceContents struct {
	Meta     Meta    `json:"_meta,omitempty"`
	Blob     string  `json:"blob"`
	MimeType *string `json:"mimeType,omitempty"`
	URI      string  `json:"uri"`
}

// BooleanSchema represents a boolean input schema
type BooleanSchema struct {
	Default     *bool   `json:"default,omitempty"`
	Description *string `json:"description,omitempty"`
	Title       *string `json:"title,omitempty"`
}

func (b BooleanSchema) MarshalJSON() ([]byte, error) {
	type alias BooleanSchema
	return json.Marshal(&struct {
		Type string `json:"type"`
		alias
	}{
		Type:  "boolean",
		alias: (alias)(b),
	})
}

func (b *BooleanSchema) UnmarshalJSON(data []byte) error {
	type alias BooleanSchema
	aux := struct {
		Type string `json:"type"`
		alias
	}{}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	// Optional: validate `type`
	if aux.Type != "boolean" && aux.Type != "" { // allow empty if missing
		return fmt.Errorf("invalid type %q, expected \"boolean\"", aux.Type)
	}

	*b = BooleanSchema(aux.alias)
	return nil
}

// CallToolRequest represents a request to call a tool
type CallToolRequest struct {
	Context PluginRequestContext `json:"context"`
	Request CallToolRequestParam `json:"request"`
}

// CallToolRequestParam represents parameters for calling a tool
type CallToolRequestParam struct {
	Arguments map[string]any `json:"arguments,omitempty"`
	Name      string         `json:"name"`
}

// CallToolResult represents the result of calling a tool
type CallToolResult struct {
	Meta              Meta           `json:"_meta,omitempty"`
	Content           []ContentBlock `json:"content"`
	IsError           *bool          `json:"isError,omitempty"`
	StructuredContent map[string]any `json:"structuredContent,omitempty"`
}

// CompleteRequest represents a request for completion suggestions
type CompleteRequest struct {
	Context PluginRequestContext `json:"context"`
	Request CompleteRequestParam `json:"request"`
}

// CompleteRequestParam represents parameters for completion
type CompleteRequestParam struct {
	Argument CompleteRequestParamArgument `json:"argument"`
	Context  *CompleteRequestParamContext `json:"context,omitempty"`
	Ref      Reference                    `json:"ref"`
}

// CompleteRequestParamArgument represents an argument for completion
type CompleteRequestParamArgument struct {
	Name  string `json:"name"`
	Value string `json:"value"`
}

// CompleteRequestParamContext represents context for completion
type CompleteRequestParamContext struct {
	Arguments map[string]string `json:"arguments,omitempty"`
}

// CompleteResult represents completion suggestions
type CompleteResult struct {
	Completion CompleteResultCompletion `json:"completion"`
}

// CompleteResultCompletion represents completion values
type CompleteResultCompletion struct {
	HasMore *bool    `json:"hasMore,omitempty"`
	Total   *int64   `json:"total,omitempty"`
	Values  []string `json:"values"`
}

type ContentBlock struct {
	Audio            *AudioContent
	EmbeddedResource *EmbeddedResource
	Image            *ImageContent
	ResourceLink     *ResourceLinkContent
	Text             *TextContent
}

func (c ContentBlock) MarshalJSON() ([]byte, error) {
	switch {
	case c.Audio != nil:
		return json.Marshal(c.Audio)
	case c.EmbeddedResource != nil:
		return json.Marshal(c.EmbeddedResource)
	case c.Image != nil:
		return json.Marshal(c.Image)
	case c.ResourceLink != nil:
		return json.Marshal(c.ResourceLink)
	case c.Text != nil:
		return json.Marshal(c.Text)
	default:
		return nil, fmt.Errorf("empty ContentItem")
	}
}

func (c *ContentBlock) UnmarshalJSON(data []byte) error {
	var head struct {
		Type string `json:"type"`
	}
	if err := json.Unmarshal(data, &head); err != nil {
		return err
	}

	switch head.Type {
	case "audio":
		var a AudioContent
		if err := json.Unmarshal(data, &a); err != nil {
			return err
		}
		c.Audio = &a
	case "resource":
		var r EmbeddedResource
		if err := json.Unmarshal(data, &r); err != nil {
			return err
		}
		c.EmbeddedResource = &r
	case "image":
		var i ImageContent
		if err := json.Unmarshal(data, &i); err != nil {
			return err
		}
		c.Image = &i
	case "resource_link":
		var rl ResourceLinkContent
		if err := json.Unmarshal(data, &rl); err != nil {
			return err
		}
		c.ResourceLink = &rl
	case "text":
		var t TextContent
		if err := json.Unmarshal(data, &t); err != nil {
			return err
		}
		c.Text = &t
	default:
		return fmt.Errorf("unknown content type %q", head.Type)
	}

	return nil
}

// CreateMessageRequestParam represents a request to create a message
type CreateMessageRequestParam struct {
	IncludeContext   *CreateMessageRequestParamIncludeContext `json:"includeContext,omitempty"`
	MaxTokens        int64                                    `json:"maxTokens"`
	Messages         []SamplingMessage                        `json:"messages"`
	ModelPreferences *ModelPreferences                        `json:"modelPreferences,omitempty"`
	StopSequences    []string                                 `json:"stopSequences,omitempty"`
	SystemPrompt     *string                                  `json:"systemPrompt,omitempty"`
	Temperature      *float64                                 `json:"temperature,omitempty"`
}

// CreateMessageRequestParamIncludeContext represents context inclusion options
type CreateMessageRequestParamIncludeContext string

const (
	AllServers CreateMessageRequestParamIncludeContext = "allServers"
	None       CreateMessageRequestParamIncludeContext = "none"
	ThisServer CreateMessageRequestParamIncludeContext = "thisServer"
)

func (t *CreateMessageRequestParamIncludeContext) UnmarshalJSON(data []byte) error {
	var s string
	if err := json.Unmarshal(data, &s); err != nil {
		return err
	}

	ct := CreateMessageRequestParamIncludeContext(s)
	if !ct.Valid() {
		return fmt.Errorf("invalid CreateMessageRequestParamIncludeContext %q", s)
	}

	*t = ct
	return nil
}

func (t CreateMessageRequestParamIncludeContext) Valid() bool {
	switch t {
	case AllServers, None, ThisServer:
		return true
	default:
		return false
	}
}

// CreateMessageResult represents the result of creating a message
type CreateMessageResult struct {
	Content    CreateMessageResultContent `json:"content"`
	Model      string                     `json:"model"`
	Role       Role                       `json:"role"`
	StopReason *string                    `json:"stopReason,omitempty"`
}

type CreateMessageResultContent SamplingMessage

// ElicitRequestParamWithTimeout represents a request for user elicitation
type ElicitRequestParamWithTimeout struct {
	Message         string `json:"message"`
	RequestedSchema Schema `json:"requestedSchema"`
	Timeout         *int64 `json:"timeout,omitempty"`
}

// ElicitResult represents the result of an elicitation
type ElicitResult struct {
	Action  ElicitResultAction                  `json:"action"`
	Content map[string]ElicitResultContentValue `json:"content,omitempty"`
}

// ElicitResultAction represents the action taken in elicitation
type ElicitResultAction string

const (
	Accept  ElicitResultAction = "accept"
	Cancel  ElicitResultAction = "cancel"
	Decline ElicitResultAction = "decline"
)

func (e *ElicitResultAction) UnmarshalJSON(data []byte) error {
	var s string
	if err := json.Unmarshal(data, &s); err != nil {
		return err
	}

	ea := ElicitResultAction(s)
	if !ea.Valid() {
		return fmt.Errorf("invalid ElicitResultAction %q", s)
	}

	*e = ea
	return nil
}

func (e ElicitResultAction) Valid() bool {
	switch e {
	case Accept, Cancel, Decline:
		return true
	default:
		return false
	}
}

type ElicitResultContentValue struct {
	String  *string
	Number  *json.Number
	Boolean *bool
}

func (v ElicitResultContentValue) MarshalJSON() ([]byte, error) {
	switch {
	case v.String != nil:
		return json.Marshal(v.String)
	case v.Number != nil:
		return json.Marshal(v.Number)
	case v.Boolean != nil:
		return json.Marshal(v.Boolean)
	default:
		return nil, fmt.Errorf("ElicitResultContentValue has no value set")
	}
}

func (v *ElicitResultContentValue) UnmarshalJSON(data []byte) error {
	// Clear existing values
	*v = ElicitResultContentValue{}

	// Try string first
	var s string
	if err := json.Unmarshal(data, &s); err == nil {
		v.String = &s
		return nil
	}

	// Then bool
	var b bool
	if err := json.Unmarshal(data, &b); err == nil {
		v.Boolean = &b
		return nil
	}

	// Then number
	var n json.Number
	if err := json.Unmarshal(data, &n); err == nil {
		v.Number = &n
		return nil
	}

	// If all fail, it's not a valid primitive for this type
	return fmt.Errorf("ElicitResultContentValue: unsupported JSON value: %s", string(data))
}

// EmbeddedResource represents an embedded resource
type EmbeddedResource struct {
	Meta        Meta             `json:"_meta,omitempty"`
	Annotations *Annotations     `json:"annotations,omitempty"`
	Resource    ResourceContents `json:"resource"`
}

func (e EmbeddedResource) MarshalJSON() ([]byte, error) {
	type alias EmbeddedResource

	return json.Marshal(&struct {
		Type string `json:"type"`
		alias
	}{
		Type:  "resource",
		alias: (alias)(e),
	})
}

func (e *EmbeddedResource) UnmarshalJSON(data []byte) error {
	type alias EmbeddedResource
	aux := struct {
		Type string `json:"type"`
		alias
	}{}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	if aux.Type != "resource" && aux.Type != "" {
		return fmt.Errorf("invalid type %q, expected \"resource\"", aux.Type)
	}

	*e = EmbeddedResource(aux.alias)
	return nil
}

// EnumSchema represents an enum input schema
type EnumSchema struct {
	Description *string  `json:"description,omitempty"`
	Enum        []string `json:"enum"`
	EnumNames   []string `json:"enumNames,omitempty"`
	Title       *string  `json:"title,omitempty"`
}

func (e EnumSchema) MarshallJSON() ([]byte, error) {
	type alias EnumSchema

	return json.Marshal(&struct {
		Type string `json:"type"`
		alias
	}{
		Type:  "resource",
		alias: (alias)(e),
	})
}

func (e *EnumSchema) UnmarshalJSON(data []byte) error {
	type alias EnumSchema
	aux := struct {
		Type string `json:"type"`
		alias
	}{}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	if aux.Type != "string" && aux.Type != "" {
		return fmt.Errorf("invalid type %q, expected \"string\"", aux.Type)
	}

	*e = EnumSchema(aux.alias)
	return nil
}

// GetPromptRequest represents a request to get a prompt
type GetPromptRequest struct {
	Context PluginRequestContext  `json:"context"`
	Request GetPromptRequestParam `json:"request"`
}

// GetPromptRequestParam represents parameters for getting a prompt
type GetPromptRequestParam struct {
	Arguments map[string]string `json:"arguments,omitempty"`
	Name      string            `json:"name"`
}

// GetPromptResult represents the result of getting a prompt
type GetPromptResult struct {
	Description *string         `json:"description,omitempty"`
	Messages    []PromptMessage `json:"messages"`
}

// ImageContent represents image content
type ImageContent struct {
	Meta        Meta         `json:"_meta,omitempty"`
	Annotations *Annotations `json:"annotations,omitempty"`
	Data        string       `json:"data"`
	MimeType    string       `json:"mimeType"`
}

func (i ImageContent) MarshallJSON() ([]byte, error) {
	type alias ImageContent

	return json.Marshal(&struct {
		Type string `json:"type"`
		alias
	}{
		Type:  "image",
		alias: (alias)(i),
	})
}

func (i *ImageContent) UnmarshalJSON(data []byte) error {
	type alias ImageContent
	aux := struct {
		Type string `json:"type"`
		alias
	}{}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	if aux.Type != "image" && aux.Type != "" { // allow empty if missing
		return fmt.Errorf("invalid type %q, expected \"image\"", aux.Type)
	}

	*i = ImageContent(aux.alias)
	return nil
}

// ListPromptsRequest represents a request to list prompts
type ListPromptsRequest struct {
	Context PluginRequestContext `json:"context"`
}

// ListPromptsResult represents the result of listing prompts
type ListPromptsResult struct {
	Prompts []Prompt `json:"prompts"`
}

// ListResourcesRequest represents a request to list resources
type ListResourcesRequest struct {
	Context PluginRequestContext `json:"context"`
}

// ListResourcesResult represents the result of listing resources
type ListResourcesResult struct {
	Resources []Resource `json:"resources"`
}

// ListResourceTemplatesRequest represents a request to list resource templates
type ListResourceTemplatesRequest struct {
	Context PluginRequestContext `json:"context"`
}

// ListResourceTemplatesResult represents the result of listing resource templates
type ListResourceTemplatesResult struct {
	ResourceTemplates []ResourceTemplate `json:"resourceTemplates"`
}

// ListRootsResult represents the result of listing roots
type ListRootsResult struct {
	Roots []Root `json:"roots"`
}

// ListToolsRequest represents a request to list tools
type ListToolsRequest struct {
	Context PluginRequestContext `json:"context"`
}

// ListToolsResult represents the result of listing tools
type ListToolsResult struct {
	Tools []Tool `json:"tools"`
}

// LoggingLevel represents the severity level of a log message
type LoggingLevel string

const (
	Debug     LoggingLevel = "debug"
	Info      LoggingLevel = "info"
	Notice    LoggingLevel = "notice"
	Warning   LoggingLevel = "warning"
	Error     LoggingLevel = "error"
	Critical  LoggingLevel = "critical"
	Alert     LoggingLevel = "alert"
	Emergency LoggingLevel = "emergency"
)

func (l *LoggingLevel) UnmarshalJSON(data []byte) error {
	var s string
	if err := json.Unmarshal(data, &s); err != nil {
		return err
	}

	ll := LoggingLevel(s)
	if !ll.Validate() {
		return fmt.Errorf("invalid LoggingLevel %q", s)
	}

	*l = ll
	return nil
}

func (l LoggingLevel) Validate() bool {
	switch l {
	case Debug, Info, Notice, Warning, Error, Critical, Alert, Emergency:
		return true
	default:
		return false
	}
}

// LoggingMessageNotificationParam represents a logging message notification
type LoggingMessageNotificationParam struct {
	Data   any          `json:"data"`
	Level  LoggingLevel `json:"level"`
	Logger *string      `json:"logger,omitempty"`
}

// Meta represents metadata as a generic JSON object
type Meta map[string]any

// ModelHint represents a hint for model selection
type ModelHint struct {
	Name string `json:"name"`
}

// ModelPreferences represents preferences for model selection
type ModelPreferences struct {
	CostPriority         float32     `json:"costPriority,omitempty"`
	Hints                []ModelHint `json:"hints,omitempty"`
	IntelligencePriority float32     `json:"intelligencePriority,omitempty"`
	SpeedPriority        float32     `json:"speedPriority,omitempty"`
}

// NumberSchema represents a number input schema
type NumberSchema struct {
	Description *string    `json:"description,omitempty"`
	Maximum     *float64   `json:"maximum,omitempty"`
	Minimum     *float64   `json:"minimum,omitempty"`
	Title       *string    `json:"title,omitempty"`
	Type        NumberType `json:"type"` // "number" or "integer"
}

// NumberType represents the type of a number schema
type NumberType string

const (
	Number  NumberType = "number"
	Integer NumberType = "integer"
)

func (n *NumberType) UnmarshalJSON(data []byte) error {
	var s string
	if err := json.Unmarshal(data, &s); err != nil {
		return err
	}

	nt := NumberType(s)
	if !nt.Valid() {
		return fmt.Errorf("invalid NumberType %q", s)
	}

	*n = nt
	return nil
}

func (n NumberType) Valid() bool {
	switch n {
	case Number, Integer:
		return true
	default:
		return false
	}
}

// PluginNotificationContext represents the context for a plugin notification
type PluginNotificationContext struct {
	Meta Meta `json:"meta"`
}

// PluginRequestContext represents the context for a plugin request
type PluginRequestContext struct {
	Meta Meta            `json:"_meta"`
	ID   PluginRequestId `json:"id"`
}

type PluginRequestId struct {
	String *string
	Number *int64
}

func (p PluginRequestId) MarshalJSON() ([]byte, error) {
	switch {
	case p.String != nil:
		return json.Marshal(p.String)
	case p.Number != nil:
		return json.Marshal(p.Number)
	default:
		return nil, fmt.Errorf("empty PluginRequestId")
	}
}

func (p *PluginRequestId) UnmarshalJSON(data []byte) error {
	*p = PluginRequestId{}

	// Try string first
	var s string
	if err := json.Unmarshal(data, &s); err == nil {
		p.String = &s
		return nil
	}

	// Then number
	var n int64
	if err := json.Unmarshal(data, &n); err == nil {
		p.Number = &n
		return nil
	}

	// If all fail, it's not a valid primitive for this type
	return fmt.Errorf("PluginRequestId: unsupported JSON value: %s", string(data))
}

// PrimitiveSchemaDefinition is a union type for schema definitions
type PrimitiveSchemaDefinition struct {
	Boolean *BooleanSchema
	Enum    *EnumSchema
	Number  *NumberSchema
	String  *StringSchema
}

func (p PrimitiveSchemaDefinition) MarshalJSON() ([]byte, error) {
	switch {
	case p.Boolean != nil:
		return json.Marshal(p.Boolean)
	case p.Enum != nil:
		return json.Marshal(p.Enum)
	case p.Number != nil:
		return json.Marshal(p.Number)
	case p.String != nil:
		return json.Marshal(p.String)
	default:
		return nil, fmt.Errorf("empty PrimitiveSchemaDefinition")
	}
}

func (p *PrimitiveSchemaDefinition) UnmarshalJSON(data []byte) error {
	var head struct {
		Type string `json:"type"`
	}
	if err := json.Unmarshal(data, &head); err != nil {
		return err
	}

	switch head.Type {
	case "boolean":
		var b BooleanSchema
		if err := json.Unmarshal(data, &b); err != nil {
			return err
		}
		p.Boolean = &b
	case "string":
		var e EnumSchema
		if err := json.Unmarshal(data, &e); err != nil {
			var s StringSchema
			if err := json.Unmarshal(data, &s); err != nil {
				return err
			}
			p.String = &s
		} else {
			p.Enum = &e
		}
	case "number", "integer":
		var n NumberSchema
		if err := json.Unmarshal(data, &n); err != nil {
			return err
		}
		p.Number = &n
	}

	return nil
}

// ProgressNotificationParam represents a progress notification
type ProgressNotificationParam struct {
	Message       *string  `json:"message,omitempty"`
	Progress      float64  `json:"progress"`
	ProgressToken string   `json:"progressToken"`
	Total         *float64 `json:"total,omitempty"`
}

// Prompt represents a prompt
type Prompt struct {
	Arguments   []PromptArgument `json:"arguments,omitempty"`
	Description *string          `json:"description,omitempty"`
	Name        string           `json:"name"`
	Title       *string          `json:"title,omitempty"`
}

// PromptArgument represents an argument for a prompt
type PromptArgument struct {
	Description *string `json:"description,omitempty"`
	Name        string  `json:"name"`
	Required    *bool   `json:"required,omitempty"`
	Title       *string `json:"title,omitempty"`
}

// PromptMessage represents a message in a prompt
type PromptMessage struct {
	Content ContentBlock `json:"content"`
	Role    Role         `json:"role"`
}

// PromptReference represents a reference to a prompt
type PromptReference struct {
	Name  string  `json:"name"`
	Title *string `json:"title,omitempty"`
}

func (p PromptReference) MarshalJSON() ([]byte, error) {
	type alias PromptReference
	return json.Marshal(&struct {
		Type string `json:"type"`
		alias
	}{
		Type:  "prompt",
		alias: (alias)(p),
	})
}

func (p *PromptReference) UnmarshalJSON(data []byte) error {
	type alias PromptReference
	aux := struct {
		Type string `json:"type"`
		alias
	}{}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	if aux.Type != "prompt" && aux.Type != "" { // allow empty if missing
		return fmt.Errorf("invalid type %q, expected \"prompt\"", aux.Type)
	}

	*p = PromptReference(aux.alias)
	return nil
}

// ReadResourceRequest represents a request to read a resource
type ReadResourceRequest struct {
	Context PluginRequestContext     `json:"context"`
	Request ReadResourceRequestParam `json:"request"`
}

// ReadResourceRequestParam represents parameters for reading a resource
type ReadResourceRequestParam struct {
	URI string `json:"uri"`
}

// ReadResourceResult represents the result of reading a resource
type ReadResourceResult struct {
	Contents []ResourceContents `json:"contents"`
}

type Reference struct {
	Prompt           *PromptReference
	ResourceTemplate *ResourceTemplateReference
}

func (r Reference) MarshalJSON() ([]byte, error) {
	switch {
	case r.Prompt != nil:
		return json.Marshal(r.Prompt)
	case r.ResourceTemplate != nil:
		return json.Marshal(r.ResourceTemplate)
	default:
		return nil, fmt.Errorf("empty Reference")
	}
}

func (r *Reference) UnmarshalJSON(data []byte) error {
	var head struct {
		Type string `json:"type"`
	}
	if err := json.Unmarshal(data, &head); err != nil {
		return err
	}

	switch head.Type {
	case "prompt":
		var p PromptReference
		if err := json.Unmarshal(data, &p); err != nil {
			return err
		}
		r.Prompt = &p
	case "resource":
		var rt ResourceTemplateReference
		if err := json.Unmarshal(data, &rt); err != nil {
			return err
		}
		r.ResourceTemplate = &rt
	default:
		return fmt.Errorf("unknown reference type %q", head.Type)
	}

	return nil
}

// Resource represents a resource
type Resource struct {
	Annotations *Annotations `json:"annotations,omitempty"`
	Description *string      `json:"description,omitempty"`
	MimeType    *string      `json:"mimeType,omitempty"`
	Name        string       `json:"name"`
	Size        *int64       `json:"size,omitempty"`
	Title       *string      `json:"title,omitempty"`
	URI         string       `json:"uri"`
}

type ResourceContents struct {
	Blob *BlobResourceContents
	Text *TextResourceContents
}

func (R ResourceContents) MarshalJSON() ([]byte, error) {
	switch {
	case R.Blob != nil:
		return json.Marshal(R.Blob)
	case R.Text != nil:
		return json.Marshal(R.Text)
	default:
		return nil, fmt.Errorf("empty ResourceContents")
	}
}

func (r *ResourceContents) UnmarshalJSON(data []byte) error {
	// Clear existing values
	*r = ResourceContents{}

	// Try blob first
	var b BlobResourceContents
	if err := json.Unmarshal(data, &b); err == nil {
		r.Blob = &b
		return nil
	}

	// Then text
	var t TextResourceContents
	if err := json.Unmarshal(data, &t); err == nil {
		r.Text = &t
		return nil
	}

	// If all fail, it's not a valid ResourceContents
	return fmt.Errorf("ResourceContents: unsupported JSON value: %s", string(data))
}

// ResourceLinkContent represents a link to a resource
type ResourceLinkContent struct {
	Meta        Meta         `json:"_meta,omitempty"`
	Annotations *Annotations `json:"annotations,omitempty"`
	Description *string      `json:"description,omitempty"`
	MimeType    *string      `json:"mimeType,omitempty"`
	Name        string       `json:"name"`
	Size        *int64       `json:"size,omitempty"`
	Title       *string      `json:"title,omitempty"`
	URI         string       `json:"uri"`
}

func (r ResourceLinkContent) MarshallJSON() ([]byte, error) {
	type alias ResourceLinkContent
	return json.Marshal(&struct {
		Type string `json:"type"`
		alias
	}{
		Type:  "resource_link",
		alias: (alias)(r),
	})
}

func (r *ResourceLinkContent) UnmarshalJSON(data []byte) error {
	type alias ResourceLinkContent
	aux := struct {
		Type string `json:"type"`
		alias
	}{}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	if aux.Type != "resource_link" && aux.Type != "" { // allow empty if missing
		return fmt.Errorf("invalid type %q, expected \"resource_link\"", aux.Type)
	}

	*r = ResourceLinkContent(aux.alias)
	return nil
}

// ResourceTemplate represents a resource template
type ResourceTemplate struct {
	Annotations *Annotations `json:"annotations,omitempty"`
	Description *string      `json:"description,omitempty"`
	MimeType    *string      `json:"mimeType,omitempty"`
	Name        string       `json:"name"`
	Title       *string      `json:"title,omitempty"`
	URITemplate string       `json:"uriTemplate"`
}

// ResourceTemplateReference represents a reference to a resource template
type ResourceTemplateReference struct {
	URI string `json:"uri"`
}

func (r ResourceTemplateReference) MarshallJSON() ([]byte, error) {
	type alias ResourceTemplateReference
	return json.Marshal(&struct {
		Type string `json:"type"`
		alias
	}{
		Type:  "resource",
		alias: (alias)(r),
	})
}

func (r *ResourceTemplateReference) UnmarshalJSON(data []byte) error {
	type alias ResourceTemplateReference
	aux := struct {
		Type string `json:"type"`
		alias
	}{}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	if aux.Type != "resource" && aux.Type != "" { // allow empty if missing
		return fmt.Errorf("invalid type %q, expected \"resource\"", aux.Type)
	}

	*r = ResourceTemplateReference(aux.alias)
	return nil
}

// ResourceUpdatedNotificationParam represents a resource update notification
type ResourceUpdatedNotificationParam struct {
	URI string `json:"uri"`
}

// Role represents the role of a message sender
type Role string

const (
	Assistant Role = "assistant"
	User      Role = "user"
)

func (r *Role) UnmarshalJSON(data []byte) error {
	var s string
	if err := json.Unmarshal(data, &s); err != nil {
		return err
	}

	rr := Role(s)
	if !rr.Valid() {
		return fmt.Errorf("invalid Role %q", s)
	}

	*r = rr
	return nil
}

func (r Role) Valid() bool {
	switch r {
	case Assistant, User:
		return true
	default:
		return false
	}
}

// Root represents a root directory or resource
type Root struct {
	Name *string `json:"name,omitempty"`
	URI  string  `json:"uri"`
}

type SamplingMessage struct {
	Audio *AudioContent
	Image *ImageContent
	Text  *TextContent
}

func (s SamplingMessage) MarshalJSON() ([]byte, error) {
	switch {
	case s.Audio != nil:
		return json.Marshal(s.Audio)
	case s.Image != nil:
		return json.Marshal(s.Image)
	case s.Text != nil:
		return json.Marshal(s.Text)
	default:
		return nil, fmt.Errorf("empty SamplingMessage")
	}
}

func (s *SamplingMessage) UnmarshalJSON(data []byte) error {
	var head struct {
		Type string `json:"type"`
	}
	if err := json.Unmarshal(data, &head); err != nil {
		return err
	}

	switch head.Type {
	case "audio":
		var a AudioContent
		if err := json.Unmarshal(data, &a); err != nil {
			return err
		}
		s.Audio = &a
	case "image":
		var i ImageContent
		if err := json.Unmarshal(data, &i); err != nil {
			return err
		}
		s.Image = &i
	case "text":
		var t TextContent
		if err := json.Unmarshal(data, &t); err != nil {
			return err
		}
		s.Text = &t
	default:
		return fmt.Errorf("unknown content type %q", head.Type)
	}

	return nil
}

// Schema represents a JSON schema
type Schema struct {
	Properties map[string]PrimitiveSchemaDefinition `json:"properties,omitempty"`
	Required   []string                             `json:"required,omitempty"`
}

func (s Schema) MarshallJSON() ([]byte, error) {
	type alias Schema
	return json.Marshal(&struct {
		Type string `json:"type"`
		alias
	}{
		Type:  "object",
		alias: (alias)(s),
	})
}

func (s *Schema) UnmarshalJSON(data []byte) error {
	type alias Schema
	aux := struct {
		Type string `json:"type"`
		alias
	}{}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	// Optional: validate `type`
	if aux.Type != "object" && aux.Type != "" { // allow empty if missing
		return fmt.Errorf("invalid type %q, expected \"object\"", aux.Type)
	}

	*s = Schema(aux.alias)
	return nil
}

// StringSchema represents a string input schema
type StringSchema struct {
	Description *string             `json:"description,omitempty"`
	Format      *StringSchemaFormat `json:"format,omitempty"`
	MaxLength   *int64              `json:"maxLength,omitempty"`
	MinLength   *int64              `json:"minLength,omitempty"`
	Title       *string             `json:"title,omitempty"`
}

func (s StringSchema) MarshallJSON() ([]byte, error) {
	type alias StringSchema
	return json.Marshal(&struct {
		Type string `json:"type"`
		alias
	}{
		Type:  "string",
		alias: (alias)(s),
	})
}

func (s *StringSchema) UnmarshalJSON(data []byte) error {
	type alias StringSchema
	aux := struct {
		Type string `json:"type"`
		alias
	}{}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	// Optional: validate `type`
	if aux.Type != "string" && aux.Type != "" { // allow empty if missing
		return fmt.Errorf("invalid type %q, expected \"string\"", aux.Type)
	}

	*s = StringSchema(aux.alias)
	return nil
}

// StringSchemaFormat represents the format of a string schema
type StringSchemaFormat string

const (
	Email    StringSchemaFormat = "email"
	URI      StringSchemaFormat = "uri"
	Date     StringSchemaFormat = "date"
	DateTime StringSchemaFormat = "date_time"
)

func (s *StringSchemaFormat) UnmarshalJSON(data []byte) error {
	var str string
	if err := json.Unmarshal(data, &str); err != nil {
		return err
	}

	sf := StringSchemaFormat(str)
	if !sf.Valid() {
		return fmt.Errorf("invalid StringSchemaFormat %q", str)
	}

	*s = sf
	return nil
}

func (s StringSchemaFormat) Valid() bool {
	switch s {
	case Email, URI, Date, DateTime:
		return true
	default:
		return false
	}
}

// TextContent represents text content
type TextContent struct {
	Meta        Meta         `json:"_meta,omitempty"`
	Annotations *Annotations `json:"annotations,omitempty"`
	Text        string       `json:"text"`
}

func (t TextContent) MarshallJSON() ([]byte, error) {
	type alias TextContent
	return json.Marshal(&struct {
		Type string `json:"type"`
		alias
	}{
		Type:  "text",
		alias: (alias)(t),
	})
}

func (t *TextContent) UnmarshalJSON(data []byte) error {
	type alias TextContent
	aux := struct {
		Type string `json:"type"`
		alias
	}{}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	if aux.Type != "text" && aux.Type != "" { // allow empty if missing
		return fmt.Errorf("invalid type %q, expected \"text\"", aux.Type)
	}

	*t = TextContent(aux.alias)
	return nil
}

// TextResourceContents represents text resource contents
type TextResourceContents struct {
	Meta     Meta    `json:"_meta,omitempty"`
	MimeType *string `json:"mimeType,omitempty"`
	Text     string  `json:"text"`
	URI      string  `json:"uri"`
}

// Tool represents a tool
type Tool struct {
	Annotations  *Annotations `json:"annotations,omitempty"`
	Description  *string      `json:"description,omitempty"`
	InputSchema  ToolSchema   `json:"inputSchema"`
	Name         string       `json:"name"`
	OutputSchema *ToolSchema  `json:"outputSchema,omitempty"`
	Title        *string      `json:"title,omitempty"`
}

// ToolSchema represents the schema for tool input or output
type ToolSchema struct {
	Properties map[string]any `json:"properties,omitempty"`
	Required   []string       `json:"required,omitempty"`
	Type       string         `json:"type"` // "object"
}
