#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementPath {
    parts: Vec<String>,
    original: String,
}

impl ElementPath {
    pub fn new(path: &str) -> Self {
        let parts: Vec<String> = path.split('.').map(|s| s.to_string()).collect();
        Self {
            parts,
            original: path.to_string(),
        }
    }

    pub fn parts(&self) -> &[String] {
        &self.parts
    }

    pub fn as_str(&self) -> &str {
        &self.original
    }

    pub fn depth(&self) -> usize {
        self.parts.len()
    }

    pub fn is_parent_of(&self, other: &ElementPath) -> bool {
        if self.depth() >= other.depth() {
            return false;
        }

        for (i, part) in self.parts.iter().enumerate() {
            if other.parts.get(i) != Some(part) {
                return false;
            }
        }

        true
    }

    pub fn is_child_of(&self, other: &ElementPath) -> bool {
        other.is_parent_of(self)
    }

    pub fn is_immediate_child_of(&self, parent: &ElementPath) -> bool {
        self.depth() == parent.depth() + 1 && self.is_child_of(parent)
    }

    pub fn parent(&self) -> Option<ElementPath> {
        if self.parts.len() <= 1 {
            return None;
        }

        let parent_parts = self.parts[0..self.parts.len() - 1].to_vec();
        let parent_path = parent_parts.join(".");
        Some(ElementPath::new(&parent_path))
    }

    pub fn matches_choice_type(&self, base_path: &ElementPath) -> bool {
        if self.depth() != base_path.depth() {
            return false;
        }

        for i in 0..self.parts.len() - 1 {
            if self.parts[i] != base_path.parts[i] {
                return false;
            }
        }

        let base_last = base_path.parts.last().unwrap();
        let self_last = self.parts.last().unwrap();

        if base_last.ends_with("[x]") {
            let base_prefix = &base_last[..base_last.len() - 3];
            self_last.starts_with(base_prefix)
        } else {
            false
        }
    }

    fn is_lowercase_start(s: &str) -> bool {
        s.chars().next().is_some_and(|c| c.is_lowercase())
    }

    pub fn normalize_choice_type(&self) -> ElementPath {
        let mut normalized_parts = self.parts.clone();
        if let Some(last) = normalized_parts.last_mut() {
            if last.len() > 3 && Self::is_lowercase_start(last) {
                for (i, c) in last.char_indices() {
                    if c.is_uppercase() {
                        let prefix = &last[..i];
                        *last = format!("{prefix}[x]");
                        break;
                    }
                }
            }
        }
        let normalized_path = normalized_parts.join(".");
        ElementPath::new(&normalized_path)
    }

    pub fn is_slice(&self) -> bool {
        self.original.contains(':')
    }

    pub fn slice_name(&self) -> Option<&str> {
        for part in &self.parts {
            if let Some(colon_pos) = part.rfind(':') {
                return Some(&part[colon_pos + 1..]);
            }
        }
        None
    }

    pub fn base_path(&self) -> ElementPath {
        if !self.is_slice() {
            return self.clone();
        }

        let base_parts: Vec<String> = self
            .parts
            .iter()
            .map(|part| {
                if let Some(colon_pos) = part.rfind(':') {
                    part[..colon_pos].to_string()
                } else {
                    part.clone()
                }
            })
            .collect();

        let base_path = base_parts.join(".");
        ElementPath::new(&base_path)
    }

    pub fn is_reslice(&self) -> bool {
        if let Some(last_part) = self.parts.last() {
            last_part.matches(':').count() > 1
        } else {
            false
        }
    }

    pub fn parent_slice(&self) -> Option<ElementPath> {
        if !self.is_reslice() {
            return None;
        }

        if let Some(last_part) = self.parts.last() {
            let colon_pos = last_part.rfind(':').unwrap();
            let mut parent_parts = self.parts.clone();
            parent_parts.pop();
            parent_parts.push(last_part[..colon_pos].to_string());
            let parent_path = parent_parts.join(".");
            return Some(ElementPath::new(&parent_path));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_parsing() {
        let path = ElementPath::new("Patient.name.given");
        assert_eq!(path.parts(), &["Patient", "name", "given"]);
        assert_eq!(path.as_str(), "Patient.name.given");
        assert_eq!(path.depth(), 3);
    }

    #[test]
    fn test_single_part_path() {
        let path = ElementPath::new("Patient");
        assert_eq!(path.parts(), &["Patient"]);
        assert_eq!(path.depth(), 1);
    }

    #[test]
    fn test_is_parent_of() {
        let parent = ElementPath::new("Patient.name");
        let child = ElementPath::new("Patient.name.given");
        let not_child = ElementPath::new("Patient.identifier");

        assert!(parent.is_parent_of(&child));
        assert!(!parent.is_parent_of(&not_child));
        assert!(!parent.is_parent_of(&parent));
    }

    #[test]
    fn test_is_child_of() {
        let parent = ElementPath::new("Patient.name");
        let child = ElementPath::new("Patient.name.given");

        assert!(child.is_child_of(&parent));
        assert!(!parent.is_child_of(&child));
    }

    #[test]
    fn test_is_immediate_child_of() {
        let parent = ElementPath::new("Patient.name");
        let immediate_child = ElementPath::new("Patient.name.given");
        let grandchild = ElementPath::new("Patient.name.given.extension");

        assert!(immediate_child.is_immediate_child_of(&parent));
        assert!(!grandchild.is_immediate_child_of(&parent));
        assert!(!parent.is_immediate_child_of(&immediate_child));
    }

    #[test]
    fn test_parent() {
        let path = ElementPath::new("Patient.name.given");
        let parent = path.parent().unwrap();
        assert_eq!(parent.as_str(), "Patient.name");

        let root = ElementPath::new("Patient");
        assert!(root.parent().is_none());
    }

    #[test]
    fn test_matches_choice_type() {
        let base = ElementPath::new("Observation.value[x]");
        let string_variant = ElementPath::new("Observation.valueString");
        let quantity_variant = ElementPath::new("Observation.valueQuantity");
        let codeable_variant = ElementPath::new("Observation.valueCodeableConcept");
        let other = ElementPath::new("Observation.status");

        assert!(string_variant.matches_choice_type(&base));
        assert!(quantity_variant.matches_choice_type(&base));
        assert!(codeable_variant.matches_choice_type(&base));
        assert!(!other.matches_choice_type(&base));
    }

    #[test]
    fn test_normalize_choice_type() {
        let string_path = ElementPath::new("Observation.valueString");
        let normalized = string_path.normalize_choice_type();
        assert_eq!(normalized.as_str(), "Observation.value[x]");

        let quantity_path = ElementPath::new("Observation.valueQuantity");
        let normalized = quantity_path.normalize_choice_type();
        assert_eq!(normalized.as_str(), "Observation.value[x]");

        let codeable_path = ElementPath::new("Observation.valueCodeableConcept");
        let normalized = codeable_path.normalize_choice_type();
        assert_eq!(normalized.as_str(), "Observation.value[x]");
    }

    #[test]
    fn test_normalize_non_choice_type() {
        let normal_path = ElementPath::new("Patient.name");
        let normalized = normal_path.normalize_choice_type();
        assert_eq!(normalized.as_str(), "Patient.name");
    }

    #[test]
    fn test_multi_level_parent_child() {
        let root = ElementPath::new("Patient");
        let level1 = ElementPath::new("Patient.name");
        let level2 = ElementPath::new("Patient.name.given");
        let level3 = ElementPath::new("Patient.name.given.extension");

        assert!(root.is_parent_of(&level1));
        assert!(root.is_parent_of(&level2));
        assert!(root.is_parent_of(&level3));

        assert!(level1.is_parent_of(&level2));
        assert!(level1.is_parent_of(&level3));

        assert!(level2.is_parent_of(&level3));
    }

    #[test]
    fn test_parent_chain() {
        let path = ElementPath::new("Patient.name.given.extension");

        let parent1 = path.parent().unwrap();
        assert_eq!(parent1.as_str(), "Patient.name.given");

        let parent2 = parent1.parent().unwrap();
        assert_eq!(parent2.as_str(), "Patient.name");

        let parent3 = parent2.parent().unwrap();
        assert_eq!(parent3.as_str(), "Patient");

        assert!(parent3.parent().is_none());
    }

    #[test]
    fn test_is_slice() {
        let slice_path = ElementPath::new("Patient.identifier:MRN");
        let normal_path = ElementPath::new("Patient.identifier");

        assert!(slice_path.is_slice());
        assert!(!normal_path.is_slice());
    }

    #[test]
    fn test_slice_name() {
        let slice_path = ElementPath::new("Patient.identifier:MRN");
        assert_eq!(slice_path.slice_name(), Some("MRN"));

        let normal_path = ElementPath::new("Patient.identifier");
        assert_eq!(normal_path.slice_name(), None);

        let nested_slice = ElementPath::new("Patient.identifier:MRN.system");
        assert_eq!(nested_slice.slice_name(), Some("MRN"));
    }

    #[test]
    fn test_base_path() {
        let slice_path = ElementPath::new("Patient.identifier:MRN");
        let base = slice_path.base_path();
        assert_eq!(base.as_str(), "Patient.identifier");

        let normal_path = ElementPath::new("Patient.identifier");
        let base_normal = normal_path.base_path();
        assert_eq!(base_normal.as_str(), "Patient.identifier");
    }

    #[test]
    fn test_is_reslice() {
        let reslice_path = ElementPath::new("Patient.identifier:MRN:subslice");
        let slice_path = ElementPath::new("Patient.identifier:MRN");
        let normal_path = ElementPath::new("Patient.identifier");

        assert!(reslice_path.is_reslice());
        assert!(!slice_path.is_reslice());
        assert!(!normal_path.is_reslice());
    }

    #[test]
    fn test_parent_slice() {
        let reslice_path = ElementPath::new("Patient.identifier:MRN:subslice");
        let parent = reslice_path.parent_slice().unwrap();
        assert_eq!(parent.as_str(), "Patient.identifier:MRN");

        let slice_path = ElementPath::new("Patient.identifier:MRN");
        assert!(slice_path.parent_slice().is_none());
    }

    #[test]
    fn test_slice_with_children() {
        let slice_child = ElementPath::new("Patient.identifier:MRN.system");
        assert!(slice_child.is_slice());
        assert_eq!(slice_child.slice_name(), Some("MRN"));

        let base = slice_child.base_path();
        assert_eq!(base.as_str(), "Patient.identifier.system");
    }

    #[test]
    fn test_is_lowercase_start() {
        assert!(ElementPath::is_lowercase_start("abc"));
        assert!(!ElementPath::is_lowercase_start("Abc"));
        assert!(!ElementPath::is_lowercase_start(""));
        assert!(!ElementPath::is_lowercase_start("123"));
        assert!(!ElementPath::is_lowercase_start("ABC"));
    }

    #[test]
    fn test_normalize_choice_type_minimum_length() {
        // "valA" -> len 4. >3. 'v' is lowercase. 'A' is upper. -> "val[x]"
        let path = ElementPath::new("Observation.valA");
        let normalized = path.normalize_choice_type();
        assert_eq!(normalized.as_str(), "Observation.val[x]");

        // "vaA" -> len 3. Not >3. -> "vaA"
        let short_path = ElementPath::new("Observation.vaA");
        let short_normalized = short_path.normalize_choice_type();
        assert_eq!(short_normalized.as_str(), "Observation.vaA");
    }
}
