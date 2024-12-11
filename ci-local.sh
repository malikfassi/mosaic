#!/bin/bash

# Check if act is installed
if ! command -v act &> /dev/null; then
    echo "act is not installed. Installing..."
    if [[ "$OSTYPE" == "darwin"* ]]; then
        brew install act
    else
        curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash
    fi
fi

# Default event is push
EVENT=${1:-push}

# Create secrets file if it doesn't exist
if [ ! -f .secrets ]; then
    echo "Creating .secrets file..."
    cat > .secrets << EOL
DISCORD_WEBHOOK=dummy-webhook-for-testing
GIST_TOKEN=dummy-token-for-testing
EOL
fi

# Run specific workflow or all workflows
if [ -n "$2" ]; then
    WORKFLOW="--workflow $2"
else
    WORKFLOW=""
fi

# Run act with common configuration
act $EVENT \
    $WORKFLOW \
    --secret-file .secrets \
    --platform ubuntu-latest=ghcr.io/catthehacker/ubuntu:act-latest \
    --bind \
    -v