# Invalid Or Ambiguous CQL Corpus Cases

This register records external or reduced CQL cases that should not be counted
as `rh-cql` remediation targets until they are reclassified. Prefer adding a
small reduced case with the Java translator diagnostic instead of relying on a
large external source file.

## Omitted-Colon Define Body

Status: source-invalid for remediation purposes.

Spec disposition:

- Treat this as outside the accepted remediation grammar because expression
  definitions require the expression body to follow `:`, and the Java
  translator rejects the reduced source without producing ELM.

Reduced CQL: `conformance/corpus/invalid-or-ambiguous/OmittedColon.cql`

```cql
library OmittedColon version '0.1.0'

define Answer
  42
```

Java translator status:

- CLI process exit: `0`
- Translation result: compile error
- Diagnostic: `Syntax error at 42`
- ELM output: none

`rh-cql` status:

- Parse error, because expression definitions require `:`.

Disposition:

- Do not implement omitted-colon define bodies as a Java-passing parser target.
- Keep regular `define Name: expression` and access-modified forms as the
  remediation target for expression definitions.
- Keep this case quarantined in corpus audit output; it is a source-validity
  fixture, not a remediation target.
