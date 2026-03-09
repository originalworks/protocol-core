import { Address, BigInt, dataSource, store } from '@graphprotocol/graph-ts'

import { SplitCurrency } from './types/schema'
import { ERC20Token } from './types/templates'
import { CurrencyAdded, CurrencyRemoved } from './types/CurrencyManager/CurrencyManager'

export function handleCurrencyAdded(event: CurrencyAdded): void {
  let splitCurrency = SplitCurrency.load(event.params.currencyAddress.toHex())

  if (splitCurrency == null) {
    splitCurrency = new SplitCurrency(event.params.currencyAddress.toHex())
    splitCurrency.addedAt = event.block.timestamp
    splitCurrency.currencyAddress = event.params.currencyAddress
    splitCurrency.network = dataSource.network()
    if (event.params.currencyAddress.toHex() == Address.zero().toHex()) {
      splitCurrency.isNativeCoin = true
      splitCurrency.symbol = dataSource.network() === 'polygon' ? 'MATIC' : 'ETH'
      splitCurrency.decimals = BigInt.fromString('18')
    } else {
      splitCurrency.symbol = event.params.symbol
      splitCurrency.decimals = BigInt.fromI32(event.params.decimals)
      splitCurrency.isNativeCoin = false

      ERC20Token.create(event.params.currencyAddress)
    }
    splitCurrency.save()
  }
}

export function handleCurrencyRemoved(event: CurrencyRemoved): void {
  store.remove('SplitCurrency', event.params.currencyAddress.toHex())
}
