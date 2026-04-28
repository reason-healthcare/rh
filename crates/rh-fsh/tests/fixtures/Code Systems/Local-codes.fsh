// @Name: Local Code Systems

// Define a local code system
CodeSystem: LymphNodeCS
Id:         lymph-node-cs
Title:     "Lymph node code system"
Description: "Lymph node terms that could not be found in standard sources such as SNOMED CT."
// For information about creating a code system in the `terminology.hl7.org` namespace, see
// this documentation on the UTG process: https://confluence.hl7.org/display/VMAH/Vocabulary+Maintenance+at+HL7
// You can also create a URL local to your Implementation Guide like `http://hl7.org/fhir/uv/your-ig/CodeSystem/lymphnodes`
// The "Terminology" stream on chat.fhir.org (free registration required) is a good place to ask
// questions about how to handle this for your specific use case (direct link: https://bit.ly/terminology-fhir-chat)
* ^url =  "http://terminology.hl7.org/CodeSystem/lymphnodes"
// Spacing layout over three lines per term is optional, for clarity
// The definition (second text string) is optional
* #LN_AX_L1
    "Level 1 Axillary Lymph Nodes"
    "Level 1 is the bottom level, below the lower edge of the pectoralis minor muscle."
* #LN_Ax_L2
    "Level 2 Axillary Lymph Nodes"
    "Level II is lying underneath the pectoralis minor muscle."
* #LN_Ax_L3
    "Level 3 Axillary Lymph Nodes"
    "Level III is above the pectoralis minor muscle."
* #LN_Neck_IA
    "Level IA (Submental) neck node"
    "Lymph nodes within the triangular boundary of the anterior belly of the digastric muscles and the hyoid bone"
* #LN_Neck_IB
    "Level IB (Submandibular) neck node"
    "Lymph nodes within the boundaries of the anterior and posterior bellies of the digastric muscles, the stylohyoid muscle, and the body of the mandible"
// truncated (you get the idea)

// Here is a short example of how to use those codes
Instance: Lymph-nodes
InstanceOf: BodyStructure
* patient = Reference(adam-everyman)
* location = LymphNodeCS#LN_Neck_IA "Level IA (Submental) neck node"
* morphology = http://snomed.info/sct#40225001 "Cystic lymphangioma (morphologic abnormality)"

// Note that BodyStruture.patient has a cardinality of 1..1, so it must be included
// (https://www.hl7.org/fhir/bodystructure.html). The resource below allows us to meet this
// conformance requirement.

Instance: adam-everyman
InstanceOf: Patient
Description: "Example of Patient"
* name.family = "Everyman"
* name.given[0] = "Adam"
* gender = #male
* birthDate = "1962-02-05"
