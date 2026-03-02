import { Transfer as AgreementTransferEvent } from '../types/templates/AgreementERC20/AgreementERC20';
import { _handleAgreementTransfer } from './handlers/agreementTransfer';

export function handleAgreementTransfer(event: AgreementTransferEvent): void {
  _handleAgreementTransfer(event)
}
