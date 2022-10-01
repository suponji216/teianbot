import { ApplicationCommandDataResolvable, ApplicationCommandType, ChatInputCommandInteraction } from "discord.js";
import { TeianbotType } from "./type";

const commandData: ApplicationCommandDataResolvable = {
    name: 'test',
    description: 'test',
    type: ApplicationCommandType.ChatInput
}

const command = async (interaction: ChatInputCommandInteraction) => {

}

const data: TeianbotType = { commandData: commandData, command: command }

export default data;