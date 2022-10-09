import { ApplicationCommandDataResolvable, ChatInputCommandInteraction } from "discord.js"

import team from './team';
import test from './test';

export type TeianbotType = {
    commandData: ApplicationCommandDataResolvable
    command: (interaction: ChatInputCommandInteraction) => Promise<void>
}

const data: Array<TeianbotType> = [team, test]

export default data;