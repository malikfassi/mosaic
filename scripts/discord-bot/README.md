# Pixel Canvas Discord Bot

This bot monitors Pixel Canvas contract events and provides Discord notifications and commands.

## Features

### Event Notifications
- NFT minting events
- Color change events
- Area transfers
- Price updates

### Commands
- `/stats` - View contract statistics
- `/recent` - View recent activities
- `/price <area_id>` - Check area price

## Setup

1. Create a Discord Application
   - Go to [Discord Developer Portal](https://discord.com/developers/applications)
   - Create a new application
   - Create a bot user
   - Copy the bot token

2. Configure Bot Permissions
   - Required permissions:
     - Send Messages
     - Embed Links
     - Read Message History
     - Use Slash Commands
   - Generate invite URL with these permissions

3. Environment Setup
   ```bash
   # Clone the repository
   git clone <repository-url>
   cd scripts/discord-bot

   # Install dependencies
   npm install

   # Copy and configure environment variables
   cp .env.example .env
   # Edit .env with your values
   ```

4. Register Slash Commands
   ```bash
   node deploy-commands.js
   ```

5. Run the Bot
   ```bash
   # Development
   npm run dev

   # Production
   npm start
   ```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `DISCORD_BOT_TOKEN` | Your Discord bot token |
| `DISCORD_CHANNEL_ID` | Channel ID for notifications |
| `NFT_CONTRACT_ADDRESS` | Stargaze NFT contract address |
| `COLORING_CONTRACT_ADDRESS` | Coloring contract address |
| `STARGAZE_RPC_ENDPOINT` | Stargaze RPC endpoint |
| `STARGAZE_CHAIN_ID` | Stargaze chain ID |
| `EVENT_POLLING_INTERVAL` | Event check interval (ms) |
| `DEBUG_MODE` | Enable debug logging |

## Adding to Your Server

1. Use the generated invite URL from step 2
2. Select your server and authorize the bot
3. Configure the notification channel
4. Update `.env` with the channel ID

## Development

- Use `npm run dev` for development with auto-reload
- Check logs for event monitoring and command handling
- Test all commands in a development server first

## Production Deployment

1. Set up a production server
2. Install Node.js and npm
3. Clone the repository
4. Install dependencies
5. Configure production environment variables
6. Use PM2 or similar for process management:
   ```bash
   npm install -g pm2
   pm2 start index.js --name "pixel-canvas-bot"
   ```

## Troubleshooting

Common issues and solutions:

1. Bot not responding
   - Check bot token
   - Verify permissions
   - Check channel ID

2. Missing events
   - Check RPC endpoint
   - Verify contract addresses
   - Check event polling interval

3. Command errors
   - Verify slash command registration
   - Check bot permissions
   - Review error logs 