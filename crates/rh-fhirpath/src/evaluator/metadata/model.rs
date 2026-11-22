//! Logical FHIR navigation metadata derived from core StructureDefinition snapshots.
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FhirModel {
    #[default]
    R4,
    R5,
}

struct Type {
    parent: &'static str,
    fields: HashMap<&'static str, &'static str>,
}

type Model = HashMap<&'static str, Type>;

fn load(table: &'static str) -> Model {
    table
        .lines()
        .map(|line| {
            let mut cells = line.split('\t');
            let name = cells.next().unwrap();
            let parent = cells.next().unwrap();
            let fields = cells.map(|cell| cell.split_once('=').unwrap()).collect();
            (name, Type { parent, fields })
        })
        .collect()
}

static R4: LazyLock<Model> = LazyLock::new(|| load(include_str!("r4.tsv")));
static R5: LazyLock<Model> = LazyLock::new(|| load(include_str!("r5.tsv")));

impl FhirModel {
    fn types(self) -> &'static Model {
        match self {
            Self::R4 => &R4,
            Self::R5 => &R5,
        }
    }

    pub fn is_type(self, name: &str) -> bool {
        self.types().contains_key(name)
    }

    pub fn base(self, name: &str) -> Option<&'static str> {
        self.types()
            .get(name)
            .map(|entry| entry.parent)
            .filter(|p| !p.is_empty())
    }

    pub fn is_subtype(self, mut name: &str, parent: &str) -> bool {
        loop {
            if name == parent {
                return true;
            }
            match self.base(name) {
                Some(base) => name = base,
                None => return false,
            }
        }
    }

    pub fn field_type(self, owner: &str, key: &str) -> Option<&'static str> {
        self.field(owner, key).map(|(_, ty)| ty)
    }

    pub fn field_is_repeating(self, owner: &str, key: &str) -> bool {
        self.lookup(owner, key).is_some_and(|(_, _, many)| many)
    }

    pub(crate) fn field(self, owner: &str, key: &str) -> Option<(&'static str, &'static str)> {
        self.lookup(owner, key).map(|(name, ty, _)| (name, ty))
    }

    /// Return the logical name and declared type for a physical JSON property.
    fn lookup(self, mut owner: &str, key: &str) -> Option<(&'static str, &'static str, bool)> {
        while let Some(ty) = self.types().get(owner) {
            if let Some((name, code)) = ty.fields.get_key_value(key) {
                return Some((name, code.trim_start_matches('*'), code.starts_with('*')));
            }
            for (name, codes) in &ty.fields {
                let Some(base) = name.strip_suffix("[x]") else {
                    continue;
                };
                let Some(suffix) = key.strip_prefix(base) else {
                    continue;
                };
                if !suffix.starts_with(|c: char| c.is_ascii_uppercase()) {
                    continue;
                }
                if let Some(code) = codes
                    .trim_start_matches('*')
                    .split(',')
                    .find(|code| code.eq_ignore_ascii_case(suffix))
                {
                    return Some((base, code, codes.starts_with('*')));
                }
            }
            owner = ty.parent;
        }
        None
    }
}

impl FhirModel {
    /// Resolve a logical choice and its concrete FHIR type to the serialization key.
    pub fn choice_key(self, owner: &str, logical: &str, declared: &str) -> Option<String> {
        let mut chars = declared.chars();
        let candidate = format!(
            "{}{}{}",
            logical,
            chars.next()?.to_ascii_uppercase(),
            chars.as_str()
        );
        self.field(owner, &candidate)
            .filter(|(name, ty)| *name == logical && *ty == declared)
            .map(|_| candidate)
    }
}
