package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"strings"

	pdk "github.com/extism/go-pdk"
)

func Call(input CallToolRequest) (CallToolResult, error) {
	args := input.Params.Arguments
	if args == nil {
		return CallToolResult{}, errors.New("Arguments must be provided")
	}

	argsMap := args.(map[string]interface{})
	fmt.Println("argsMap", argsMap)
	return getCryptoPrice(argsMap)
}

func getCryptoPrice(args map[string]interface{}) (CallToolResult, error) {
	symbol, ok := args["symbol"].(string)
	if !ok {
		return CallToolResult{}, errors.New("symbol must be provided")
	}

	// Convert symbol to uppercase
	symbol = strings.ToUpper(symbol)

	// Use CoinGecko API to get the price
	url := fmt.Sprintf("https://api.coingecko.com/api/v3/simple/price?ids=%s&vs_currencies=usd", strings.ToLower(symbol))
	req := pdk.NewHTTPRequest(pdk.MethodGet, url)
	resp := req.Send()

	var result map[string]map[string]float64
	if err := json.Unmarshal(resp.Body(), &result); err != nil {
		return CallToolResult{}, fmt.Errorf("failed to parse response: %v", err)
	}

	if price, ok := result[strings.ToLower(symbol)]["usd"]; ok {
		priceStr := fmt.Sprintf("%.2f USD", price)
		return CallToolResult{
			Content: []Content{
				{
					Type: ContentTypeText,
					Text: &priceStr,
				},
			},
		}, nil
	}

	return CallToolResult{}, fmt.Errorf("price not found for %s", symbol)
}

func Describe() (ListToolsResult, error) {
	return ListToolsResult{
		Tools: []ToolDescription{
			{
				Name:        "crypto-price",
				Description: "Get the current price of a cryptocurrency in USD",
				InputSchema: map[string]interface{}{
					"type":     "object",
					"required": []string{"symbol"},
					"properties": map[string]interface{}{
						"symbol": map[string]interface{}{
							"type":        "string",
							"description": "the cryptocurrency symbol/id (e.g., bitcoin, ethereum)",
						},
					},
				},
			},
		},
	}, nil
}
