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

# Create artifact directory if it doesn't exist
mkdir -p /tmp/artifacts

# Run specific workflow or all workflows
if [ -n "$2" ]; then
    WORKFLOW="-W $2"
else
    WORKFLOW=""
fi

# Set default platform
PLATFORM="ubuntu-latest=ghcr.io/catthehacker/ubuntu:act-latest"

# Print debug info
echo "Event: $EVENT"
echo "Workflow: $WORKFLOW"
echo "Platform: $PLATFORM"
echo "Working directory: $(pwd)"
echo "Contents of .secrets:"
cat .secrets
echo

# Run act with common configuration
act $EVENT \
    $WORKFLOW \
    --secret-file .secrets \
    --platform $PLATFORM \
    --artifact-server-path /tmp/artifacts \
    --env GITHUB_REPOSITORY="malikfassifihri/mosaic" \
    --env GITHUB_REF="refs/heads/main" \
    -v