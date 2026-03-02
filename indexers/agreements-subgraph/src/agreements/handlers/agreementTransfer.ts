import { BigInt, Address, dataSource, store } from '@graphprotocol/graph-ts';

import {
  Agreement,
  RegisteredUser,
  AgreementHolder,
  AgreementCreated,
  AgreementTransfer,
} from '../../types/schema';
import { Transfer } from '../../types/templates/AgreementERC20/AgreementERC20';

export function _handleAgreementTransfer<T extends Transfer>(event: T) {
  let agreement = Agreement.load(event.address.toHex());
  let agreementCreated = AgreementCreated.load(event.address.toHex());

  if (agreement != null && agreementCreated != null) {
    if (event.params.from.toHex() == Address.zero().toHex()) {
      agreement.totalSupply = agreement.totalSupply.plus(event.params.value);
    }

    const agreementTransfer = new AgreementTransfer(`${event.transaction.hash.toHex()}-${event.logIndex.toString()}`);
    agreementTransfer.contract = event.address;
    agreementTransfer.agreement = agreement.id;
    agreementTransfer.timestamp = event.block.timestamp;
    agreementTransfer.blockHash = event.block.hash;
    agreementTransfer.network = dataSource.network();
    agreementTransfer.blockNumber = event.block.number;
    agreementTransfer.transactionHash = event.transaction.hash;
    agreementTransfer.logIndex = event.logIndex;
    agreementTransfer.from = event.params.from;
    agreementTransfer.to = event.params.to;
    agreementTransfer.value = event.params.value;
    agreementTransfer.fee = BigInt.zero();
    const receipt = event.receipt;
    if (receipt != null) {
      agreementTransfer.fee = receipt.gasUsed.times(event.transaction.gasPrice);
    }
    agreementTransfer.save();

    let agreementHolders = agreement.holders;
    let agreementTransferReceiver = AgreementHolder.load(`${event.address.toHex()}-${event.params.to.toHex()}`);

    if (agreementTransferReceiver == null) {
      agreementTransferReceiver = new AgreementHolder(`${event.address.toHex()}-${event.params.to.toHex()}`);
      agreementTransferReceiver.balance = event.params.value;
      agreementTransferReceiver.network = dataSource.network();
      agreementTransferReceiver.isAdmin = false;
      agreementTransferReceiver.holderAddress = event.params.to;
      agreementTransferReceiver.ownedAgreementAddress = event.address;
      agreementTransferReceiver.agreement = agreement.id;
      agreementTransferReceiver.save();

      let registeredUser = RegisteredUser.load(event.params.to.toHex());
      if (registeredUser == null) {
        registeredUser = new RegisteredUser(event.params.to.toHex());
        registeredUser.registrationTimestamp = event.block.timestamp;
        registeredUser.network = dataSource.network();
        registeredUser.save();
      }

      agreementHolders.push(agreementTransferReceiver.id);
      agreement.holders = agreementHolders;
      agreementCreated.holders = agreementHolders;
    } else {
      agreementTransferReceiver.balance = agreementTransferReceiver.balance.plus(event.params.value);
      agreementTransferReceiver.holderAddress = event.params.to;
      agreementTransferReceiver.ownedAgreementAddress = event.address;
      agreementTransferReceiver.save();
    }

    let agreementTransferSender = AgreementHolder.load(
      `${event.address.toHex()}-${event.params.from.toHex()}`,
    )
    if (agreementTransferSender != null) {
      const currentBalance = agreementTransferSender.balance;
      if (
        currentBalance == event.params.value &&
        !agreementTransferSender.isAdmin
      ) {
        for (let i = 0; i < agreement.holders.length; i++) {
          if (agreement.holders[i] == agreementTransferSender.id) {
            agreementHolders.splice(i, 1);
          }
        }
        agreement.holders = agreementHolders;
        agreementCreated.holders = agreementHolders;
        store.remove(
          'AgreementHolder',
          `${event.address.toHex()}-${event.params.from.toHex()}`,
        );
      } else {
        agreementTransferSender.balance = currentBalance.minus(event.params.value);
        agreementTransferSender.save();
      }
    }

    const agreementTransfers = agreement.transfers;
    agreementTransfers.push(agreementTransfer.id);
    agreement.transfers = agreementTransfers;
    agreement.save();
    agreementCreated.save();
  }
}
