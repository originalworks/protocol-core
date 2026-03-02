import { BigInt, dataSource } from '@graphprotocol/graph-ts';

import {
  Agreement,
  AdminAdded,
  AgreementHolder,
} from '../../types/schema';
import { AdminAdded as AdminAddedEvent } from '../../types/templates/AgreementERC20/AgreementERC20';

export function _handleAdminAdded<T extends AdminAddedEvent>(event: T): void {
  let agreement = Agreement.load(event.address.toHex());
  if (agreement != null) {
    let agreementHolder = AgreementHolder.load(`${event.address.toHex()}-${event.params.account.toHex()}`);
    if (agreementHolder != null) {
      agreementHolder.isAdmin = true;
    } else {
      agreementHolder = new AgreementHolder(`${event.address.toHex()}-${event.params.account.toHex()}`);
      agreementHolder.isAdmin = true;
      agreementHolder.balance = new BigInt(0);
      agreementHolder.network = dataSource.network();
      agreementHolder.agreement = agreement.id;
      agreementHolder.holderAddress = event.params.account;
      agreementHolder.ownedAgreementAddress = event.address;
      let agreementHolders = agreement.holders;
      agreementHolders.push(agreementHolder.id);
      agreement.holders = agreementHolders;
    }
  }
  if (!agreement) {
  } else {
    let agreementHolder = AgreementHolder.load(
      `${event.address.toHex()}-${event.params.account.toHex()}`,
    );
    if (agreementHolder) {
      agreementHolder.isAdmin = true;
    } else {
      agreementHolder = new AgreementHolder(
        `${event.address.toHex()}-${event.params.account.toHex()}`,
      );
      agreementHolder.isAdmin = true;
      agreementHolder.balance = new BigInt(0);
      agreementHolder.network = dataSource.network();
      agreementHolder.agreement = agreement.id;
      agreementHolder.holderAddress = event.params.account;
      agreementHolder.ownedAgreementAddress = event.address;
      let agreementHolders = agreement.holders;
      agreementHolders.push(agreementHolder.id);
      agreement.holders = agreementHolders;
    }
    agreement.save();
    agreementHolder.save();
  }

  let adminAdded = new AdminAdded(`${event.transaction.hash.toHex()}-${event.logIndex.toString()}`);
  adminAdded.contract = event.address;
  adminAdded.network = dataSource.network();
  adminAdded.timestamp = event.block.timestamp;
  adminAdded.blockHash = event.block.hash;
  adminAdded.agreement = agreement.id;
  adminAdded.blockNumber = event.block.number;
  adminAdded.transactionHash = event.transaction.hash;
  adminAdded.logIndex = event.logIndex;
  adminAdded.account = event.params.account;
  adminAdded.from = event.transaction.from;
  adminAdded.fee = BigInt.zero();
  const receipt = event.receipt;
  if (receipt) {
    adminAdded.fee = receipt.gasUsed.times(event.transaction.gasPrice);
  }

  adminAdded.save();
}
