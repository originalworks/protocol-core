import { dataSource } from '@graphprotocol/graph-ts'

import {
  AdminAdded as AdminAddedEvent,
  TransferSingle as TransferEvent,
  AdminRemoved as AdminRemovedEvent,
  DataHashChanged as DataHashChangedEvent,
  ContractUriChanged as ContractUriChangedEvent,
  HolderFundsClaimed as HolderFundsClaimedEvent,
  NativeCoinReceived as NativeCoinReceivedEvent,
} from '../types/templates/AgreementERC1155/AgreementERC1155'
import { Agreement, ContractUriChanged } from '../types/schema'
import { _handleAdminAdded } from './handlers/adminAdded'
import { _handleAdminRemoved } from './handlers/adminRemoved'
import { _handleDataHashChanged } from './handlers/dataHashChanged'
import { _handleAgreementTransfer } from './handlers/agreementTransfer'
import { _handleHolderFundsClaimed } from './handlers/holderFundsClaimed'
import { _handleNativeCoinReceived } from './handlers/nativeCoinReceived'

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

export function handleNativeCoinReceived(event: NativeCoinReceivedEvent): void {
  _handleNativeCoinReceived(event)
}

export function handleDataHashChanged(event: DataHashChangedEvent): void {
  _handleDataHashChanged(event)
}

export function handleContractUriChanged(event: ContractUriChangedEvent): void {
  const contractURI = event.params.contactUri
  let agreement = Agreement.load(event.address.toHex())
  if (!agreement) {
    return
  } else {
    agreement.contractURI = contractURI
    agreement.save()
  }

  let contractUriChanged = new ContractUriChanged(
    `${event.transaction.hash.toHex()}-${event.logIndex.toString()}`,
  )
  contractUriChanged.contract = event.address
  contractUriChanged.timestamp = event.block.timestamp
  contractUriChanged.network = dataSource.network()
  contractUriChanged.blockHash = event.block.hash
  contractUriChanged.blockNumber = event.block.number
  contractUriChanged.transactionHash = event.transaction.hash
  contractUriChanged.logIndex = event.logIndex
  contractUriChanged.uri = contractURI

  contractUriChanged.save()
}
