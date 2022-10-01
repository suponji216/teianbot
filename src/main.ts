import { Client, Interaction, ChatInputCommandInteraction, GatewayIntentBits, } from 'discord.js'
import commands from './commands/index';

const client = new Client({
    intents: [GatewayIntentBits.Guilds, GatewayIntentBits.GuildMembers, GatewayIntentBits.GuildMessages, GatewayIntentBits.MessageContent, GatewayIntentBits.GuildVoiceStates],
})


client.once('ready', async () => {
    await client.application?.commands.set(commands.map(item => item.commandData), '794097946391216128');
})

client.on('interactionCreate', async (interaction: Interaction) => {
    commands.forEach(async (value) => {
        if ('name' in value.commandData)
            if (value.commandData.name === (interaction as ChatInputCommandInteraction).commandName)
                value.command(interaction as ChatInputCommandInteraction);
    })
});

client.login(process.env.TOKEN).catch(error => console.log(error))