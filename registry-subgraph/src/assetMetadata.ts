import { dataSource, Bytes, json, log } from "@graphprotocol/graph-ts";
import { AssetMetadata } from "./types/schema";

/**
 * Invoked automatically by the 'file/ipfs' dataSource template,
 * passing in the raw file content as `content: Bytes`.
 */
export function handleAssetMetadata(content: Bytes): void {
  // This is the “parameter” we passed to .create(…)
  // for IPFS data sources, this stringParam() is the full <CID>/some/path
  let ipfsPath = dataSource.stringParam();
  log.info("Handling IPFS file data source: {}", [ipfsPath]);

  // Convert raw bytes to string
  let rawContent = content.toString();

  // Optionally parse as JSON. If the IPFS file is truly raw JSON, this works.
  let jsonResult = json.try_fromString(rawContent);
  if (jsonResult.isError) {
    log.warning("Could not parse JSON from IPFS file: {}", [ipfsPath]);
    return;
  }

  // In real code, parse fields out of jsonResult.value.toObject()…

  // Create an entity to store everything
  let entity = new AssetMetadata(ipfsPath);
  entity.rawContent = rawContent;
  entity.save();

  log.info("Saved AssetMetadata with id: {}", [ipfsPath]);
}
