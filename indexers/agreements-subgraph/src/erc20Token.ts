import { BigInt, dataSource } from '@graphprotocol/graph-ts'

import { Transfer as TokenTransfer } from './types/templates/ERC20Token/ERC20TokenMock'
import {
  Agreement,
  PaymentBatcher,
  RegisteredUser,
  SplitCurrency,
  SplitTransferToAgreement,
  TrackedTokenTransfer
} from './types/schema';

export function handleERC20TokenTransfer(event: TokenTransfer): void {
  let agreement = Agreement.load(event.params.to.toHex())
  if (agreement) {
    const currency = SplitCurrency.load(event.address.toHex())
    const paymentBatcher = PaymentBatcher.load(event.params.from.toHex())
    let isRoyaltyPayout = false
    if (paymentBatcher) {
      isRoyaltyPayout = true
    }
    let splitTransferToAgreement = new SplitTransferToAgreement(
      `${event.transaction.hash.toHex()}-${event.logIndex.toString()}`,
    )
    splitTransferToAgreement.agreement = agreement.id
    splitTransferToAgreement.from = event.params.from
    splitTransferToAgreement.to = event.params.to
    splitTransferToAgreement.isRoyaltyPayout = isRoyaltyPayout
    splitTransferToAgreement.inNativeCoin = false
    splitTransferToAgreement.value = event.params.value
    splitTransferToAgreement.currencyAddress = event.address
    splitTransferToAgreement.transactionHash = event.transaction.hash
    splitTransferToAgreement.timestamp = event.block.timestamp
    splitTransferToAgreement.network = dataSource.network()
    if (paymentBatcher) {
      splitTransferToAgreement.payoutRequester = paymentBatcher.tenantName
    } else {
      splitTransferToAgreement.payoutRequester = 'unknown'
    }
    if (currency) {
      splitTransferToAgreement.inLendingCurrency = currency.isLendingCurrency
    } else {
      splitTransferToAgreement.inLendingCurrency = false
    }
    splitTransferToAgreement.fee = BigInt.zero()
    const receipt = event.receipt
    if (receipt) {
      splitTransferToAgreement.fee = receipt.gasUsed.times(
        event.transaction.gasPrice,
      )
    }
    splitTransferToAgreement.save()
  } else {
    let registeredUserReceiver = RegisteredUser.load(
      event.params.to.toHex(),
    )
    let registeredUserSender = RegisteredUser.load(
      event.params.from.toHex(),
    )

    if (registeredUserReceiver || registeredUserSender) {
      let trackedTokenTransfer = new TrackedTokenTransfer(
        `${event.transaction.hash.toHex()}-${event.logIndex.toString()}`,
      )
      trackedTokenTransfer.contract = event.address
      trackedTokenTransfer.from = event.params.from
      trackedTokenTransfer.network = dataSource.network()
      trackedTokenTransfer.to = event.params.to
      trackedTokenTransfer.value = event.params.value
      trackedTokenTransfer.timestamp = event.block.timestamp
      trackedTokenTransfer.blockHash = event.block.hash
      trackedTokenTransfer.blockNumber = event.block.number
      trackedTokenTransfer.transactionHash = event.transaction.hash
      trackedTokenTransfer.logIndex = event.logIndex
      trackedTokenTransfer.fee = BigInt.zero()
      const receipt = event.receipt
      if (receipt) {
        trackedTokenTransfer.fee = receipt.gasUsed.times(
          event.transaction.gasPrice,
        )
      }
      trackedTokenTransfer.save()
    }
  }
}
