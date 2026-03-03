import { Address, BigInt, dataSource } from '@graphprotocol/graph-ts'

import {
  Agreement,
  AgreementCreated,
  AgreementImplementation,
  AgreementImplementationChanged,
} from './types/schema'
import {
  AgreementCreated as AgreementCreatedEvent,
  AgreementImplementationChanged as AgreementImplementationChangedEvent,
} from './types/AgreementFactory/AgreementFactory'

export function handleAgreementImplementationChanged(event: AgreementImplementationChangedEvent): void {
  let agreementImplementationChanged = new AgreementImplementationChanged(
    `${event.transaction.hash.toHex()}-${event.logIndex.toString()}`,
  )
  agreementImplementationChanged.contract = event.address
  agreementImplementationChanged.network = dataSource.network()
  agreementImplementationChanged.timestamp = event.block.timestamp
  agreementImplementationChanged.blockHash = event.block.hash
  agreementImplementationChanged.blockNumber = event.block.number
  agreementImplementationChanged.transactionHash = event.transaction.hash
  agreementImplementationChanged.logIndex = event.logIndex
  agreementImplementationChanged.agreementImplementation = event.params.agreementImplementation
  agreementImplementationChanged.save()

  let agreementImplementation = AgreementImplementation.load(event.params.agreementImplementation.toHex())

  if (!agreementImplementation) {
    agreementImplementation = new AgreementImplementation(event.params.agreementImplementation.toHex())
    agreementImplementation.network = dataSource.network()
    agreementImplementation.timestamp = event.block.timestamp
    agreementImplementation.tokenStandard = event.params.tokenStandard
    agreementImplementation.implementationAddress = event.params.agreementImplementation
    agreementImplementation.save()
  }
}

export function handleAgreementCreated(event: AgreementCreatedEvent): void {
  const agreement = new Agreement(event.params.agreementAddress.toHex())
  agreement.address = event.params.agreementAddress
  agreement.network = dataSource.network()
  agreement.factoryContract = event.address
  agreement.tokenStandard = event.params.tokenStandard
  agreement.timestamp = event.block.timestamp
  agreement.creationTxHash = event.transaction.hash
  agreement.transfers = []
  agreement.totalSupply = BigInt.zero()
  agreement.holders = []
  agreement.rwaID = event.params.rwaId
  agreement.save()

  const receipt = event.receipt
  const agreementCreated = new AgreementCreated(event.params.agreementAddress.toHex())
  if (receipt) {
    agreementCreated.fee = receipt.gasUsed.times(event.transaction.gasPrice)
  }
  agreementCreated.contract = event.address
  agreementCreated.timestamp = event.block.timestamp
  agreementCreated.blockHash = event.block.hash
  agreementCreated.network = dataSource.network()
  agreementCreated.blockNumber = event.block.number
  agreementCreated.transactionHash = event.transaction.hash
  agreementCreated.logIndex = event.logIndex
  agreementCreated.tokenStandard = event.params.tokenStandard
  agreementCreated.agreement = agreement.id
  agreementCreated.holders = []
  agreementCreated.save()

  // TODO: add templates based on tokenStandard
  // if (event.params.tokenStandard == 0) {}
}
