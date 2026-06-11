library DevDaysMeasure version '1.0.0'
using FHIR version '4.0.1'
include FHIRHelpers version '4.0.1'

parameter "Measurement Period" Interval<DateTime>
  default Interval[@2024-01-01, @2024-12-31]

context Patient

// ── Initial Population ───────────────────────────────────────────────────────

define "In Age Range":
  AgeInYearsAt(start of "Measurement Period") in Interval[18, 75]

define "Initial Population":
  "In Age Range"

// ── Stratifiers ──────────────────────────────────────────────────────────────

define "Is Adult":
  AgeInYearsAt(start of "Measurement Period") >= 18

define "Is Senior":
  AgeInYearsAt(start of "Measurement Period") >= 65

// ── Denominator / Numerator ──────────────────────────────────────────────────

define "Denominator":
  "Initial Population"

define "Numerator":
  "Initial Population" and "Is Adult"
