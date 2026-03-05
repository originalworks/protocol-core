import { Bytes } from '@graphprotocol/graph-ts'

export function checkForRoyaltyPayoutTx(input: Bytes): boolean {
  return input.length == 100
}
