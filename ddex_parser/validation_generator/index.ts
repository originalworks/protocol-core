import { Parser } from "xml2js"
import { readFileSync, writeFileSync } from "fs"

const buildStruct = (name: string, validation: string) => {
    return `
        #[allow(dead_code)]
        pub struct ${name}Validator;
        impl ${name}Validator {
            #[allow(dead_code)]
            pub fn validate(value: &String) -> bool {
                let validation = r"${validation}";
                Regex::new(validation).unwrap().is_match(value)
            }
        }`
}

const buildMod = (structs: string) => {
    return `
        use regex::Regex;
        
        ${structs}
    `
}

interface Restriction {
    restriction: string
    type: "enum"|"pattern"
    nodeInfo?: NodeInfo
}

class NodeInfo {
    public rawPath: string
    public path: string[]
    public restrictedNodeName?: string
    public restrictedNodeTag?: string

    constructor() {
        this.rawPath = "root"
        this.path = ["root"]
    }

    clone = () => {
        const cloned = new NodeInfo()
        cloned.path = [...this.path]
        cloned.rawPath = this.rawPath
        cloned.restrictedNodeName = this.restrictedNodeName
        cloned.restrictedNodeTag = this.restrictedNodeTag
        return cloned
    }
}

const buildRestriction = (node: any): Restriction | undefined => {
    if (node["xs:restriction"]) {
        const resNode = node["xs:restriction"]
        if (Array.isArray(resNode) && resNode.length === 1) {
             const restriction = resNode[0]
             const enumeration = restriction["xs:enumeration"]?.map((en: any) => {
                 return `^${en["$"].value}$`
             }).join("|")
             const pattern = `^${restriction["xs:pattern"]?.[0]["$"].value}$`

             return {
                restriction: enumeration || pattern, 
                type: !!enumeration ? "enum" : "pattern"
            }
        }
     }
}

interface RestrictionRegistry {
    restrictions: Restriction[],
    registry: {
        [k: string]: boolean
    }
}

const registry: RestrictionRegistry = {
    restrictions: [],
    registry: {}
} 

// restrictionsLog is used to produce structs without duplicates,
// debugLog collects all appearances of each validation so it can be compared with schema
const findRestrictionRecursive = (
    element: any, 
    nodeInfo = new NodeInfo(), 
    restrictionsLog: RestrictionRegistry = structuredClone(registry), 
    debugLog: RestrictionRegistry = structuredClone(registry)
) => {
    if (Array.isArray(element)) {
        for (let i = 0; i < element.length; i++) {
            findRestrictionRecursive(element[i], nodeInfo, restrictionsLog, debugLog)
        }
    } else {
        const newNodeInfo = nodeInfo.clone()

        const name = element["$"]?.name
        if (name) {
            newNodeInfo.rawPath = `${nodeInfo.rawPath}(${name})`
            newNodeInfo.restrictedNodeName = name.charAt(0).toUpperCase() + name.slice(1)
            newNodeInfo.restrictedNodeTag = newNodeInfo.path[newNodeInfo.path.length - 1]
        }

        const restriction = buildRestriction(element)
        if (restriction) {
            restriction.nodeInfo = newNodeInfo
            debugLog.restrictions.push(restriction)
            if (newNodeInfo.restrictedNodeName && !restrictionsLog.registry[newNodeInfo.restrictedNodeName]) {
                restrictionsLog.restrictions.push(restriction)
                restrictionsLog.registry[newNodeInfo.restrictedNodeName] = true  
            }
            return 
        }

        Object.entries(element).forEach(([key, value]) => {
            if (key.startsWith("xs:")) {
                newNodeInfo.rawPath = `${newNodeInfo.rawPath}::${key}`
                newNodeInfo.path.push(key)
            }

            if (typeof value === "string") {
                return
            }
            
            findRestrictionRecursive(value, newNodeInfo, restrictionsLog, debugLog)
        })
    }
    return {restrictionsLog, debugLog}
}


const main = async () => {
    const rawXml = readFileSync("ddex_schema/xml/flattened/schema.xsd")
    new Parser().parseString(rawXml, (err, res) => {
        const result = findRestrictionRecursive(res)

        if (!result) {
            throw new Error("No result")
        }
        const {restrictionsLog, debugLog} = result

        // Build module
        const structs = restrictionsLog.restrictions.map((item) => buildStruct(item.nodeInfo?.restrictedNodeName || '', item.restriction))
        const module = buildMod(structs.join('\n'))
        writeFileSync("ddex_schema/src/validation.rs",module)
        
        // Debug
        const sorted = debugLog.restrictions.sort((a, b) =>  {
            const nameA = a.nodeInfo?.restrictedNodeName!
            const nameB = b.nodeInfo?.restrictedNodeName!
            return nameA.localeCompare(nameB)
        })
        writeFileSync("validation_generator/debug.json", JSON.stringify(sorted, null, 2))
    })
}

void main()