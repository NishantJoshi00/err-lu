# Error-Cat

A fast, WebAssembly-compatible error categorization library that helps standardize error messages across different payment processors.

## Description

Error-Cat is a Rust library that provides a unified interface for mapping payment processor-specific error codes to standardized error categories. It offers:

- Fast lookups through optimized data structures
- WebAssembly (WASM) support for browser-based applications
- A simple, easy-to-use API for error code mapping
- Build-time error mapping compilation for optimal runtime performance

## Installation

Add Error-Cat to your Rust project by including it in your `Cargo.toml`:

```toml
[dependencies]
err-cat = "0.1.0"

# Optional: Enable WASM support
err-cat = { version = "0.1.0", features = ["wasm"] }
```

For WASM usage, you'll need to:

1. Install wasm-pack:
```bash
cargo install wasm-pack
```

2. Build the WASM package:
```bash
wasm-pack build --target web
```

3. The generated files will be in the `pkg/` directory, ready for web use.

## Usage

### Rust API

```rust
use err_cat::prelude::*;

// Get mapping for a specific connector
let mapping = get_connector_mapping("cybersource");
if let Some(category) = mapping.get("PROCESSOR_DECLINED") {
    println!("Error category: {}", category); // Outputs: "Issue with Payment Method"
}

// Get list of supported connectors
let connectors = get_connector_list();
```

### WASM/JavaScript API

```javascript
import init, { lookup } from 'err-cat';

await init();

// Look up error category
const category = lookup("cybersource", "PROCESSOR_DECLINED");
console.log(category); // Outputs: "Issue with Payment Method"
```

## Features

- **Native Rust Support**: High-performance error code mapping in pure Rust applications
- **WASM Integration**: Seamless usage in web applications through WebAssembly
- **Build-time Processing**: Error mappings are compiled into the binary for zero runtime overhead
- **Extensible Design**: Easy to add new payment processors and error codes
- **Type Safety**: Leverages Rust's type system to ensure mapping correctness
- **Zero Dependencies** in runtime (only build dependencies for processing mapping data)

## Contributing Guidelines

1. Create an issue for any new feature or bug fix you want to work on
2. Tag your issues appropriately with [BUG], [FEATURE], etc.
3. Don't work on an issue that's already assigned to someone else
4. Ensure your code follows the project's style guidelines
5. Write tests for any new functionality
6. Update documentation when adding or modifying features

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Build Dependencies

- Rust 1.70+
- wasm-bindgen (for WASM support)
- bincode (for binary serialization)
- criterion (for benchmarking)
