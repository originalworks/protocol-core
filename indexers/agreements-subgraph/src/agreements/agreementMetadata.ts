import {
  json,
  Bytes,
  TypedMap,
  JSONValue,
  dataSource,
} from '@graphprotocol/graph-ts'

import { AgreementMetadata } from '../types/schema'

export function handleAgreementMetadata(content: Bytes): void {
  let agreementMetadata = new AgreementMetadata(dataSource.stringParam())

  agreementMetadata.rawContent = content.toString()

  const metadata = json.fromBytes(content).toObject()
  if (metadata) {
    const type = metadata.get('type')

    if (type) {
      agreementMetadata.type = type.toString()
      if (type.toString() == 'AGREEMENT') {
        agreementMetadata.title = getValueIfExist(metadata, 'title')
      } else if (type.toString() == 'ASSET') {
        const factsheetJSONValue = metadata.get('factsheet')
        if (factsheetJSONValue) {
          const factsheetObject = factsheetJSONValue.toObject()
          agreementMetadata.isrc = getValueIfExist(factsheetObject, 'isrc')
          agreementMetadata.assetURI = getValueIfExist(
            factsheetObject,
            'assetURI',
          )
          agreementMetadata.ownerURI = getValueIfExist(
            factsheetObject,
            'ownerURI',
          )
          agreementMetadata.title = getValueIfExist(factsheetObject, 'name')
        }
      }
    }
  }
  agreementMetadata.save()
}

function getValueIfExist(
  sourceObject: TypedMap<string, JSONValue>,
  parameterName: string,
): string | null {
  const jsonValue = sourceObject.get(parameterName)
  if (jsonValue) {
    return jsonValue.toString()
  } else {
    return null
  }
}
