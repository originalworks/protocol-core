import {
  TypedMap,
  JSONValue,
  JSONValueKind,
} from "@graphprotocol/graph-ts";

export function getValueIfExist(
  sourceObject: TypedMap<string, JSONValue>,
  parameterName: string,
): string | null {
  const jsonValue = sourceObject.get(parameterName);
  if (jsonValue && jsonValue.kind == JSONValueKind.STRING) {
    return jsonValue.toString();
  } else {
    return null;
  }
}

export function getObject(value: JSONValue | null): TypedMap<string, JSONValue> | null {
  return value && value.kind == JSONValueKind.OBJECT ? value.toObject() : null;
}

export function getArray(value: JSONValue | null): JSONValue[] | null {
  return value && value.kind == JSONValueKind.ARRAY ? value.toArray() : null;
}

export function deduplicateStringList(input: string): string {
  let items = input.split(",");
  let seen = new Map<string, boolean>();
  let unique: string[] = [];

  for (let i = 0; i < items.length; i++) {
    let item = items[i].trim();
    if (!seen.has(item)) {
      seen.set(item, true);
      unique.push(item);
    }
  }

  return unique.join(", ");
}
