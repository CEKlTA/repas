# Repas

A powerful command-line utility for managing dotfiles through symbolic links. Repas helps you centralize your configuration files in one location while maintaining their functionality through symbolic links.

## Overview

Repas is designed to solve the common problem of managing dotfiles (configuration files) across different machines. It works by:
1. Moving your configuration files to a central location (by default `~/.repas`)
2. Creating symbolic links in their original locations
3. Providing functionality to restore files to their original locations when needed

## Installation

### Prerequisites

- Rust 1.87.0 or higher
- Unix-like operating system (Linux, macOS) due to symbolic link functionality

### Building from Source

```bash
git clone https://your-repository/repas.git
cd repas
cargo build --release
```

The compiled binary will be available at `target/release/repas`

## Usage

Repas supports two main operations: `track` and . `restore`

### Track Files

The `track` operation moves files to the central storage location and creates symbolic links in their original locations.

``` bash
repas track /path/to/file1 /path/to/file2
```

Example:

``` bash
repas track ~/.vimrc ~/.bashrc

```

This will:
1. Create the directory if it doesn't exist `~/.repas`
2. Move the files to `~/.repas`
3. Create symbolic links in their original locations

### Restore Files

The operation moves files back to their original locations, removing the symbolic links. `restore`

``` bash
repas restore /path/to/symlink1 /path/to/symlink2
```

Example:

``` bash
repas restore ~/.vimrc
```

This will:
1. Remove the symbolic link
2. Move the actual file back to its original location

## Configuration

### Environment Variables

| Variable   | Description                   | Default    |
|------------|-------------------------------|------------|
| `APP_PATH` | Custom path for storing files | `~/.repas` |

Example of using a custom storage location:

``` bash
export APP_PATH="~/my-dotfiles"
repas track ~/.config/myapp
```

## Features

### Smart Path Handling

- Automatic home directory expansion (`~` gets expanded to the user's home directory)
- Symlink detection to prevent recursive linking
- Full path support for both source and destination

### Colored Output

The utility provides clear, color-coded output for different operations:

- Yellow: Warnings and directory creation
- Cyan: Tracking operations
- Magenta: Symlink removal
- Green: Restore operations
- Red: Error messages

### Error Handling

Comprehensive error handling for common scenarios:

- Existing symlinks
- Missing files
- Invalid paths
- Permission issues

## Technical Details

### Project Structure

``` 
repas/
├── src/
│   ├── main.rs         # CLI interface and main program logic
│   └── functions/      # Core functionality
│       └── mod.rs      # File operations implementation
```

### Dependencies

- `clap` (4.5.39): Command-line argument parsing
- `colored` (3.0.0): Terminal output coloring

## Error Messages

When an operation fails, Repas provides detailed error messages:

``` 
Failed to track path:
  /path/to/file
  <specific error message>
```

``` 
Failed to restore path:
  /path/to/symlink
  <specific error message>
```

## Safety Features

- Checks if a file is already a symlink before tracking
- Verifies symlink targets exist before restoration
- Creates a storage directory automatically when needed
- Provides clear warnings for potential issues

## Contributing

Contributions are welcome! Here are some ways you can help:
- Adding new features
- Improving documentation
- Reporting bugs
- Suggesting enhancements

## License
- MIT

## Future Enhancements

Potential features for future versions:
- Batch operations support
- Configuration file support
- Backup functionality
- Cross-platform support
- Status command to show tracked files
- Dry-run mode