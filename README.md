# Repas

A blazingly fast CLI tool for managing files and folders with symbolic links to centralize configuration across multiple machines. (or any other type of file if you feel like it)

> **PLEASE, for god's sake DO NOT run this with `sudo`**  
> If you know what you're doing and have permission problems, use:
> 
> ```bash
> sudo -E repas ...
> ```
> 
> Otherwise, it will use `/root` as the intended path — which may seriously mess things up.

## Overview

I've got tired of searching for config files all over my big-ass linux. So, I made this to move configuration files to a central location (`~/.repas` by default). Also, you can restore the files if you ever need to.

## Installation

The compiled binary will be available at `target/release/repas`

### Building from Source

### Prerequisites

- Rust ^1.87.0
- Unix-like operating system (Linux, macOS) due to symbolic link functionality

```bash
git clone https://your-repository/repas.git
cd repas
cargo build --release
```

## Usage

Currently, there are two main operations supported: `track` and `restore`.

### Tracking Files

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
2. Move each file to `~/.repas`
3. Create symbolic links in their original locations

### Restoring Files

The `restore` operation moves files back to their original locations, removing the symbolic links

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

| Variable             | Description                   | Default    |
|----------------------|-------------------------------|------------|
| `REPAS_TRACK_FOLDER` | Custom path for storing files | `~/.repas` |

## Features

### Smart Path Handling

- Automatic home directory expansion (`~` gets expanded to the user's home directory environmental variable)
- Symlink detection to prevent recursive linking

### Colored Output

Yes, There are colors, I'm not a monster:

- Yellow: Warnings and directory creation
- Cyan: Tracking operations
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
│   ├── main.rs         # CLI interface and main logic
│   └── functions/      # Core functionality
│       └── mod.rs      # File operations implementation
```

### Dependencies

- `clap` (4.5.39): Command-line argument parsing
- `colored` (3.0.0): Terminal output coloring

## Error Messages

When an operation fails, don't blame the program, you get detailed error messages:

## Safety Features

- Checks if a file is already a symlink before tracking
- Verifies symlink targets exist before restoration
- Creates a storage directory automatically when needed
- Provides clear warnings for potential issues

## Contributing

Contributions are welcome! (and wanted); Here are some ways you can help:

- Adding new features
- Improving documentation
- Reporting bugs (if you can find one)
- Suggesting enhancements

## License

Let's fucking go open-source (MIT)

## Future Enhancements

Potential features for future versions:

- Batch operations support (like reading from a file)
- Backup functionality
- Cross-platform support (are you on Windows??? switch to linux rn)
- Status command to show tracked files