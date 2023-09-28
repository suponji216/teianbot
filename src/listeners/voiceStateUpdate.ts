import { ApplyOptions } from '@sapphire/decorators';
import { Listener, ListenerOptions, ok } from '@sapphire/framework';
import { VoiceState } from 'discord.js';
import { prisma } from '../lib/prisma';

@ApplyOptions<ListenerOptions>({})
export class UserEvent extends Listener {
	public override async run(oldState: VoiceState, newState: VoiceState) {
		if (oldState.channelId === null && newState.channelId !== null) {
			const { member, channel } = await this.dataCheck(newState.id, newState.channelId);
			await prisma.inout.create({ data: { userId: member.id, channelId: channel.id, type: 'connect' } });
		} else if (oldState.channelId !== null && newState.channelId === null) {
			const { member, channel } = await this.dataCheck(oldState.id, oldState.channelId);
			await prisma.inout.create({ data: { userId: member.id, channelId: channel.id, type: 'disconnect' } });
		} else if (oldState.channelId !== null && newState.channelId !== null && oldState.channelId !== newState.channelId) {
			const oldData = await this.dataCheck(newState.id, newState.channelId);
			const newData = await this.dataCheck(oldState.id, oldState.channelId);
			await prisma.inout.create({ data: { userId: oldData.member.id, channelId: oldData.channel.id, type: 'disconnect' } });
			await prisma.inout.create({ data: { userId: newData.member.id, channelId: newData.channel.id, type: 'connect' } });
		} else {
		}
	}

	private async dataCheck(memberId: string, channelId: string) {
		const member = await this.memberCheck(memberId);
		const channel = await this.channelCheck(channelId);
		return { member: member.unwrap(), channel: channel.unwrap() };
	}

	private async memberCheck(memberId: string) {
		try {
			const user = await prisma.user.findUniqueOrThrow({ where: { discordId: memberId } });
			return ok(user);
		} catch {
			const user = await prisma.user.create({ data: { discordId: memberId } });
			return ok(user);
		}
	}

	private async channelCheck(channelId: string) {
		try {
			const channel = await prisma.channel.findUniqueOrThrow({ where: { discordId: channelId } });
			return ok(channel);
		} catch {
			const channel = await prisma.channel.create({ data: { discordId: channelId, type: 'voice' } });
			return ok(channel);
		}
	}
}
