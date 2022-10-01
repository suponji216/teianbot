import { ApplicationCommandDataResolvable, ChatInputCommandInteraction } from "discord.js"

export type TeianbotType = {
    commandData: ApplicationCommandDataResolvable
    command: (interaction: ChatInputCommandInteraction) => void
}