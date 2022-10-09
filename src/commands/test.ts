import { ApplicationCommandDataResolvable, ApplicationCommandType, ChatInputCommandInteraction } from "discord.js";
import { TeianbotType } from "./index";

const commandData: ApplicationCommandDataResolvable = {
    name: 'test',
    description: 'test',
    type: ApplicationCommandType.ChatInput,
    defaultMemberPermissions: "Administrator"
}

const command = async (interaction: ChatInputCommandInteraction) => {
    interaction.reply('test')
}

const data: TeianbotType = { commandData: commandData, command: command }

export default data;