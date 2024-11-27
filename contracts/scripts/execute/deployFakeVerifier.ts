import { deployFakeGroth16Verifier } from "../actions/contract-deployment/FakeGroth16Verifier/FakeGroth16Verifier.deploy";

async function main() {
  const verifier = await deployFakeGroth16Verifier();
  console.log({ verifierAddress: await verifier.getAddress() });
}

main();
