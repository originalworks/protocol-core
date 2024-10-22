1. Original schemas taken from:
  - http://ddex.net/xml/ern/43 (ern)
  - http://ddex.net/xml/allowed-value-sets (avs)
2. All `<xs:annotation>` tags and it's children were removed to lower schema size
3. ERN and AVS were flattened into one schema:
  - `<xs:import>` tag was removed from ERN
  - `avs:...` names were replaced by `Avs...`
  - `_` were removed from names in both AVS and ERN

Problem: Complex types extending simple types with enum restriction creates `content` field twice and enum validation is ignored.
Example:
```
  <xs:complexType name="SoundRecordingType">
        <xs:simpleContent>
            <xs:extension base="AvsSoundRecordingType">
                <xs:attribute name="Namespace" type="xs:string">
                </xs:attribute>
                <xs:attribute name="UserDefinedValue" type="xs:string">
                </xs:attribute>
            </xs:extension>
        </xs:simpleContent>
    </xs:complexType>

    <xs:simpleType name="AvsSoundRecordingType">
        <xs:restriction base="xs:string">
            <xs:enumeration value="AudioStem">
            </xs:enumeration>
            <xs:enumeration value="Clip">
            </xs:enumeration>
            <xs:enumeration value="MusicalWorkReadalongSoundRecording">
            </xs:enumeration>
            <xs:enumeration value="MusicalWorkSoundRecording">
            </xs:enumeration>
            <xs:enumeration value="NonMusicalWorkReadalongSoundRecording">
            </xs:enumeration>
            <xs:enumeration value="NonMusicalWorkSoundRecording">
            </xs:enumeration>
            <xs:enumeration value="SpokenWordSoundRecording">
            </xs:enumeration>
            <xs:enumeration value="Unknown">
            </xs:enumeration>
            <xs:enumeration value="UserDefined">
            </xs:enumeration>
        </xs:restriction>
    </xs:simpleType>
```

compiles to

```
pub struct SoundRecordingType {
            pub content: AvsSoundRecordingType,
            #[yaserde(attribute, rename = "Namespace")]
            pub namespace: Option<String>,
            #[yaserde(attribute, rename = "UserDefinedValue")]
            pub user_defined_value: Option<String>,
        }

 pub struct AvsSoundRecordingType {
            #[yaserde(text)]
            pub content: std::string::String,
        }
```

With this structure schema deserialisation fails even for valid document.

Temporary fix: 

```
pub struct SoundRecordingType {
            #[yaserde(text)] // replaced custom struct
            pub content: string,
            #[yaserde(attribute, rename = "Namespace")]
            pub namespace: Option<String>,
            #[yaserde(attribute, rename = "UserDefinedValue")]
            pub user_defined_value: Option<String>,
        }

```
