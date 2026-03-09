import { BigInt, dataSource } from '@graphprotocol/graph-ts'

import { Agreement, SplitTransferToAgreement } from './types/schema'
import { Transfer as TokenTransfer } from './types/templates/ERC20Token/ERC20TokenMock'

export function handleERC20TokenTransfer(event: TokenTransfer): void {
  const agreement = Agreement.load(event.params.to.toHex())

  if (agreement != null) {
    // TODO: add logic to get isRoyaltyPayout and payoutRequester when available
    const splitTransferToAgreement = new SplitTransferToAgreement(
      `${event.transaction.hash.toHex()}-${event.logIndex.toString()}`,
    )
    splitTransferToAgreement.agreement = agreement.id
    splitTransferToAgreement.from = event.params.from
    splitTransferToAgreement.to = event.params.to
    splitTransferToAgreement.isRoyaltyPayout = false
    splitTransferToAgreement.inNativeCoin = false
    splitTransferToAgreement.value = event.params.value
    splitTransferToAgreement.currencyAddress = event.address
    splitTransferToAgreement.transactionHash = event.transaction.hash
    splitTransferToAgreement.timestamp = event.block.timestamp
    splitTransferToAgreement.network = dataSource.network()
    splitTransferToAgreement.payoutRequester = 'unknown'
    splitTransferToAgreement.fee = BigInt.zero()
    const receipt = event.receipt
    if (receipt != null) {
      splitTransferToAgreement.fee = receipt.gasUsed.times(
        event.transaction.gasPrice,
      )
    }
    splitTransferToAgreement.save()
  }
}
