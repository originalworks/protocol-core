import { Address, BigInt, dataSource } from '@graphprotocol/graph-ts'

import { Agreement, SplitTransferToAgreement } from '../../types/schema'
import { NativeCoinReceived } from '../../types/templates/AgreementERC20/AgreementERC20'

export function _handleNativeCoinReceived<T extends NativeCoinReceived>(
  event: T,
): void {
  let agreement = Agreement.load(event.address.toHex())
  if (agreement) {
    let splitTransferToAgreement = new SplitTransferToAgreement(
      `${event.transaction.hash.toHex()}-${event.logIndex.toString()}`,
    )
    splitTransferToAgreement.agreement = agreement.id
    splitTransferToAgreement.from = event.params.from
    splitTransferToAgreement.to = agreement.address
    splitTransferToAgreement.value = event.params.amount
    splitTransferToAgreement.isRoyaltyPayout = false
    splitTransferToAgreement.inLendingCurrency = false
    splitTransferToAgreement.inNativeCoin = true
    splitTransferToAgreement.transactionHash = event.transaction.hash
    splitTransferToAgreement.timestamp = event.block.timestamp
    splitTransferToAgreement.network = dataSource.network()
    splitTransferToAgreement.payoutRequester = 'unknown'
    splitTransferToAgreement.currencyAddress = Address.zero()
    splitTransferToAgreement.fee = BigInt.zero()
    const receipt = event.receipt
    if (receipt) {
      splitTransferToAgreement.fee = receipt.gasUsed.times(
        event.transaction.gasPrice,
      )
    }
    splitTransferToAgreement.save()
  }
}
