const { Client, GatewayIntentBits, EmbedBuilder } = require('discord.js');
const { CosmWasmClient } = require('@cosmjs/cosmwasm-stargate');
const { DirectSecp256k1HdWallet } = require('@cosmjs/proto-signing');
require('dotenv').config();

const client = new Client({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.MessageContent,
  ],
});

// Contract addresses
const NFT_CONTRACT = process.env.NFT_CONTRACT_ADDRESS;
const COLORING_CONTRACT = process.env.COLORING_CONTRACT_ADDRESS;

// RPC configuration
const RPC_ENDPOINT = process.env.STARGAZE_RPC_ENDPOINT;
const CHAIN_ID = process.env.STARGAZE_CHAIN_ID;

// Initialize CosmWasm client
let cosmWasmClient;

async function initializeCosmWasmClient() {
  cosmWasmClient = await CosmWasmClient.connect(RPC_ENDPOINT);
}

// Event monitoring
async function monitorContractEvents() {
  // Poll for new events every 6 seconds (average block time)
  setInterval(async () => {
    try {
      // Query NFT contract events
      const nftEvents = await cosmWasmClient.queryContractSmart(NFT_CONTRACT, {
        get_events: { from_height: lastProcessedHeight }
      });

      // Query coloring contract events
      const coloringEvents = await cosmWasmClient.queryContractSmart(COLORING_CONTRACT, {
        get_events: { from_height: lastProcessedHeight }
      });

      // Process events
      processEvents([...nftEvents, ...coloringEvents]);
    } catch (error) {
      console.error('Error monitoring events:', error);
    }
  }, 6000);
}

// Event processing
function processEvents(events) {
  events.forEach(event => {
    switch(event.type) {
      case 'nft_mint':
        sendNftMintNotification(event);
        break;
      case 'color_change':
        sendColorChangeNotification(event);
        break;
      case 'area_transfer':
        sendAreaTransferNotification(event);
        break;
      case 'price_update':
        sendPriceUpdateNotification(event);
        break;
    }
  });
}

// Discord message builders
function sendNftMintNotification(event) {
  const embed = new EmbedBuilder()
    .setTitle('🎨 New Area Minted!')
    .setColor('#00ff00')
    .addFields(
      { name: 'Area ID', value: event.area_id },
      { name: 'Owner', value: event.owner },
      { name: 'Price', value: `${event.price} STARS` }
    )
    .setTimestamp();

  sendToDiscord(embed);
}

function sendColorChangeNotification(event) {
  const embed = new EmbedBuilder()
    .setTitle('🎨 Color Changed')
    .setColor('#0099ff')
    .addFields(
      { name: 'Area ID', value: event.area_id },
      { name: 'New Color', value: event.color },
      { name: 'Changed By', value: event.user }
    )
    .setTimestamp();

  sendToDiscord(embed);
}

function sendAreaTransferNotification(event) {
  const embed = new EmbedBuilder()
    .setTitle('🔄 Area Transferred')
    .setColor('#ff9900')
    .addFields(
      { name: 'Area ID', value: event.area_id },
      { name: 'From', value: event.from },
      { name: 'To', value: event.to }
    )
    .setTimestamp();

  sendToDiscord(embed);
}

function sendPriceUpdateNotification(event) {
  const embed = new EmbedBuilder()
    .setTitle('💰 Price Updated')
    .setColor('#ff0000')
    .addFields(
      { name: 'Area ID', value: event.area_id },
      { name: 'New Price', value: `${event.price} STARS` },
      { name: 'Updated By', value: event.user }
    )
    .setTimestamp();

  sendToDiscord(embed);
}

// Discord command handlers
client.on('interactionCreate', async interaction => {
  if (!interaction.isCommand()) return;

  const { commandName } = interaction;

  switch (commandName) {
    case 'stats':
      await handleStatsCommand(interaction);
      break;
    case 'recent':
      await handleRecentCommand(interaction);
      break;
    case 'price':
      await handlePriceCommand(interaction);
      break;
  }
});

async function handleStatsCommand(interaction) {
  const stats = await getContractStats();
  const embed = new EmbedBuilder()
    .setTitle('📊 Contract Statistics')
    .setColor('#00ff00')
    .addFields(
      { name: 'Total Areas', value: stats.totalAreas.toString() },
      { name: 'Total Owners', value: stats.uniqueOwners.toString() },
      { name: 'Total Volume', value: `${stats.totalVolume} STARS` }
    )
    .setTimestamp();

  await interaction.reply({ embeds: [embed] });
}

async function handleRecentCommand(interaction) {
  const activities = await getRecentActivities();
  const embed = new EmbedBuilder()
    .setTitle('🕒 Recent Activities')
    .setColor('#0099ff')
    .setDescription(activities.map(a => 
      `• ${a.type}: Area ${a.area_id} (${a.time})`
    ).join('\n'))
    .setTimestamp();

  await interaction.reply({ embeds: [embed] });
}

async function handlePriceCommand(interaction) {
  const areaId = interaction.options.getString('area_id');
  const price = await getAreaPrice(areaId);
  
  const embed = new EmbedBuilder()
    .setTitle(`💰 Price Check: Area ${areaId}`)
    .setColor('#ff9900')
    .addFields(
      { name: 'Current Price', value: `${price} STARS` }
    )
    .setTimestamp();

  await interaction.reply({ embeds: [embed] });
}

// Utility functions
function sendToDiscord(embed) {
  const channel = client.channels.cache.get(process.env.DISCORD_CHANNEL_ID);
  if (channel) {
    channel.send({ embeds: [embed] });
  }
}

// Initialize bot
async function initialize() {
  try {
    await initializeCosmWasmClient();
    await client.login(process.env.DISCORD_BOT_TOKEN);
    console.log('Bot is ready!');
    monitorContractEvents();
  } catch (error) {
    console.error('Initialization error:', error);
    process.exit(1);
  }
}

initialize(); 