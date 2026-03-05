import {
  Transfer as TransferEvent,
  AdminAdded as AdminAddedEvent,
  AdminRemoved as AdminRemovedEvent,
  HolderFundsClaimed as HolderFundsClaimedEvent,
} from '../types/templates/AgreementERC20/AgreementERC20'
import { _handleAdminAdded } from './handlers/adminAdded'
import { _handleAdminRemoved } from './handlers/adminRemoved'
import { _handleAgreementTransfer } from './handlers/agreementTransfer'
import { _handleHolderFundsClaimed } from './handlers/holderFundsClaimed'

export function handleAgreementTransfer(event: TransferEvent): void {
  _handleAgreementTransfer(event)
}

export function handleAdminAdded(event: AdminAddedEvent): void {
  _handleAdminAdded(event)
}

export function handleAdminRemoved(event: AdminRemovedEvent): void {
  _handleAdminRemoved(event)
}

export function handleHolderFundsClaimed(event: HolderFundsClaimedEvent): void {
  _handleHolderFundsClaimed(event)
}
