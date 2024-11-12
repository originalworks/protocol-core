import { deployDdexSequencer } from "../actions/contract-deployment/DdexSequencer/DdexSequencer.deploy";
import { deployOwnToken } from "../actions/contract-deployment/OwnToken/OwnToken.deploy";
import { deployStakeVault } from "../actions/contract-deployment/StakeVault/StakeVault.deploy";
import { deployWhitelist } from "../actions/contract-deployment/Whitelist/Whitelist.deploy";
import { ethers as hardhatEthers } from "hardhat";
import { ethers, Signer, HDNodeWallet } from "ethers";
import { FixtureOutput } from "./fixture.types";

const SLASH_RATE = 1000;

export async function deployFixture(): Promise<FixtureOutput> {
  const [signer] = await hardhatEthers.getSigners();
  const dataProvider = await getEthersWalletWithFunds(signer);
  const dataProvider2 = await getEthersWalletWithFunds(signer);
  const validator = await getEthersWalletWithFunds(signer);
  const validator2 = await getEthersWalletWithFunds(signer);

  const dataProvidersWhitelist = await deployWhitelist(signer, [
    dataProvider.address,
    dataProvider2.address,
  ]);
  const validatorsWhitelist = await deployWhitelist(signer, [
    validator.address,
    validator2.address,
  ]);
  const ownToken = await deployOwnToken();
  const stakeVault = await deployStakeVault({
    stakeTokenAddress: await ownToken.getAddress(),
    _slashRate: SLASH_RATE,
  });
  const ddexSequencer = await deployDdexSequencer({
    dataProvidersWhitelist: await dataProvidersWhitelist.getAddress(),
    validatorsWhitelist: await validatorsWhitelist.getAddress(),
    stakeVaultAddress: await stakeVault.getAddress(),
  });

  return {
    deployer: signer,
    dataProvidersWhitelist,
    validatorsWhitelist,
    ownToken,
    stakeVault,
    ddexSequencer,
    dataProviders: [dataProvider, dataProvider2],
    validators: [validator, validator2],
  };
}

// it's necessary to use ethers.Wallet instead of hardhatEthers.Wallet
// as only the first one currently supports type 3 EIP4844 transaction
export async function getEthersWalletWithFunds(
  fundsSource: Signer
): Promise<HDNodeWallet> {
  const wallet = ethers.Wallet.createRandom(hardhatEthers.provider);
  const tx = await fundsSource.sendTransaction({
    to: wallet,
    value: ethers.parseEther("5"),
  });
  await tx.wait();
  return wallet;
}

// it's necessary to use ethers.Wallet instead of hardhatEthers.Wallet
// as only the first one currently supports type 3 EIP4844 transaction
// This function works only with Kurtosis testnet setup!!!!
export function getKurtosisEthersWallets(): HDNodeWallet[] {
  const phrase = `${process.env.KURTOSIS_MNEMONIC}`;
  const mnemonic = ethers.Mnemonic.fromPhrase(phrase);
  const wallets: HDNodeWallet[] = [];
  for (let i = 0; i < 20; i++) {
    wallets.push(
      ethers.HDNodeWallet.fromMnemonic(mnemonic, `m/44'/60'/0'/0/${i}`).connect(
        hardhatEthers.provider
      )
    );
  }

  return wallets;
}
