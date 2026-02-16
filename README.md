# fnorm

A Rust-based command-line bulk renaming tool designed to normalize file names across directories.

Supports recursive walking, dynamic rename templates, image dimension extraction, configuration via `.fnorm.toml`, dry-run safety, and collision handling.

## Usage

```bash
fnorm.exe [OPTIONS] [PATH]
```

- **`[PATH]`**: Root directory to process. Defaults to the current directory (`.`).

### Options

| Option            | Description                                  | Default            |
| ----------------- | -------------------------------------------- | ------------------ |
| `--config <PATH>` | Use a specific config file for this run      | None               |
| `--global`        | Use global indexing instead of per-directory | `false`            |
| `--no-dry-run`    | Actually perform renames                     | Enabled by default |
| `-h, --help`      | Show help                                    | -                  |

## Behavior

### Recursive Walking

- Uses `walkdir` to traverse all subdirectories.
- Processes only files (skips directories).
- Processing order is filesystem-dependent.

### Indexing

By default, the `{N}` index resets for every subdirectory. If you run with the `--global` flag, `{N}` increments globally across the entire tree. CLI flags always override settings found inside `.fnorm.toml`.

## Template Engine

Everything outside of curly braces `{}` is treated as a literal string.

### Supported Placeholders

| Placeholder | Description                           |
| ----------- | ------------------------------------- |
| `{parent}`  | Parent folder name (lowercase)        |
| `{PARENT}`  | Parent folder name (uppercase)        |
| `{N}`       | Incrementing index                    |
| `{ext}`     | Original file extension (without dot) |
| `{width}`   | Image width (empty for non-images)    |
| `{height}`  | Image height (empty for non-images)   |

### Template Examples

1. **Standard:** `{parent}-{N}.{ext}`
   _Output:_ `folder-1.txt`, `folder-2.txt`
2. **Prefix:** `{N}-preview.{ext}`
   _Output:_ `1-preview.png`, `2-preview.png`
3. **Image Aware:** `{parent}-{N}_{width}x{height}.{ext}`
   _Image:_ `photos-1_1920x1080.jpg`
   _Text file:_ `photos-2.txt` (Separators `_` and `x` are cleaned up automatically).

## Configuration

`fnorm` supports layered configuration with the following priority:

1. Command line flag: `--config <path>`
2. Local `.fnorm.toml` in the execution directory
3. Global `.fnorm.toml` in the executable directory
4. Built-in defaults

### Example `.fnorm.toml`

```toml
default_template = "{parent}-{N}.{ext}"

# Options: "per-directory" | "global"
index_mode = "per-directory"

# Characters removed when dimension placeholders are empty
cleanup_separators = ["_", "-", " "]
```

## Safety & Collisions

### Dry Run (Default)

By default, `fnorm` **does not rename files**. It only prints the planned changes. To execute the renaming, you must explicitly use:

```bash
fnorm --no-dry-run
```

### Collision Handling

`fnorm` will never overwrite existing files. If a name collision occurs (e.g., `file.txt` already exists), it automatically resolves the conflict by appending an incrementor:

- `file.txt`
- `file-1.txt`
- `file-2.txt`

## Example

Given this directory structure:

```bash
test/
├── image1.jpg
├── image2.jpg
├── file.txt
├── folder1/
│   └── testfile.txt
└── folder2/
    └── picture.png
```

With the following template in `.fnorm.toml`:

```toml
default_template = "{parent}-{N}_{width}x{height}.{ext}"
```

Running `fnorm ./test/` results in this **dry-run** output:

```text
./test/image1.jpg           -> ./test/test-1_1920x1080.jpg
./test/image2.jpg           -> ./test/test-2_1280x720.jpg
./test/file.txt             -> ./test/test-3.txt
./test/folder1/testfile.txt -> ./test/folder1/folder1-1.txt
./test/folder2/picture.png  -> ./test/folder2/folder2-1_800x600.png
```

> **Note:** Non-image files automatically drop `{width}x{height}` placeholders and their preceding separators.

## Technical Details

- **Image Detection:** Uses the `image` crate to extract metadata without a full decode, keeping the process fast.
- **Platform Support:** Works on Windows, macOS, and Linux using native path separators and UTF-8 handling.
- **Limitations:** No parallel processing or exclusion filters are currently implemented (planned for future releases).

## Installation

### From Source

```bash
git clone https://github.com/HardBoss07/fnorm.git
cd fnorm
cargo build --release
```

The binary will be located at `target/release/fnorm`.

## License

This project is licensed under the **MIT License**.
