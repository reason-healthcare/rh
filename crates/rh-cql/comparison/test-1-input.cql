library ApplicabilityLogic version '0.1.0'

using FHIR version '4.0.1'

include FHIRHelpers version '0.1.0'
include ActiveMethotrexateFeatureLogic version '0.1.0' called ActiveMethotrexateFeature
include ActiveSulfasalazineFeatureLogic version '0.1.0' called ActiveSulfasalazineFeature
include LastCbcPanelReportDateFeatureLogic version '0.1.0' called LastCbcPanelReportDateFeatureLogic


context Patient

define "On Methotrexate therapy":
  if (exists(ActiveMethotrexateFeature."On Methotrexate")) then
    ActiveMethotrexateFeature."On Methotrexate".value as boolean
  else
    false

define "On Sulfasalazine therapy":
  if (exists(ActiveSulfasalazineFeature."On Sulfasalazine")) then
    ActiveSulfasalazineFeature."On Sulfasalazine".value as boolean
  else
    false

define "Condition":
  First([Condition] M
    sort by recordedDate
  )

define "Timing of Service":
  "Condition".recordedDate.value

/**
 * @description
 * This is true if: there are no results, or the results are older than 6 months
 */
define "Missing last CBC result":
  if exists((LastCbcPanelReportDateFeatureLogic."Last CBC Panel Report Date".value as dateTime).value) then
    LastCbcPanelReportDateFeatureLogic."Last CBC Panel Report Date".value as dateTime before (Today() - 6 months)
  else
    true

define "DEBUG 1 (value)":
  (LastCbcPanelReportDateFeatureLogic."Last CBC Panel Report Date".value)

define "DEBUG 2 (value.value)":
  (LastCbcPanelReportDateFeatureLogic."Last CBC Panel Report Date".value).value

define "DEBUG 3 (exists)":
  exists(LastCbcPanelReportDateFeatureLogic."Last CBC Panel Report Date")

define "DEBUG 4 (exists value as dateTime)":
  exists(LastCbcPanelReportDateFeatureLogic."Last CBC Panel Report Date".value as dateTime)

define "DEBUG 5 (exists value as dateTime).value":
  exists((LastCbcPanelReportDateFeatureLogic."Last CBC Panel Report Date".value as dateTime).value)

/**
 * @description
 * Use case feature expressions to build out applicability
 */
define "Should order CBC if on Sulfasalazine therapy and missing test":
  if ("Missing last CBC result" and "On Sulfasalazine therapy") then
    true
  else
    false

/**
 * @description
 * Use case feature expressions to build out applicability
 */
define "Should order CBC if on Methotrexate therapy and missing test":
  if ("Missing last CBC result" and "On Methotrexate therapy") then
    true
  else
    false
