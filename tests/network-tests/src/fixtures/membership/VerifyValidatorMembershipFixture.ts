import Long from 'long'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { ISubmittableResult } from '@polkadot/types/types/'
import { WorkerId, MemberId } from '@joystream/types/primitives'
import { RemarkMetadataAction } from '@joystream/metadata-protobuf'
import { Api } from '../../Api'
import { QueryNodeApi } from '../../QueryNodeApi'
import { AnyQueryNodeEvent, EventDetails } from '../../types'
import { Utils } from '../../utils'
import { MembershipFieldsFragment } from '../../graphql/generated/queries'
import { WithMembershipWorkersFixture } from './WithMembershipWorkersFixture'

export type VerifyValidatorInput = {
  memberId: MemberId
  isVerified: boolean
  asWorker?: WorkerId
  expectFailure?: boolean
}

export class VerifyValidatorMembershipFixture extends WithMembershipWorkersFixture {
  protected verifications: VerifyValidatorInput[]

  public constructor(api: Api, query: QueryNodeApi, verifications: VerifyValidatorInput[]) {
    super(api, query)
    this.verifications = verifications
  }

  protected async getSignerAccountOrAccounts(): Promise<string[]> {
    return this.getSignersFromInput(this.verifications)
  }

  protected async getExtrinsics(): Promise<SubmittableExtrinsic<'promise'>[]> {
    return this.verifications.map((u) => {
      const metadata = Utils.metadataToBytes(RemarkMetadataAction, {
        verifyValidator: { memberId: Long.fromString(u.memberId.toString()), isVerified: u.isVerified },
      })
      return u.memberId
        ? this.api.tx.membershipWorkingGroup.workerRemark(u.memberId, metadata)
        : this.api.tx.membershipWorkingGroup.leadRemark(metadata)
    })
  }

  protected async getEventFromResult(result: ISubmittableResult): Promise<EventDetails> {
    if (this.api.findEvent(result, 'membershipWorkingGroup', 'WorkerRemarked')) {
      return this.api.getEventDetails(result, 'membershipWorkingGroup', 'WorkerRemarked')
    } else {
      return this.api.getEventDetails(result, 'membershipWorkingGroup', 'LeadRemarked')
    }
  }

  private assertQueriedMembershipsAreValid(qMembers: MembershipFieldsFragment[]): void {
    this.events.map((e, i) => {
      const verification = this.verifications[i]
      if (verification.expectFailure) return

      const qMembership = qMembers.find((p) => p.id === verification.memberId.toString())
      Utils.assert(qMembership, 'Query node: Membership not found')
    })
  }

  protected assertQueryNodeEventIsValid(qEvent: AnyQueryNodeEvent, i: number): void {
    // TODO: implement QN checks
  }

  async runQueryNodeChecks(): Promise<void> {
    await super.runQueryNodeChecks()
    const qMembers = await this.query.getMembersByIds(this.verifications.map((m) => m.memberId))
    this.assertQueriedMembershipsAreValid(qMembers)
  }
}
