package main

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"os"
	"path/filepath"
	"text/template"
	"time"
)

type ComponentHashes struct {
	Frontend      string
	MosaicTile    string
}

type DeployInfo struct {
	Timestamp          string
	MosaicTileAddress string
}

type BalanceInfo struct {
	Address string
	Balance string
}

type TemplateData struct {
	LastUpdated string
	Hashes      ComponentHashes
	Deploy      DeployInfo
	Balances    map[string]BalanceInfo
}

func calculateDirectoryHash(path string) (string, error) {
	hash := sha256.New()
	err := filepath.Walk(path, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() {
			data, err := ioutil.ReadFile(path)
			if err != nil {
				return err
			}
			hash.Write(data)
		}
		return nil
	})
	if err != nil {
		return "", err
	}
	return hex.EncodeToString(hash.Sum(nil))[:8], nil
}

func getLatestDeployInfo() (DeployInfo, error) {
	gistId := os.Getenv("GIST_ID")
	token := os.Getenv("GITHUB_TOKEN")
	url := fmt.Sprintf("https://api.github.com/gists/%s", gistId)

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		return DeployInfo{}, err
	}

	req.Header.Set("Authorization", "token "+token)
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return DeployInfo{}, err
	}
	defer resp.Body.Close()

	var gist struct {
		Files map[string]struct {
			Content string `json:"content"`
		} `json:"files"`
	}

	if err := json.NewDecoder(resp.Body).Decode(&gist); err != nil {
		return DeployInfo{}, err
	}

	var latestDeploy struct {
		Timestamp string `json:"timestamp"`
		Job struct {
			Data struct {
				MosaicTileAddress    string `json:"mosaic_tile_address"`
			} `json:"data"`
		} `json:"job"`
	}

	var deployInfo DeployInfo
	for filename, file := range gist.Files {
		if filename == "mosaic_tile_nft_deploy.json" {
			if err := json.Unmarshal([]byte(file.Content), &latestDeploy); err != nil {
				continue
			}
			deployInfo = DeployInfo{
				Timestamp:            latestDeploy.Timestamp,
				MosaicTileAddress:    latestDeploy.Job.Data.MosaicTileAddress,
			}
			break
		}
	}

	return deployInfo, nil
}

func main() {
	// Calculate component hashes
	frontendHash, err := calculateDirectoryHash("frontend")
	if err != nil {
		fmt.Printf("Error calculating frontend hash: %v\n", err)
		os.Exit(1)
	}

	mosaicTileHash, err := calculateDirectoryHash("contracts/mosaic_tile_nft")
	if err != nil {
		fmt.Printf("Error calculating mosaic tile hash: %v\n", err)
		os.Exit(1)
	}


	// Get latest deploy info
	deployInfo, err := getLatestDeployInfo()
	if err != nil {
		fmt.Printf("Error getting deploy info: %v\n", err)
		os.Exit(1)
	}

	// Prepare template data
	data := TemplateData{
		LastUpdated: time.Now().UTC().Format(time.RFC3339),
		Hashes: ComponentHashes{
			Frontend:      frontendHash,
			MosaicTile:    mosaicTileHash,
		},
		Deploy: deployInfo,
		Balances: map[string]BalanceInfo{
			"deployer": {Address: os.Getenv("DEPLOYER_ADDRESS")},
			"minter":   {Address: os.Getenv("MINTER_ADDRESS")},
			"owner":    {Address: os.Getenv("OWNER_ADDRESS")},
			"user":     {Address: os.Getenv("USER_ADDRESS")},
		},
	}

	// Read template
	tmpl, err := template.ParseFiles("README.template.md")
	if err != nil {
		fmt.Printf("Error parsing template: %v\n", err)
		os.Exit(1)
	}

	// Create output file
	output, err := os.Create("README.md")
	if err != nil {
		fmt.Printf("Error creating output file: %v\n", err)
		os.Exit(1)
	}
	defer output.Close()

	// Execute template
	if err := tmpl.Execute(output, data); err != nil {
		fmt.Printf("Error executing template: %v\n", err)
		os.Exit(1)
	}

	fmt.Println("README.md updated successfully")
} 