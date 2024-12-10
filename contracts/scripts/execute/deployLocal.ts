import { deployDdexSequencer } from "../actions/contract-deployment/DdexSequencer/DdexSequencer.deploy";
import { deployOwnToken } from "../actions/contract-deployment/OwnToken/OwnToken.deploy";
import { deployStakeVault } from "../actions/contract-deployment/StakeVault/StakeVault.deploy";
import { deployWhitelist } from "../actions/contract-deployment/Whitelist/Whitelist.deploy";
import { getKurtosisEthersWallets } from "../fixture/fixture.deploy";
import { deployVerifier } from "../actions/contract-deployment/Verifier/Verifier.deploy";
import { deployFakeGroth16Verifier } from "../actions/contract-deployment/FakeGroth16Verifier/FakeGroth16Verifier.deploy";

const SLASH_RATE = 1000;

async function main() {
  const [signer, validator, validator2, dataProvider, dataProvider2] =
    getKurtosisEthersWallets();

  console.log("Deploying whitelists...");
  const dataProvidersWhitelist = await deployWhitelist(signer, [
    dataProvider.address,
    dataProvider2.address,
  ]);
  const validatorsWhitelist = await deployWhitelist(signer, [
    validator.address,
    validator2.address,
  ]);

  console.log("Deploying DDEX sequencer...");
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

  console.log("Deploying Verifier...");
  const fakeGroth16Verifier = await deployFakeGroth16Verifier();

  const verifier = await deployVerifier(
    await ddexSequencer.getAddress(),
    await fakeGroth16Verifier.getAddress()
  );

  await ddexSequencer.setVerifier(verifier);

  console.log("deployment data:", {
    ddexSequencer: await ddexSequencer.getAddress(),
    accounts: {
      deployer: await signer.getAddress(),
      validators: [validator.address, validator2.address],
      dataProviders: [dataProvider.address, dataProvider2.address],
    },
    whitelists: {
      dataProvidersWhitelist: await dataProvidersWhitelist.getAddress(),
      validatorsWhitelist: await validatorsWhitelist.getAddress(),
    },
  });
}

main();
