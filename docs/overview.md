  1. /docs/rh-fsh-planning.md - Master Planning Document

  - Executive summary with FSH adoption statistics
  - SUSHI architecture reference with module breakdown
  - Complete FSH language overview (11 entity types, 16 rule types)
  - Proposed rh-fsh crate architecture
  - Integration plan with existing rh crates
  - Implementation phases (5 phases from parser to testing)

  2. /docs/rh-fsh-parser-strategy.md - Parser Strategy Analysis

  - Detailed analysis of SUSHI's ANTLR4 grammar (FSH.g4, FSHLexer.g4)
  - Evaluation of 4 parser options:
    - antlr4rust - Direct port, slower but spec-compliant
    - pest - Recommended, good balance of effort/control
    - LALRPOP - Typed AST but needs custom lexer
    - Hand-written - Maximum control, highest effort
  - Implementation roadmap for pest approach
  - Grammar examples and migration considerations

  3. /docs/task-fsh-spec-extraction.md - Specification Extraction Script Task

  - Task for writing a script to scrape and transform the FSH specification
  - Script requirements (sectioning, example extraction, cleanup)
  - Detailed output format (tokens, entities, rules, paths, values)
  - Cross-referencing approach with ANTLR grammar
  - Quality criteria and validation strategy

  4. /docs/rh-fsh-regression-testing.md - Testing Strategy

  - Complete documentation of SUSHI's regression approach
  - FSH Finder integration for test repository discovery
  - Four-level testing strategy:
    a. Unit tests for components
    b. SUSHI test fixture ports
    c. Full regression testing
    d. CI integration
  - Repository selection heuristics
  - Comparison criteria and acceptable differences

  5. /docs/rh-fsh-planning-todo.md - Open Planning Items

  - Tracks areas requiring further research and planning
  - Prioritized list of topics not yet fully addressed

  Key Findings

  Parser Recommendation: Use pest for the initial implementation. It offers:
  - Native Rust integration (no Java toolchain)
  - Readable grammar syntax
  - Good error messages
  - Migration path to LALRPOP or hand-written if needed

  Testing Strategy: The SUSHI regression testing approach using FSH Finder is critical for ensuring compatibility. With 1000+ FSH repositories indexed, this provides
  comprehensive real-world validation.

  ANTLR Grammar Notes: The FSH grammar has several complexities:
  - Mode switching in the lexer for context-sensitive tokens
  - The STAR token requires newline before asterisk
  - SEQUENCE is a permissive catch-all that makes keyword handling tricky

  Next Steps

  The documents are structured to support these subsequent tasks:
  1. Execute the spec extraction task (task-fsh-spec-extraction.md)
  2. Create a proof-of-concept pest grammar
  3. Implement core AST types based on SUSHI's fshtypes
  4. Set up regression testing infrastructure
