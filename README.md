
# File Aggregator (`fileagg`)

Welcome to `fileagg`, the ultimate file aggregation tool! If you've ever needed to consolidate multiple files from a directory into a single file effortlessly, `fileagg` is your go-to solution. With features like respecting `.gitignore`, excluding `node_modules`, and supporting multiple output options, `fileagg` makes file management a breeze.

## Features

- **Aggregate File Contents**: Combine contents from all files in a directory into one file.
- **Respects `.gitignore`**: Automatically skip files listed in `.gitignore`.
- **Excludes `node_modules`**: No more unnecessary bloat in your aggregated output.
- **Flexible Output Options**: Save to a file, print to console, or copy directly to clipboard.
- **Include Hidden Files**: Optionally include hidden files in your aggregation.
- **Filter by File Type**: Only aggregate specific file types you care about.
- **Progress Indication**: See real-time progress as files are processed.

## Installation

### Prerequisites

Ensure you have [Rust](https://www.rust-lang.org/tools/install) and Cargo installed on your system.

### Installing via Cargo

To install `fileagg` as a Cargo package, run the following command:

```bash
cargo install fileagg
```

### Building Locally

If you prefer to build `fileagg` from source:

1. Clone the repository or download the source code.

2. Navigate to the project directory:

   ```bash
   cd path/to/your/fileagg
   ```

3. Build and install locally:

   ```bash
   cargo install --path .
   ```

## Usage

### Default Behavior

By default, `fileagg` aggregates files in the current directory and saves the output to `fileagg_output.txt`.

```bash
fileagg
```

### Commands and Options

#### 1. Specify Directory

Aggregate files from a specific directory:

```bash
fileagg /path/to/directory
```

#### 2. Specify Output File

To customize the output file location and name:

```bash
fileagg --output result.txt
```

This command saves the output to `result.txt`.

#### 3. Print to Standard Output

To print the aggregated contents to the console:

```bash
fileagg --stdout
```

#### 4. Copy to Clipboard

Copy the aggregated contents to your clipboard:

```bash
fileagg --clipboard
```

#### 5. Include Hidden Files

To include hidden files (e.g., `.env`, `.gitignore`):

```bash
fileagg --include-hidden
```

#### 6. Ignore `.gitignore` Rules

If you want to ignore `.gitignore` rules and include all files:

```bash
fileagg --no-ignore
```

#### 7. Filter by File Types

Aggregate only specific file types:

```bash
fileagg --file-types rs,js,py
```

### Example Commands

1. Aggregate all files in the current directory and save to `fileagg_output.txt`:

   ```bash
   fileagg
   ```

2. Aggregate files from a specific directory and save to `custom_output.txt`:

   ```bash
   fileagg /path/to/directory --output custom_output.txt
   ```

3. Include hidden files and print output to console:

   ```bash
   fileagg --include-hidden --stdout
   ```

4. Aggregate specific file types and copy to clipboard:

   ```bash
   fileagg --file-types md,txt --clipboard
   ```

## Notes

- **Node Modules Exclusion**: `fileagg` automatically skips files in the `node_modules` directory.
- **Default Output**: If no output method is specified, results are saved to `fileagg_output.txt`.
- **Safety First**: Ensure the output file isn't in the list of files to aggregate to prevent overwriting.

## Contributing

We welcome contributions! Feel free to submit issues, pull requests, or ideas for improvements.

## Contact

For questions or feedback, reach out to [simplysabir](https://github.com/simplysabir).

