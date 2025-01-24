# Parser

## Contents

1. ddex_schema - core of the project, it has all structs needed to deserialize xml
2. yaserde & yaserde_derive - dependencies of `ddex_schema` that were customized to support regex validation
3. validation_generator - generator used to generate... validation out of xml. Uses ts.
4. resources - sample ddex messages along with ern xml schema (stripped from descriptions and flattened)
5. runner - binary file to play around with the parser.
