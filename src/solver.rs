//! Dependency spec parsing + constraint checking (pacman/alpm semantics).
//!
//! Specs: `name`, `name>=ver`, `name<=ver`, `name=ver`, `name>ver`,
//! `name<ver`, and versioned provides `name=ver`.

/// A parsed dependency spec.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepSpec {
    /// Package name (or soname).
    pub name: String,
    /// Optional constraint operator.
    pub op: Option<ConstraintOp>,
    /// Constraint version.
    pub version: Option<String>,
}

/// Version constraint operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintOp {
    /// `=` exact.
    Eq,
    /// `>=`.
    Gte,
    /// `<=`.
    Lte,
    /// `>`.
    Gt,
    /// `<`.
    Lt,
}

/// Parse a dep spec like `foo>=1.2-3` or `libfoo.so=1`.
pub fn parse_spec(spec: &str) -> DepSpec {
    // Longest ops first.
    for (op_str, op) in [
        (">=", ConstraintOp::Gte),
        ("<=", ConstraintOp::Lte),
        ("=", ConstraintOp::Eq),
        (">", ConstraintOp::Gt),
        ("<", ConstraintOp::Lte),
    ] {
        if let Some(pos) = spec.find(op_str) {
            // For `>`, ensure it's not `>=` (already matched) — fine since
            // we check >= first.
            return DepSpec {
                name: spec[..pos].to_string(),
                op: Some(op),
                version: Some(spec[pos + op_str.len()..].to_string()),
            };
        }
    }
    DepSpec { name: spec.to_string(), op: None, version: None }
}

/// alpm-style version comparison: returns Ordering of a vs b.
///
/// Handles `epoch:pkgver-pkgrel`; compares numerics numerically,
/// strings lexically (rpmvercmp-lite).
pub fn vercmp(a: &str, b: &str) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    fn parse(v: &str) -> (u64, &str, &str) {
        let (epoch, rest) = match v.split_once(':') {
            Some((e, r)) => (e.parse().unwrap_or(0), r),
            None => (0, v),
        };
        let (ver, rel) = match rest.rsplit_once('-') {
            Some((v, r)) => (v, r),
            None => (rest, ""),
        };
        (epoch, ver, rel)
    }
    let (ea, va, ra) = parse(a);
    let (eb, vb, rb) = parse(b);
    if ea != eb {
        return ea.cmp(&eb);
    }
    let c = cmp_segments(va, vb);
    if c != Ordering::Equal {
        return c;
    }
    cmp_segments(ra, rb)
}

fn cmp_segments(a: &str, b: &str) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    let mut ai = a.chars().peekable();
    let mut bi = b.chars().peekable();
    loop {
        match (ai.peek().copied(), bi.peek().copied()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(x), Some(y)) => {
                if x.is_ascii_digit() && y.is_ascii_digit() {
                    let mut na = String::new();
                    while let Some(&c) = ai.peek() {
                        if c.is_ascii_digit() { na.push(c); ai.next(); } else { break; }
                    }
                    let mut nb = String::new();
                    while let Some(&c) = bi.peek() {
                        if c.is_ascii_digit() { nb.push(c); bi.next(); } else { break; }
                    }
                    let va: u64 = na.parse().unwrap_or(0);
                    let vb: u64 = nb.parse().unwrap_or(0);
                    if va != vb {
                        return va.cmp(&vb);
                    }
                } else {
                    if x != y {
                        return x.cmp(&y);
                    }
                    ai.next();
                    bi.next();
                }
            }
        }
    }
}

/// Check whether `candidate_version` satisfies the spec's constraint.
pub fn satisfies(spec: &DepSpec, candidate_version: &str) -> bool {
    let (Some(op), Some(want)) = (spec.op, spec.version.as_ref()) else {
        return true; // unconstrained
    };
    let ord = vercmp(candidate_version, want);
    match op {
        ConstraintOp::Eq => ord == std::cmp::Ordering::Equal,
        ConstraintOp::Gte => ord != std::cmp::Ordering::Less,
        ConstraintOp::Lte => ord != std::cmp::Ordering::Greater,
        ConstraintOp::Gt => ord == std::cmp::Ordering::Greater,
        ConstraintOp::Lt => ord == std::cmp::Ordering::Less,
    }
}

/// Resolve a dep name against candidates (installed or repo packages):
/// tries name match first, then provides entries.
pub fn resolve_name<'a, T>(
    spec: &DepSpec,
    candidates: &'a [T],
    name_of: impl Fn(&T) -> &str,
    provides_of: impl Fn(&T) -> &[String],
    version_of: impl Fn(&T) -> &str,
) -> Option<&'a T> {
    // 1. Direct name match + version constraint.
    for c in candidates {
        if name_of(c) == spec.name && satisfies(spec, version_of(c)) {
            return Some(c);
        }
    }
    // 2. Provides match.
    for c in candidates {
        for prov in provides_of(c) {
            let ps = parse_spec(prov);
            if ps.name == spec.name {
                // If the provide carries a version, prefer it; else accept.
                if let Some(pv) = &ps.version {
                    if satisfies(spec, pv) {
                        return Some(c);
                    }
                } else if satisfies(spec, version_of(c)) {
                    return Some(c);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let s = parse_spec("foo>=1.2-3");
        assert_eq!(s.name, "foo");
        assert_eq!(s.op, Some(ConstraintOp::Gte));
        assert_eq!(s.version.as_deref(), Some("1.2-3"));
        let s = parse_spec("bar");
        assert_eq!(s.name, "bar");
        assert!(s.op.is_none());
    }

    #[test]
    fn test_vercmp() {
        assert_eq!(vercmp("1.0", "1.0"), std::cmp::Ordering::Equal);
        assert_eq!(vercmp("1.10", "1.9"), std::cmp::Ordering::Greater);
        assert_eq!(vercmp("2:1.0-1", "1:9.9-9"), std::cmp::Ordering::Greater); // epoch wins
        assert_eq!(vercmp("1.0-2", "1.0-10"), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_satisfies() {
        let s = parse_spec("foo>=1.0");
        assert!(satisfies(&s, "1.0"));
        assert!(satisfies(&s, "2.0"));
        assert!(!satisfies(&s, "0.9"));
        let s = parse_spec("foo=1.0-1");
        assert!(satisfies(&s, "1.0-1"));
        assert!(!satisfies(&s, "1.0-2"));
    }

    #[test]
    fn test_resolve_provides() {
        let pkgs = vec![
            ("real-a".to_string(), "1.0".to_string(), vec!["libfoo.so=1".to_string()]),
            ("real-b".to_string(), "1.0".to_string(), vec![]),
        ];
        let spec = parse_spec("libfoo.so");
        let hit = resolve_name(
            &spec,
            &pkgs,
            |p| p.0.as_str(),
            |p| p.2.as_slice(),
            |p| p.1.as_str(),
        );
        assert_eq!(hit.unwrap().0, "real-a");
    }
}
