use std::fmt;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct ValidationTimings {
    pub base_validation: Duration,
    pub profile_snapshot_lookup: Duration,
    pub rule_compilation_cache_lookup: Duration,
    pub unknown_property_validation: Duration,
    pub cardinality_validation: Duration,
    pub type_validation: Duration,
    pub reference_target_validation: Duration,
    pub fixed_pattern_validation: Duration,
    pub binding_validation: Duration,
    pub invariant_validation: Duration,
    pub expression_datatype_validation: Duration,
    pub extension_validation: Duration,
    pub slicing_validation: Duration,
    pub known_code_terminology_local_checks: Duration,
    pub ucum_validation: Duration,
    pub known_coding_code_validation: Duration,
    pub us_core_ethnicity_validation: Duration,
    pub coding_system_uri_validation: Duration,
}

impl ValidationTimings {
    pub fn total_timed(&self) -> Duration {
        self.base_validation
            + self.profile_snapshot_lookup
            + self.rule_compilation_cache_lookup
            + self.unknown_property_validation
            + self.cardinality_validation
            + self.type_validation
            + self.reference_target_validation
            + self.fixed_pattern_validation
            + self.binding_validation
            + self.invariant_validation
            + self.expression_datatype_validation
            + self.extension_validation
            + self.slicing_validation
    }
}

impl fmt::Display for ValidationTimings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Validation timing breakdown:")?;
        writeln!(f, "  base validation: {:.3} ms", ms(self.base_validation))?;
        writeln!(
            f,
            "  profile snapshot lookup: {:.3} ms",
            ms(self.profile_snapshot_lookup)
        )?;
        writeln!(
            f,
            "  rule compilation/cache lookup: {:.3} ms",
            ms(self.rule_compilation_cache_lookup)
        )?;
        writeln!(
            f,
            "  unknown-property validation: {:.3} ms",
            ms(self.unknown_property_validation)
        )?;
        writeln!(
            f,
            "  cardinality validation: {:.3} ms",
            ms(self.cardinality_validation)
        )?;
        writeln!(f, "  type validation: {:.3} ms", ms(self.type_validation))?;
        writeln!(
            f,
            "  reference target validation: {:.3} ms",
            ms(self.reference_target_validation)
        )?;
        writeln!(
            f,
            "  fixed/pattern validation: {:.3} ms",
            ms(self.fixed_pattern_validation)
        )?;
        writeln!(
            f,
            "  binding validation: {:.3} ms",
            ms(self.binding_validation)
        )?;
        writeln!(
            f,
            "  invariant validation: {:.3} ms",
            ms(self.invariant_validation)
        )?;
        writeln!(
            f,
            "  expression datatype validation: {:.3} ms",
            ms(self.expression_datatype_validation)
        )?;
        writeln!(
            f,
            "  extension validation: {:.3} ms",
            ms(self.extension_validation)
        )?;
        writeln!(
            f,
            "  slicing validation: {:.3} ms",
            ms(self.slicing_validation)
        )?;
        writeln!(
            f,
            "  known-code/terminology local checks within base: {:.3} ms",
            ms(self.known_code_terminology_local_checks)
        )?;
        writeln!(f, "    UCUM validation: {:.3} ms", ms(self.ucum_validation))?;
        writeln!(
            f,
            "    known coding code validation: {:.3} ms",
            ms(self.known_coding_code_validation)
        )?;
        writeln!(
            f,
            "    US Core ethnicity validation: {:.3} ms",
            ms(self.us_core_ethnicity_validation)
        )?;
        writeln!(
            f,
            "    coding system URI validation: {:.3} ms",
            ms(self.coding_system_uri_validation)
        )?;
        write!(
            f,
            "  total timed sections: {:.3} ms",
            ms(self.total_timed())
        )
    }
}

fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}
