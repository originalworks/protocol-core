import { deployDdexSequencer } from "../actions/contract-deployment/DdexSequencer/DdexSequencer.deploy";
import { deployOwnToken } from "../actions/contract-deployment/OwnToken/OwnToken.deploy";
import { deployStakeVault } from "../actions/contract-deployment/StakeVault/StakeVault.deploy";
import { deployWhitelist } from "../actions/contract-deployment/Whitelist/Whitelist.deploy";
import { ethers } from "hardhat";
import { getKurtosisEthersWallets } from "../fixture/fixture.deploy";

const SLASH_RATE = 1000;

async function main() {
  const [signer, validator, validator2, dataProvider, dataProvider2] =
    getKurtosisEthersWallets();

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

  console.log({
    token: await ownToken.getAddress(),
    deployer: await signer.getAddress(),
    validator: validator.address,
    validator2: validator2.address,
    dataProvider: dataProvider.address,
    dataProvider2: dataProvider2.address,
    ddexSequencer: await ddexSequencer.getAddress(),
    ownToken: await ownToken.getAddress(),
    dataProvidersWhitelist: await dataProvidersWhitelist.getAddress(),
    validatorsWhitelist: await validatorsWhitelist.getAddress(),
  });
}

main();
