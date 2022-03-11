import { BaseQueryNodeFixture, FixtureRunner } from '../../Fixture'
import { Debugger, extendDebug } from '../../Debugger'
import { JoystreamCLI, ICreatedVideoData } from '../../cli/joystream'
import { PaidTermId, MemberId } from '@joystream/types/members'
import { QueryNodeApi } from '../../QueryNodeApi'
import { Api } from '../../Api'
import * as path from 'path'
import { getMemberDefaults, getVideoDefaults, getChannelDefaults } from './contentTemplates'
import { KeyringPair } from '@polkadot/keyring/types'
import { BuyMembershipHappyCaseFixture } from '../membershipModule'
import BN from 'bn.js'
import { DataObjectId, StorageBucketId } from '@joystream/types/storage'
import { Worker, WorkerId } from '@joystream/types/working-group'
import { createType } from '@joystream/types'
import { singleBucketConfig } from '../../flows/storagev2/initStorage'

interface IMember {
  keyringPair: KeyringPair
  account: string
  memberId: MemberId
}

// settings
const sufficientTopupAmount = new BN(1000000) // some very big number to cover fees of all transactions

const cliExamplesFolderPath = path.dirname(require.resolve('@joystream/cli/package.json')) + '/examples/content'

export class CreateChannelsAndVideosFixture extends BaseQueryNodeFixture {
  private paidTerms: PaidTermId
  private debug: Debugger.Debugger
  private cli: JoystreamCLI
  private channelCount: number
  private videoCount: number
  private channelCategoryId: number
  private videoCategoryId: number
  private createdItems: {
    channelIds: number[]
    videosData: ICreatedVideoData[]
  }

  constructor(
    api: Api,
    query: QueryNodeApi,
    cli: JoystreamCLI,
    paidTerms: PaidTermId,
    channelCount: number,
    videoCount: number,
    channelCategoryId: number,
    videoCategoryId: number
  ) {
    super(api, query)
    this.paidTerms = paidTerms
    this.cli = cli
    this.channelCount = channelCount
    this.videoCount = videoCount
    this.channelCategoryId = channelCategoryId
    this.videoCategoryId = videoCategoryId
    this.debug = extendDebug('fixture:CreateChannelsAndVideosFixture')

    this.createdItems = {
      channelIds: [],
      videosData: [],
    }
  }

  public getCreatedItems() {
    return this.createdItems
  }

  /*
    Execute this Fixture.
  */
  public async execute(): Promise<void> {
    this.debug('Creating members')
    const author = await this.prepareAuthor()

    this.debug('Creating channels')
    this.createdItems.channelIds = await this.createChannels(this.channelCount, this.channelCategoryId, author.account)

    this.debug('Creating videos')
    this.createdItems.videosData = await this.createVideos(
      this.videoCount,
      this.createdItems.channelIds[0],
      this.videoCategoryId
    )
  }

  /**
    Prepares author for the content to be created.
  */
  private async prepareAuthor(): Promise<IMember> {
    // prepare memberships
    const members = await this.createMembers(1)

    const authorMemberIndex = 0
    const author = members[authorMemberIndex]
    author.keyringPair.setMeta({
      ...author.keyringPair.meta,
      ...getMemberDefaults(authorMemberIndex),
    })

    this.debug('Top-uping accounts')
    await this.api.treasuryTransferBalanceToAccounts([author.keyringPair.address], sufficientTopupAmount)

    return author
  }

  /**
    Creates new accounts and registers memberships for them.
  */
  private async createMembers(numberOfMembers: number): Promise<IMember[]> {
    const keyringPairs = (await this.api.createKeyPairs(numberOfMembers)).map((kp) => kp.key)
    const accounts = keyringPairs.map((item) => item.address)
    const buyMembershipsFixture = new BuyMembershipHappyCaseFixture(this.api, accounts, this.paidTerms)

    await new FixtureRunner(buyMembershipsFixture).run()

    const memberIds = buyMembershipsFixture.getCreatedMembers()

    return keyringPairs.map((item, index) => ({
      keyringPair: item,
      account: accounts[index],
      memberId: memberIds[index],
    }))
  }

  /**
    Creates a new channel.
  */
  private async createChannels(count: number, channelCategoryId: number, authorAddress: string): Promise<number[]> {
    const createdIds = await this.createCommonEntities(count, (index) =>
      this.cli.createChannel({
        ...getChannelDefaults(index, authorAddress),
        category: channelCategoryId,
      })
    )

    return createdIds
  }

  /**
    Creates a new video.

    Note: Assets have to be accepted later on for videos to be counted as active.
  */
  private async createVideos(count: number, channelId: number, videoCategoryId: number): Promise<ICreatedVideoData[]> {
    const createVideo = async (index: number) => {
      return await this.cli.createVideo(channelId, {
        ...getVideoDefaults(index, cliExamplesFolderPath),
        category: videoCategoryId,
      })
    }
    const newVideosData = (await this.createCommonEntities(count, createVideo)) as ICreatedVideoData[]

    return newVideosData
  }

  /**
    Creates a bunch of content entities.
  */
  private async createCommonEntities<T>(count: number, createPromise: (index: number) => Promise<T>): Promise<T[]> {
    const createdIds = await Array.from(Array(count).keys()).reduce(async (accPromise, index: number) => {
      const acc = await accPromise
      const createdId = await createPromise(index)

      return [...acc, createdId]
    }, Promise.resolve([]) as Promise<T[]>)

    return createdIds
  }
}