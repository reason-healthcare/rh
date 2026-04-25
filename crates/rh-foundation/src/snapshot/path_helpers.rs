pub(super) fn split_path(path: &str) -> Vec<String> {
    path.split('.').map(ToString::to_string).collect()
}

pub(super) fn parts_are_parent_child(parent: &[String], child: &[String]) -> bool {
    parent.len() < child.len()
        && parent
            .iter()
            .zip(child.iter())
            .all(|(left, right)| left == right)
}

pub(super) fn matches_choice_type_parts(parts: &[String], base_parts: &[String]) -> bool {
    if parts.len() != base_parts.len() || parts.is_empty() {
        return false;
    }

    if !parts[..parts.len() - 1]
        .iter()
        .zip(base_parts[..base_parts.len() - 1].iter())
        .all(|(left, right)| left == right)
    {
        return false;
    }

    let Some(base_last) = base_parts.last() else {
        return false;
    };
    let Some(last) = parts.last() else {
        return false;
    };

    if let Some(prefix) = base_last.strip_suffix("[x]") {
        last.starts_with(prefix)
    } else {
        false
    }
}

pub(super) fn starts_with_lowercase(value: &str) -> bool {
    value.chars().next().is_some_and(|c| c.is_lowercase())
}

pub(super) fn normalized_choice_segment(segment: &str) -> Option<String> {
    if segment.len() <= 3 || !starts_with_lowercase(segment) {
        return None;
    }

    for (index, character) in segment.char_indices() {
        if character.is_uppercase() {
            return Some(format!("{}[x]", &segment[..index]));
        }
    }

    None
}

pub(super) fn find_slice_name(parts: &[String]) -> Option<&str> {
    parts
        .iter()
        .find_map(|part| part.rsplit_once(':').map(|(_, name)| name))
}

pub(super) fn strip_slice_segments(parts: &[String]) -> Vec<String> {
    parts
        .iter()
        .map(|part| {
            part.rsplit_once(':')
                .map_or_else(|| part.clone(), |(base, _)| base.to_string())
        })
        .collect()
}

pub(super) fn has_reslice(part: &str) -> bool {
    part.matches(':').count() > 1
}

pub(super) fn parent_slice_parts(parts: &[String]) -> Option<Vec<String>> {
    let last_part = parts.last()?;
    let (parent_slice, _) = last_part.rsplit_once(':')?;
    if !has_reslice(last_part) {
        return None;
    }

    let mut parent_parts = parts.to_vec();
    parent_parts.pop();
    parent_parts.push(parent_slice.to_string());
    Some(parent_parts)
}
