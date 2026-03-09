import { BigInt, dataSource } from '@graphprotocol/graph-ts'

import {
  Agreement,
  AgreementHolder,
} from '../../types/schema'
import { AdminAdded as AdminAddedEvent } from '../../types/templates/AgreementERC20/AgreementERC20'

export function _handleAdminAdded<T extends AdminAddedEvent>(event: T): void {
  const agreement = Agreement.load(event.address.toHex())

  if (agreement != null) {
    let agreementHolder = AgreementHolder.load(`${event.address.toHex()}-${event.params.account.toHex()}`)

    if (agreementHolder != null) {
      agreementHolder.isAdmin = true
    } else {
      agreementHolder = new AgreementHolder(
        `${event.address.toHex()}-${event.params.account.toHex()}`,
      )
      agreementHolder.isAdmin = true
      agreementHolder.balance = new BigInt(0)
      agreementHolder.network = dataSource.network()
      agreementHolder.agreement = agreement.id
      agreementHolder.holderAddress = event.params.account
      agreementHolder.ownedAgreementAddress = event.address
      let agreementHolders = agreement.holders
      agreementHolders.push(agreementHolder.id)
      agreement.holders = agreementHolders
    }
    agreement.save()
    agreementHolder.save()
  }
}
