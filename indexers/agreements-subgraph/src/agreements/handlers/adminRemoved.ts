import { BigInt, dataSource, store } from '@graphprotocol/graph-ts';

import {
  Agreement,
  AdminRemoved,
  AgreementHolder,
} from '../../types/schema';
import { AdminRemoved as AdminRemovedEvent } from '../../types/templates/AgreementERC20/AgreementERC20';

export function _handleAdminRemoved<T extends AdminRemovedEvent>(event: T): void {
  let agreement = Agreement.load(event.address.toHex());
  if (agreement != null) {
    let agreementHolder = AgreementHolder.load(`${event.address.toHex()}-${event.params.account.toHex()}`);
    if (agreementHolder != null) {
      if (agreementHolder.balance == BigInt.zero()) {
        let agreementHolders = agreement.holders;
        for (let i = 0; i < agreement.holders.length; i++) {
          if (agreement.holders[i] == agreementHolder.id) {
            agreementHolders.splice(i, 1);
          }
        }
        agreement.holders = agreementHolders;
        agreement.save();
        store.remove(
          'AgreementHolder',
          `${event.address.toHex()}-${event.params.account.toHex()}`,
        );
      } else {
        agreementHolder.isAdmin = false;
        agreementHolder.save();
      }
    }
  }

  let adminRemoved = new AdminRemoved(`${event.transaction.hash.toHex()}-${event.logIndex.toString()}`);
  adminRemoved.contract = event.address;
  adminRemoved.network = dataSource.network();
  adminRemoved.timestamp = event.block.timestamp;
  adminRemoved.blockHash = event.block.hash;
  adminRemoved.agreement = agreement.id;
  adminRemoved.blockNumber = event.block.number;
  adminRemoved.transactionHash = event.transaction.hash;
  adminRemoved.logIndex = event.logIndex;
  adminRemoved.account = event.params.account;
  adminRemoved.from = event.transaction.from;
  adminRemoved.fee = BigInt.zero();
  const receipt = event.receipt;
  if (receipt != null) {
    adminRemoved.fee = receipt.gasUsed.times(event.transaction.gasPrice);
  }

  adminRemoved.save();
}
