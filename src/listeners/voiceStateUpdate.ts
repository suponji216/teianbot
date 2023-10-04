import { ApplyOptions } from '@sapphire/decorators';
import { Listener, ListenerOptions, ok, err } from '@sapphire/framework';
import { VoiceState } from 'discord.js';
import { prisma } from '../lib/prisma';
import { client } from '..';

type Channel = {
	id: number;
	discordId: string;
	type: string;
	name: string | null;
};

type Member = {
	id: number;
	discordId: string;
	name: string | null;
};

@ApplyOptions<ListenerOptions>({})
export class UserEvent extends Listener {
	private connection: { discordId: string; inout: number }[] = [];

	public override async run(oldState: VoiceState, newState: VoiceState) {
		if (oldState.channelId === null && newState.channelId !== null) {
			const { member, channel } = await this.dataCheck(newState.id, newState.channelId);
			await this.connect(member, channel);
		} else if (oldState.channelId !== null && newState.channelId === null) {
			const { member, channel } = await this.dataCheck(oldState.id, oldState.channelId);
			await this.disconnect(member, channel);
		} else if (oldState.channelId !== null && newState.channelId !== null && oldState.channelId !== newState.channelId) {
			const oldData = await this.dataCheck(newState.id, newState.channelId);
			const newData = await this.dataCheck(oldState.id, oldState.channelId);
			await this.move(oldData, newData);
		} else {
		}
	}

	private async connect(member: Member, channel: Channel) {
		try {
			const inout = await prisma.inout.create({ data: { userId: member.id, channelId: channel.id, type: 'connect' } });
			this.connection.push({ inout: inout.id, discordId: member.discordId });
			return ok();
		} catch {
			return err();
		}
	}
	private async disconnect(member: Member, channel: Channel) {
		const pair = this.getPairId(member.discordId);
		if (pair.isOk()) {
			const disconnect = await prisma.inout.create({
				data: { userId: member.id, channelId: channel.id, type: 'disconnect', pairId: pair.unwrap().inout }
			});
			await prisma.inout.update({ where: { id: pair.unwrap().inout }, data: { pairId: disconnect.id } });
		}
	}

	private async move(oldData: { member: Member; channel: Channel }, newData: { member: Member; channel: Channel }) {
		await this.disconnect(oldData.member, oldData.channel);
		await this.connect(newData.member, newData.channel);
	}

	private getPairId(discordId: string) {
		try {
			const pairIndex = this.connection.findIndex((c) => c.discordId === discordId);
			if (pairIndex < 0) throw Error;
			const pair = this.connection.splice(pairIndex, 1)[0];
			return ok(pair);
		} catch {
			return err();
		}
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
