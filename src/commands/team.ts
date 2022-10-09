import { ApplicationCommandDataResolvable, ApplicationCommandType, ChannelType, ChatInputCommandInteraction, GuildMember, VoiceBasedChannel } from "discord.js";
import { TeianbotType } from "./index";

const commandData: ApplicationCommandDataResolvable = {
    name: '5v5',
    description: '5人のチームを2つ作る',
    type: ApplicationCommandType.ChatInput
}

const command = async (interaction: ChatInputCommandInteraction) => {
    const member = (interaction.member as GuildMember)
    const teams = createTeam(member.voice.channel?.members.map(member => member) as GuildMember[], member.guild.channels.cache.filter(channel => channel.isVoiceBased() && channel.id !== member.guild.afkChannelId).map(channel => channel) as VoiceBasedChannel[], 5, 5);
    for (const team of teams) await team.toChannel();
}

const data: TeianbotType = { commandData: commandData, command: command }

export default data;

class Team {
    private members: GuildMember[]

    private channel: VoiceBasedChannel

    constructor(members: GuildMember[], channel: VoiceBasedChannel) {
        this.members = members;
        this.channel = channel;
    }

    public async toChannel() {
        for (const member of this.members) await member.voice.setChannel(this.channel);
    }
}

const createTeam = (members: GuildMember[], channels: VoiceBasedChannel[], ...n: number[]) => {
    const total = n.reduce((prev, curr) => prev + curr);
    if (members.length < total) throw Error('人数が足りないヨ！');

    const membersList = getMembers(shuffler(members), n);
    const channelList = getChannels(shuffler(channels), n.length);

    const data: Team[] = [];

    for (const index in membersList) {
        data.push(new Team(membersList[index], channelList[index]));
    }

    return data;

}

const getMembers = (members: GuildMember[], n: number[]) => {
    const data: GuildMember[][] = [];
    n.reduce((prev, curr) => { data.push(members.slice(prev, prev + curr)); return prev + curr; }, 0);
    return data;
}

const getChannels = (channels: VoiceBasedChannel[], n: number) => {
    const randomIndex = getRandomIndex(n, channels.length);
    const data: VoiceBasedChannel[] = [];
    randomIndex.reduce((prev, curr) => { data.push(channels[curr]); console.log(curr); return curr; }, 0)
    return data;
}

const getRandomIndex = (n: number, max: number) => {
    const index = new Set<number>();
    while (true) {
        index.add(Math.floor(Math.random() * max))
        if ([...index].length === n) break;
    }
    return [...index];
}

const shuffler = ([...array]) => {
    for (let i = array.length - 1; i >= 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [array[i], array[j]] = [array[j], array[i]];
    }
    return array;
}