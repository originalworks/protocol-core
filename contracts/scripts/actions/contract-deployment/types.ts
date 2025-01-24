import { ContractVerificationInput } from "../verify/verifyContracts.types";

export interface DeploymentOutput<ContractType> {
  contract: ContractType;
  contractVerificationInput: ContractVerificationInput;
}
