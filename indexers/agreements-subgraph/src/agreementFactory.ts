import { BigInt, dataSource } from '@graphprotocol/graph-ts'

import { Agreement, AgreementCreated } from './types/schema'
import { AgreementERC1155, AgreementERC20 } from './types/templates'
import { AgreementCreated as AgreementCreatedEvent } from './types/AgreementFactory/AgreementFactory'

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
  if (receipt != null) {
    agreementCreated.fee = receipt.gasUsed.times(event.transaction.gasPrice)
  }
  agreementCreated.save()

  // TODO: add templates based on tokenStandard
  if (event.params.tokenStandard == 0) {
    AgreementERC20.create(event.params.agreementAddress)
  } else {
    AgreementERC1155.create(event.params.agreementAddress)
  }
}
