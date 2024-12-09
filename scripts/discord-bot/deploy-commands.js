const { REST, Routes, SlashCommandBuilder } = require('discord.js');
require('dotenv').config();

const commands = [
  new SlashCommandBuilder()
    .setName('stats')
    .setDescription('Get contract statistics'),
  
  new SlashCommandBuilder()
    .setName('recent')
    .setDescription('View recent activities'),
  
  new SlashCommandBuilder()
    .setName('price')
    .setDescription('Check area price')
    .addStringOption(option =>
      option
        .setName('area_id')
        .setDescription('The ID of the area to check')
        .setRequired(true)
    ),
].map(command => command.toJSON());

const rest = new REST({ version: '10' }).setToken(process.env.DISCORD_BOT_TOKEN);

(async () => {
  try {
    console.log('Started refreshing application (/) commands.');

    await rest.put(
      Routes.applicationCommands(process.env.DISCORD_CLIENT_ID),
      { body: commands },
    );

    console.log('Successfully reloaded application (/) commands.');
  } catch (error) {
    console.error('Error refreshing commands:', error);
  }
})(); 