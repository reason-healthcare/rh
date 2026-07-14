// @Name: DTR and mCODE-shaped Instances
// @Description: Parameters, questionnaires, embedded Bundle resources, and medication dosage instances representative of DTR and mCODE output.

Alias: $RxNorm = http://www.nlm.nih.gov/research/umls/rxnorm

Instance: DTRQuestionnaire
InstanceOf: Questionnaire
Usage: #inline
* id = "dtr-questionnaire"
* url = "http://example.org/Questionnaire/dtr-questionnaire"
* status = #active
* item[+].linkId = "medication"
* item[=].text = "Medication"
* item[=].type = #string

Instance: DTRParameters
InstanceOf: Parameters
Usage: #example
* parameter[+].name = "questionnaire"
* parameter[=].resource = DTRQuestionnaire
* parameter[+].name = "patient"
* parameter[=].valueReference = Reference(Patient/example)
* parameter[+].name = "request-id"
* parameter[=].valueString = "request-123"

Instance: DTRQuestionnaireResponse
InstanceOf: QuestionnaireResponse
Usage: #example
* status = #completed
* questionnaire = Canonical(DTRQuestionnaire)
* subject = Reference(Patient/example)
* item[+].linkId = "medication"
* item[=].text = "Medication"
* item[=].answer[+].valueString = "Example medication"

Instance: DTRQuestionnaireBundle
InstanceOf: Bundle
Usage: #example
* type = #collection
* entry[+].fullUrl = "http://example.org/Questionnaire/dtr-questionnaire"
* entry[=].resource = DTRQuestionnaire
* entry[+].fullUrl = "http://example.org/QuestionnaireResponse/dtr-questionnaire-response"
* entry[=].resource = DTRQuestionnaireResponse

Instance: MCODENMedicationRequest
InstanceOf: MedicationRequest
Usage: #example
* status = #active
* intent = #order
* medicationCodeableConcept = $RxNorm#1049502 "Acetaminophen 325 MG Oral Tablet"
* subject = Reference(Patient/example)
* dosageInstruction[+].text = "Take one tablet by mouth daily"
* dosageInstruction[=].timing.repeat.frequency = 1
* dosageInstruction[=].timing.repeat.period = 1
* dosageInstruction[=].timing.repeat.periodUnit = #d
* dosageInstruction[=].doseAndRate[+].doseQuantity = 325 'mg'
