# Namespaced Value Format

The parser validates and extracts structured values using the following format:

```
<namespace>:<identifier>
```

or:

```
<namespace>:<subtype>:<identifier>
```

More generally:

```
<namespace>:<subtype>:<subtype>:...:<identifier>
```

The value is split into segments using `:`:

- The **first segment** is always treated as the **namespace**.
- The **last segment** is always treated as the **identifier**.
- Any segments between them are treated as the **subtype**.

## Examples

### Valid values

Simple namespace + identifier:

```
spotify:id
```

Parsed as:

```json
{
  "namespace": "spotify",
  "subtype": undefined,
  "identifier": "id"
}
```

Namespace with subtype:

```
spotify:track:4uLU6hMCjMI75M1A2tKUQC
```

Parsed as:

```json
{
  "namespace": "spotify",
  "subtype": "track",
  "identifier": "4uLU6hMCjMI75M1A2tKUQC"
}
```

Multiple subtype segments:

```
did:pkh:eip155:100:0x1234567890abcdef
```

Parsed as:

```json
{
  "namespace": "did",
  "subtype": "pkh:eip155:100",
  "identifier": "0x1234567890abcdef"
}
```

Other examples:

```
youtube:channel:UC_x5XG1OV2P6uZZ5FSM9Ttw

isrc:USUM72401234

ai:voice:model:v2:abc123
```

---

# Validation Rules

## General Rules

A value:

- must not be empty,
- must contain a namespace and identifier separated by `:`,
- must not contain empty segments.

## Namespace Rules

The namespace:

- is required,
- must not be empty,
- must not contain leading or trailing whitespace,
- must not contain whitespace characters.

Valid:

```
spotify:id
```

Invalid:

```
:id
```

```
 spotify:id
```

```
spot ify:id
```

---

## Subtype Rules

Subtype is optional.

If present:

- it may contain one or more segments,
- every segment must contain a value,
- every segment must not contain leading or trailing whitespace,
- every segment must not contain whitespace characters.

Valid:

```
did:pkh:eip155:100:0x123
```

Parsed:

```
namespace: did
subtype: pkh:eip155:100
identifier: 0x123
```

Invalid:

```
did::123
```

Reason:

```
Subtype contains an empty segment
```

Invalid:

```
did:pk h:123
```

Reason:

```
Subtype contains whitespace
```

---

## Identifier Rules

The identifier:

- is required,
- must not be empty,
- must not contain leading or trailing whitespace,
- must not contain whitespace characters.

Valid:

```
spotify:artist123
```

Invalid:

```
spotify:
```

Reason:

```
Identifier is empty
```

Invalid:

```
spotify:artist 123
```

Reason:

```
Identifier contains whitespace
```

---

# Invalid Examples Summary

| Value             | Reason                          |
| ----------------- | ------------------------------- |
| `spotify`         | Missing namespace delimiter `:` |
| `:id`             | Namespace is empty              |
| `spotify:`        | Identifier is empty             |
| `spotify::id`     | Empty subtype segment           |
| `spotify: artist` | Identifier contains whitespace  |
| ` spotfiy:id`     | Namespace contains whitespace   |
| `spot ify:id`     | Namespace contains whitespace   |

---

# Parser Output

For a valid value:

```
did:pkh:eip155:100:0x123
```

the parser returns:

```json
{
  "original": "did:pkh:eip155:100:0x123",
  "namespace": "did",
  "subtype": "pkh:eip155:100",
  "identifier": "0x123",
  "validationError": undefined
}
```

For invalid values:

```json
{
  "original": "spotify:",
  "validationError": "Identifier is empty"
}
```

The parser does not attempt to normalize or modify values. It only validates and extracts their components.
