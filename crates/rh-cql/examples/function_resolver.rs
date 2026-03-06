//! Example demonstrating Function Resolution via the CQL compiler pipeline.
//!
//! Shows how CQL function calls are resolved to ELM by
//! the new three-stage pipeline (SemanticAnalyzer + ElmEmitter).
//! System functions --> native ELM operators; UDFs --> FunctionRef.

use rh_cql::{compile, CompilerOptions};

fn main() {
    println!("=== CQL Function Resolution Demo (new pipeline) ===\n");

    // --- Nullary System Functions ---
    println!("--- Nullary System Functions ---\n");
    let result = compile(
        "library Test version '1.0' define NowResult: Now() define DateResult: Today()",
        Some(CompilerOptions::debug()),
    )
    .unwrap();
    let json = result.to_json().unwrap();
    println!("Now()   -> ELM \"Now\":   {}", json.contains("\"Now\""));
    println!("Today() -> ELM \"Today\": {}", json.contains("\"Today\""));
    println!();

    // --- Aggregate / List Functions ---
    println!("--- Aggregate and List Functions ---\n");
    let result = compile(
        "library Test version '1.0' define S: Sum({1, 2, 3}) define C: Count({1, 2, 3}) define A: Avg({1, 2, 3}) define F: First({1, 2, 3}) define L: Last({1, 2, 3}) define D: distinct {1, 2, 2, 3}",
        Some(CompilerOptions::debug()),
    ).unwrap();
    let json = result.to_json().unwrap();
    println!(
        "Sum()     -> ELM \"Sum\":      {}",
        json.contains("\"Sum\"")
    );
    println!(
        "Count()   -> ELM \"Count\":    {}",
        json.contains("\"Count\"")
    );
    println!(
        "Avg()     -> ELM \"Avg\":      {}",
        json.contains("\"Avg\"")
    );
    println!(
        "First()   -> ELM \"First\":    {}",
        json.contains("\"First\"")
    );
    println!(
        "Last()    -> ELM \"Last\":     {}",
        json.contains("\"Last\"")
    );
    println!(
        "distinct  -> ELM \"Distinct\": {}",
        json.contains("\"Distinct\"")
    );
    println!();

    // --- String Functions ---
    println!("--- String Functions ---\n");
    let result = compile(
        "library Test version '1.0' define L: Length('hello') define U: Upper('hello') define SW: StartsWith('hello world', 'hello')",
        Some(CompilerOptions::debug()),
    ).unwrap();
    let json = result.to_json().unwrap();
    println!(
        "Length()     -> ELM \"Length\":     {}",
        json.contains("\"Length\"")
    );
    println!(
        "Upper()      -> ELM \"Upper\":      {}",
        json.contains("\"Upper\"")
    );
    println!(
        "StartsWith() -> ELM \"StartsWith\": {}",
        json.contains("\"StartsWith\"")
    );
    println!();

    // --- Logic / Null-handling Functions ---
    println!("--- Logic / Null-handling Functions ---\n");
    let result = compile(
        "library Test version '1.0' define C: Coalesce(null, 42) define N: IsNull(null)",
        Some(CompilerOptions::debug()),
    )
    .unwrap();
    let json = result.to_json().unwrap();
    println!(
        "Coalesce() -> ELM \"Coalesce\": {}",
        json.contains("\"Coalesce\"")
    );
    println!(
        "IsNull()   -> ELM \"IsNull\":   {}",
        json.contains("\"IsNull\"")
    );
    println!();

    // --- User-Defined Functions ---
    println!("--- User-Defined Function Invocations ---\n");
    let result = compile(
        "library Test version '1.0' define function Double(x Integer): x * 2 define DoubleResult: Double(21)",
        Some(CompilerOptions::debug()),
    ).unwrap();
    let json = result.to_json().unwrap();
    println!(
        "Double(21) -> FunctionRef: {}",
        json.contains("FunctionRef")
    );
    println!(
        "           -> \"Double\":    {}",
        json.contains("\"Double\"")
    );
    println!();

    // --- Full compilation ---
    println!("--- Full library compilation ---\n");
    let result = compile(
        "library Sample version '1.0' define NowTs: Now() define One: 1 + 2 define function Square(n Integer): n * n define Nine: Square(3)",
        Some(CompilerOptions::default()),
    ).unwrap();
    println!(
        "Success: {}, Errors: {}",
        result.is_success(),
        result.errors.len()
    );
    let json = result.to_json().unwrap();
    let start = json.find("\"identifier\"").unwrap_or(0);
    println!(
        "ELM identifier snippet:\n{}",
        &json[start..(start + 150).min(json.len())]
    );

    println!("\n=== Example Complete ===");
}
