import { deployFixture } from "../../fixture/fixture.deploy";
import { ethers, HDNodeWallet } from "ethers";
import { ethers as hardhatEthers } from "hardhat";

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

async function main() {
  const [deployer, validator, validator2, dataProvider, dataProvider2] =
    getKurtosisEthersWallets();

  const fixtureOutput = await deployFixture({
    deployer,
    validators: [validator.address, validator2.address],
    dataProviders: [dataProvider.address, dataProvider2.address],
    disableWhitelist: false,
  });

  console.log("deployment data:", fixtureOutput.fixtureAddresses);
}

main();