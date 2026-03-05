import { Bytes, BigInt, dataSource } from '@graphprotocol/graph-ts'

import { checkForRoyaltyPayoutTx } from '../../utils'
import { Agreement, HolderFundsClaimed, SplitCurrency } from '../../types/schema'
import { HolderFundsClaimed as HolderFundsClaimedEvent } from '../../types/templates/AgreementERC20/AgreementERC20'

export function _handleHolderFundsClaimed<T extends HolderFundsClaimedEvent>(
  event: T,
): void {
  let splitCurrency = SplitCurrency.load(event.params.currency.toHex())
  let agreement = Agreement.load(event.address.toHex())
  if (agreement && splitCurrency) {
    let holderFundsClaimed = new HolderFundsClaimed(
      `${event.transaction.hash.toHex()}-${event.logIndex.toString()}`,
    )
    const isRoyaltyPayout = checkForRoyaltyPayoutTx(event.transaction.input)
    holderFundsClaimed.contract = event.address
    holderFundsClaimed.agreement = agreement.id
    holderFundsClaimed.network = dataSource.network()
    holderFundsClaimed.timestamp = event.block.timestamp
    holderFundsClaimed.blockHash = event.block.hash
    holderFundsClaimed.blockNumber = event.block.number
    holderFundsClaimed.transactionHash = event.transaction.hash
    holderFundsClaimed.logIndex = event.logIndex
    holderFundsClaimed.account = event.params.account
    holderFundsClaimed.value = event.params.value
    holderFundsClaimed.inLendingCurrency = splitCurrency.isLendingCurrency
    holderFundsClaimed.inNativeCoin = splitCurrency.isNativeCoin
    holderFundsClaimed.currencyAddress = splitCurrency.currencyAddress
    holderFundsClaimed.isRoyaltyPayout = isRoyaltyPayout
    if (isRoyaltyPayout) {
      holderFundsClaimed.originalPayoutTxHash = Bytes.fromUint8Array(
        event.transaction.input.subarray(68),
      )
    } else {
      holderFundsClaimed.originalPayoutTxHash = Bytes.empty()
    }

    holderFundsClaimed.fee = BigInt.zero()
    const receipt = event.receipt
    if (receipt) {
      holderFundsClaimed.fee = receipt.gasUsed.times(event.transaction.gasPrice)
    }

    holderFundsClaimed.save()
  }
}
