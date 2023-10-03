import { ApplyOptions } from '@sapphire/decorators';
import { Listener, ListenerOptions, ok } from '@sapphire/framework';
import { VoiceState } from 'discord.js';
import { prisma } from '../lib/prisma';
import { client } from '..';

@ApplyOptions<ListenerOptions>({})
export class UserEvent extends Listener {
	private connection: { discordId: string; inout: number }[] = [];

	public override async run(oldState: VoiceState, newState: VoiceState) {
		if (oldState.channelId === null && newState.channelId !== null) {
			const { member, channel } = await this.dataCheck(newState.id, newState.channelId);
			const inout = await prisma.inout.create({ data: { userId: member.id, channelId: channel.id, type: 'connect' } });
			this.connection.push({ inout: inout.id, discordId: member.discordId });
		} else if (oldState.channelId !== null && newState.channelId === null) {
			const { member, channel } = await this.dataCheck(oldState.id, oldState.channelId);
			const pair = this.getPairId(member.discordId);
			const disconnect = await prisma.inout.create({
				data: { userId: member.id, channelId: channel.id, type: 'disconnect', pairId: pair.inout }
			});
			await prisma.inout.update({ where: { id: pair.inout }, data: { pairId: disconnect.id } });
		} else if (oldState.channelId !== null && newState.channelId !== null && oldState.channelId !== newState.channelId) {
			const oldData = await this.dataCheck(newState.id, newState.channelId);
			const newData = await this.dataCheck(oldState.id, oldState.channelId);
			const pair = this.getPairId(oldData.member.discordId);
			await prisma.inout.create({
				data: { userId: oldData.member.id, channelId: oldData.channel.id, type: 'disconnect', pairId: pair.inout }
			});
			await prisma.inout.create({ data: { userId: newData.member.id, channelId: newData.channel.id, type: 'connect' } });
		} else {
		}
	}

	private getPairId(discordId: string) {
		const pairIndex = this.connection.findIndex((c) => c.discordId === discordId);
		const pair = this.connection.slice(pairIndex, 1)[0];
		return pair;
	}

	private async dataCheck(memberId: string, channelId: string) {
		const member = await this.memberCheck(memberId);
		const channel = await this.channelCheck(channelId);
		return { member: member.unwrap(), channel: channel.unwrap() };
	}

	private async memberCheck(memberId: string) {
		const member = await client.users.fetch(memberId);
		try {
			const user = await prisma.user.findUniqueOrThrow({ where: { discordId: memberId } });
			if (member.displayName !== user.name) {
				await prisma.user.update({ where: { id: user.id }, data: { name: member.displayName } });
			}
			return ok(user);
		} catch {
			const user = await prisma.user.create({ data: { discordId: memberId, name: member.displayName } });
			return ok(user);
		}
	}

	private async channelCheck(channelId: string) {
		const dicsordCh = await client.channels.fetch(channelId);
		try {
			const channel = await prisma.channel.findUniqueOrThrow({ where: { discordId: channelId } });
			if (dicsordCh?.isVoiceBased() && dicsordCh.name !== channel.name) {
				await prisma.channel.update({ where: { id: channel.id }, data: { name: dicsordCh.name } });
			}
			return ok(channel);
		} catch {
			const channel = await prisma.channel.create({
				data: { discordId: channelId, type: 'voice', name: dicsordCh?.isVoiceBased() ? dicsordCh.name : '' }
			});
			return ok(channel);
		}
	}
}
