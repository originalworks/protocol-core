import { dataSource } from '@graphprotocol/graph-ts'

import { ConnectedAgreement } from '../../types/schema'
import { DataHashChanged } from '../../types/templates/AgreementERC20/AgreementERC20'

export function _handleDataHashChanged<T extends DataHashChanged>(
  event: T,
): void {
  let connectedAgreement = new ConnectedAgreement(
    `${event.address.toHex()}-${event.block.number}-${event.logIndex}`,
  )
  connectedAgreement.connectionTimestamp = event.block.timestamp
  connectedAgreement.agreementMetadata = event.params.dataHash
  connectedAgreement.agreement = event.address.toHex()
  connectedAgreement.transactionHash = event.transaction.hash
  connectedAgreement.network = dataSource.network()
  connectedAgreement.save()
  // TODO: uncomment when added agreement metadata template
  // AgreementMetadataTemplate.create(event.params.dataHash)
}
