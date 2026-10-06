import hre from 'hardhat'

const main = async () => {
    const [signer] = await hre.ethers.getSigners()
    const tx = await signer.sendTransaction({to: "", value: hre.ethers.parseEther("2")})
    console.log(tx.hash)
    await tx.wait()
    console.log("done")
}

void main()