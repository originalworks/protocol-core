import hre from 'hardhat'

const main = async () => {
    const [signer] = await hre.ethers.getSigners()
    const tx = await signer.sendTransaction({to: "0x26E9568a58A5FD4437e7674d857DC3B2b1d30DDd", value: hre.ethers.parseEther("2")})
    console.log(tx.hash)
    await tx.wait()
    console.log("done")
}

void main()