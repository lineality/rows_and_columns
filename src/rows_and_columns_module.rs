// src/rows_and_columns_module.rs

/// Primary module for the rows_and_columns CSV analysis and TUI dashboard system
/// 
/// This module serves as the main entry point for CSV data processing, analysis, and
/// visualization. It manages the binary-relative directory structure for persistent
/// data storage and coordinates all CSV operations through a terminal user interface.
/// 
/// # Core Responsibilities
/// - Initialize and manage the rows_columns_data/ directory structure
/// - Coordinate CSV file imports and directory-based storage
/// - Provide the main application interface following FF-style patterns
/// - Integrate with file selection and TUI dashboard modules
/// 
/// # Directory Structure Created
/// ```
/// rows_columns_data/
/// ├── csv_imports/           # Imported CSV datasets
/// └── analysis_cache/        # Computed statistics cache
/// ```
/// 
/// # Design Philosophy
/// - Binary-executable-relative paths for portable deployment
/// - Persistent directory-based data storage (not temporary)
/// - No pre-loading: on-demand data processing for scalability
/// - Clear error handling with comprehensive user feedback
use std::env;
use std::path::PathBuf;

// Import CSV processing capabilities
use super::csv_processor_module::{
    analyze_csv_file_structure_and_types,
    CsvAnalysisResults,
};

// Import our custom error types for comprehensive error handling
use super::error_types_module::{
    RowsAndColumnsError, 
    RowsAndColumnsResult,
    create_file_system_error,
    create_configuration_error
};

// Import the path management module for binary-relative operations
use super::manage_absolute_executable_directory_relative_paths::{
    make_verify_or_create_executabledirectoryrelative_canonicalized_dir_path,
    get_absolute_path_to_executable_parentdirectory
};
/// Configuration constants for directory structure
/// 
/// These define the standard directory names used throughout the application
/// for organizing CSV data and analysis results.
const ROWS_COLUMNS_ROOT_DIRECTORY_NAME: &str = "rows_columns_data";
const CSV_IMPORTS_SUBDIRECTORY_NAME: &str = "csv_imports";
const ANALYSIS_CACHE_SUBDIRECTORY_NAME: &str = "analysis_cache";

/// Primary application entry point for rows_and_columns CSV analysis system
/// 
/// This function handles command line arguments and initializes the application environment.
/// It supports direct CSV file processing via command line arguments, similar to the 'lines' pattern.
/// 
/// # Command Line Usage
/// * `rows_and_columns` - Interactive mode (future implementation)
/// * `rows_and_columns <csv_file_path>` - Process specific CSV file
/// * `rows_and_columns --help` - Show usage information
/// 
/// # Returns
/// * `RowsAndColumnsResult<()>` - Success or detailed error information
/// 
/// # Errors
/// * `RowsAndColumnsError::FileSystemError` - If directory creation or file access fails
/// * `RowsAndColumnsError::ConfigurationError` - If setup validation fails
/// 
/// # Examples
/// ```bash
/// # Process a specific CSV file
/// rows_and_columns data/customers.csv
/// 
/// # Show help
/// rows_and_columns --help
/// ```
pub fn run_rows_and_columns_application() -> RowsAndColumnsResult<()> {
    // Parse command line arguments
    let command_line_arguments: Vec<String> = env::args().collect();
    
    // Step 1: Display startup information to user
    display_application_startup_banner();
    
    // Step 2: Initialize and verify directory structure
    let directory_paths = initialize_application_directory_structure()?;
    
    // Step 3: Validate directory setup was successful
    validate_directory_structure_initialization(&directory_paths)?;
    
    // Step 4: Display success information to user
    display_directory_setup_success(&directory_paths);
    
    // Step 5: Process command line arguments
    if command_line_arguments.len() > 1 {
        match command_line_arguments[1].as_str() {
            "--help" | "-h" | "help" => {
                display_usage_help_information();
                return Ok(());
            }
            _ => {
                // Treat the first argument as a CSV file path
                let csv_file_path = &command_line_arguments[1];
                return process_csv_file_from_command_line(csv_file_path, &directory_paths);
            }
        }
    } else {
        // No command line arguments - show available options
        display_interactive_mode_coming_soon();
    }
    
    Ok(())
}

/// Displays usage help information for command line interface
/// 
/// This function shows users how to use the rows_and_columns application
/// with various command line options and file processing modes.
fn display_usage_help_information() {
    println!("USAGE:");
    println!("  rows_and_columns <csv_file_path>     Process a specific CSV file");
    println!("  rows_and_columns --help              Show this help information");
    println!();
    println!("EXAMPLES:");
    println!("  rows_and_columns data/customers.csv");
    println!("  rows_and_columns /home/user/sales_data.csv");
    println!("  rows_and_columns ../reports/quarterly.csv");
    println!();
    println!("FEATURES:");
    println!("  • Directory-based CSV data storage for scalability");
    println!("  • Pandas-style statistical analysis");
    println!("  • ASCII/Unicode TUI charts and visualizations");
    println!("  • Binary-relative path management for portability");
    println!();
}

/// Displays information about interactive mode (placeholder for future implementation)
/// 
/// This function informs users that interactive mode is coming in a future version
/// and shows them how to use the current command line interface.
fn display_interactive_mode_coming_soon() {
    println!("Interactive CSV file selection mode coming in next phase.");
    println!();
    println!("For now, please specify a CSV file directly:");
    println!("  rows_and_columns <path_to_csv_file>");
    println!();
    println!("Example:");
    println!("  rows_and_columns data/my_data.csv");
    println!();
    println!("For help:");
    println!("  rows_and_columns --help");
    println!();
}

// /// Processes a CSV file specified via command line argument
// /// 
// /// This function validates the provided CSV file path, converts it to an absolute path,
// /// and prepares it for processing. This follows the 'lines' pattern of direct file processing.
// /// 
// /// # Arguments
// /// * `csv_file_path_argument` - The CSV file path provided as command line argument
// /// * `directory_paths` - The application directory structure for data storage
// /// 
// /// # Returns
// /// * `RowsAndColumnsResult<()>` - Success or detailed error information
// /// 
// /// # Errors
// /// * `RowsAndColumnsError::FileSystemError` - If file access or validation fails
// /// * `RowsAndColumnsError::ConfigurationError` - If file is not a valid CSV
// fn process_csv_file_from_command_line(
//     csv_file_path_argument: &str,
//     directory_paths: &ApplicationDirectoryPaths,
// ) -> RowsAndColumnsResult<()> {
//     println!("Processing CSV file: {}", csv_file_path_argument);
//     println!();
    
//     // Step 1: Validate the provided file path
//     let csv_file_absolute_path = validate_csv_file_path_from_argument(csv_file_path_argument)?;
    
//     // Step 2: Display file information
//     display_csv_file_processing_information(&csv_file_absolute_path)?;
    
//     // Step 3: TODO - In next implementation phase, this will:
//     // - Parse CSV headers and create metadata
//     // - Create column directory structure
//     // - Process CSV data into directory-based storage
//     // - Generate statistical analysis
//     // - Show TUI dashboard options
    
//     println!("✓ CSV file validated and ready for processing");
//     println!("  File: {}", csv_file_absolute_path.display());
//     println!();
//     println!("Next implementation phase will include:");
//     println!("  • CSV parsing and column detection");
//     println!("  • Directory-based data storage creation");
//     println!("  • Statistical analysis (pandas-style)");
//     println!("  • TUI dashboard generation");
//     println!();
    
//     Ok(())
// }

/// Processes a CSV file specified via command line argument
/// 
/// This function validates the provided CSV file path, analyzes its structure and
/// column types, creates/updates metadata files, and prepares for directory-based storage.
/// 
/// # Arguments
/// * `csv_file_path_argument` - The CSV file path provided as command line argument
/// * `directory_paths` - The application directory structure for data storage
/// 
/// # Returns
/// * `RowsAndColumnsResult<()>` - Success or detailed error information
/// 
/// # Errors
/// * `RowsAndColumnsError::FileSystemError` - If file access or validation fails
/// * `RowsAndColumnsError::CsvProcessingError` - If CSV parsing fails
/// * `RowsAndColumnsError::MetadataError` - If metadata operations fail
fn process_csv_file_from_command_line(
    csv_file_path_argument: &str,
    directory_paths: &ApplicationDirectoryPaths,
) -> RowsAndColumnsResult<()> {
    println!("Processing CSV file: {}", csv_file_path_argument);
    println!();
    
    // Step 1: Validate the provided file path
    let csv_file_absolute_path = validate_csv_file_path_from_argument(csv_file_path_argument)?;
    
    // Step 2: Display basic file information
    display_csv_file_processing_information(&csv_file_absolute_path)?;
    
    // Step 3: Analyze CSV structure and column types
    let csv_analysis_results = analyze_csv_file_structure_and_types(&csv_file_absolute_path)?;
    
    // Step 4: Display detailed analysis results
    display_csv_analysis_results(&csv_analysis_results)?;
    
    // Step 5: Show next steps for user
    display_csv_processing_completion_status(&csv_analysis_results, directory_paths);
    
    Ok(())
}

/// Displays comprehensive CSV analysis results to the user
/// 
/// This function shows detailed information about the CSV structure, column types,
/// and metadata file status after analysis is complete.
/// 
/// # Arguments
/// * `analysis_results` - The complete CSV analysis results
/// 
/// # Returns
/// * `RowsAndColumnsResult<()>` - Success or error if display fails
fn display_csv_analysis_results(analysis_results: &CsvAnalysisResults) -> RowsAndColumnsResult<()> {
    println!("═══════════════════════════════════════════════════════════════");
    println!("  CSV Analysis Results");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    
    // Display file structure summary
    println!("File Structure:");
    println!("  Total Columns: {}", analysis_results.total_column_count);
    println!("  Data Rows: {}", analysis_results.total_data_row_count);
    println!("  Has Header Row: {}", analysis_results.has_header_row);
    println!();
    
    // Display column information
    println!("Column Analysis:");
    for (display_index, column_info) in analysis_results.column_information_list.iter().enumerate() {
        let display_number = display_index + 1;
        
        println!("  {}. {} ({})", 
            display_number,
            column_info.column_name,
            column_info.detected_data_type.to_toml_string()
        );
        
        println!("     Values: {} non-empty, {} empty",
            column_info.non_empty_value_count,
            column_info.empty_value_count
        );
        
        if !column_info.sample_values.is_empty() {
            let sample_display = if column_info.sample_values.len() <= 3 {
                column_info.sample_values.join(", ")
            } else {
                format!("{}, {} ... (showing 3 of {})",
                    column_info.sample_values[0],
                    column_info.sample_values[1],
                    column_info.sample_values.len()
                )
            };
            println!("     Samples: {}", sample_display);
        }
        
        println!();
    }
    
    // Display metadata file information
    println!("Metadata File:");
    if analysis_results.metadata_file_already_existed {
        println!("  ✓ Updated existing: {}", analysis_results.metadata_file_path.display());
    } else {
        println!("  ✓ Created new: {}", analysis_results.metadata_file_path.display());
    }
    println!();
    
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    
    Ok(())
}

/// Displays completion status and next steps after CSV processing
/// 
/// This function shows what was accomplished and what features will be
/// available in future implementation phases.
/// 
/// # Arguments
/// * `analysis_results` - The CSV analysis results
/// * `directory_paths` - The application directory structure
fn display_csv_processing_completion_status(
    analysis_results: &CsvAnalysisResults,
    directory_paths: &ApplicationDirectoryPaths,
) {
    println!("✓ CSV Processing Complete!");
    println!();
    
    println!("What was accomplished:");
    println!("  • File structure analyzed and validated");
    println!("  • Column data types detected: {} columns", analysis_results.total_column_count);
    println!("  • Metadata TOML file created/updated");
    println!("  • Ready for directory-based storage");
    println!();
    
    println!("Data will be stored in:");
    println!("  {}", directory_paths.csv_imports_directory.display());
    println!();
    
    println!("Next implementation phases will include:");
    println!("  • Directory structure creation for each column");
    println!("  • Row-by-row data import to individual cell files");
    println!("  • Statistical analysis (pandas-style describe())");
    println!("  • TUI dashboard with charts and visualizations");
    println!("  • Interactive data exploration interface");
    println!();
    
    // Show the user how to view their metadata file
    let filename_only = analysis_results.csv_file_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("unknown");
    
    println!("To view the generated metadata:");
    println!("  cat {}", analysis_results.metadata_file_path.display());
    println!();
    println!("To reprocess this file:");
    println!("  rows_and_columns {}", analysis_results.csv_file_path.display());
    println!();
}

/// Validates a CSV file path provided as command line argument
/// 
/// This function checks if the provided path exists, is accessible, and appears
/// to be a CSV file based on its extension and basic validation.
/// 
/// # Arguments
/// * `csv_file_path_argument` - The file path string from command line
/// 
/// # Returns
/// * `RowsAndColumnsResult<PathBuf>` - Absolute path to validated CSV file or error
/// 
/// # Errors
/// * `RowsAndColumnsError::FileSystemError` - If file doesn't exist or isn't accessible
/// * `RowsAndColumnsError::ConfigurationError` - If file doesn't appear to be CSV
fn validate_csv_file_path_from_argument(csv_file_path_argument: &str) -> RowsAndColumnsResult<PathBuf> {
    // Convert to PathBuf for easier manipulation
    let file_path = PathBuf::from(csv_file_path_argument);
    
    // Check if file exists
    if !file_path.exists() {
        return Err(create_file_system_error(
            &format!("CSV file does not exist: {}", csv_file_path_argument),
            std::io::Error::new(std::io::ErrorKind::NotFound, "File not found")
        ));
    }
    
    // Check if it's actually a file (not a directory)
    if !file_path.is_file() {
        return Err(create_configuration_error(
            &format!("Path exists but is not a file: {}", csv_file_path_argument)
        ));
    }
    
    // Check file extension suggests CSV format
    let file_extension = file_path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());
    
    match file_extension.as_deref() {
        Some("csv") | Some("tsv") => {
            // File appears to be CSV format
        }
        Some(other_extension) => {
            println!("Warning: File extension '{}' is not typical for CSV files.", other_extension);
            println!("         Proceeding anyway, but ensure this is a comma-separated values file.");
            println!();
        }
        None => {
            println!("Warning: File has no extension. Ensure this is a comma-separated values file.");
            println!();
        }
    }
    
    // Convert to absolute path for consistent handling
    let absolute_file_path = file_path.canonicalize()
        .map_err(|io_error| {
            create_file_system_error(
                &format!("Failed to resolve absolute path for: {}", csv_file_path_argument),
                io_error
            )
        })?;
    
    Ok(absolute_file_path)
}

/// Displays information about the CSV file being processed
/// 
/// This function shows file details including size, path, and basic accessibility
/// information to give users feedback about what file is being processed.
/// 
/// # Arguments
/// * `csv_file_absolute_path` - The absolute path to the CSV file
/// 
/// # Returns
/// * `RowsAndColumnsResult<()>` - Success or error if file information cannot be retrieved
fn display_csv_file_processing_information(csv_file_absolute_path: &PathBuf) -> RowsAndColumnsResult<()> {
    // Get file metadata for size and other information
    let file_metadata = std::fs::metadata(csv_file_absolute_path)
        .map_err(|io_error| {
            create_file_system_error(
                &format!("Failed to read file metadata for: {}", csv_file_absolute_path.display()),
                io_error
            )
        })?;
    
    let file_size_bytes = file_metadata.len();
    let file_size_human_readable = format_file_size_for_display(file_size_bytes);
    
    // Extract just the filename for display
    let filename_only = csv_file_absolute_path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");
    
    println!("CSV File Information:");
    println!("  Name: {}", filename_only);
    println!("  Path: {}", csv_file_absolute_path.display());
    println!("  Size: {} ({} bytes)", file_size_human_readable, file_size_bytes);
    println!("  Type: CSV/Text file");
    println!();
    
    Ok(())
}

/// Formats file size in human-readable format for display
/// 
/// # Arguments
/// * `size_bytes` - File size in bytes
/// 
/// # Returns
/// * `String` - Human-readable size (e.g., "1.2 MB", "456 KB", "12 B")
fn format_file_size_for_display(size_bytes: u64) -> String {
    const KILOBYTE: u64 = 1_024;
    const MEGABYTE: u64 = KILOBYTE * 1_024;
    const GIGABYTE: u64 = MEGABYTE * 1_024;
    
    if size_bytes >= GIGABYTE {
        format!("{:.1} GB", size_bytes as f64 / GIGABYTE as f64)
    } else if size_bytes >= MEGABYTE {
        format!("{:.1} MB", size_bytes as f64 / MEGABYTE as f64)
    } else if size_bytes >= KILOBYTE {
        format!("{:.1} KB", size_bytes as f64 / KILOBYTE as f64)
    } else {
        format!("{} B", size_bytes)
    }
}

/// Displays application startup banner with version and purpose information
/// 
/// This provides clear user feedback that the application is starting and
/// explains its purpose. Follows the minimalist FF-style interface approach.
fn display_application_startup_banner() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("  rows_and_columns - CSV Analysis & TUI Dashboard System");
    println!("  Version: 1.0.0 | Rust Edition: 2024 | License: MIT");
    println!("═══════════════════════════════════════════════════════════════");
    println!("  • Directory-based CSV data storage for scalability");
    println!("  • Pandas-style statistical analysis");
    println!("  • ASCII/Unicode TUI charts and visualizations");
    println!("  • Binary-relative path management for portability");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
}

/// Structure to hold all important directory paths for the application
/// 
/// This centralizes path management and makes it easy to pass directory
/// information between functions while maintaining type safety.
#[derive(Debug, Clone)]
pub struct ApplicationDirectoryPaths {
    /// Absolute path to the executable's parent directory
    pub executable_parent_directory: PathBuf,
    
    /// Absolute path to the main rows_columns_data directory
    pub rows_columns_root_directory: PathBuf,
    
    /// Absolute path to the csv_imports subdirectory
    pub csv_imports_directory: PathBuf,
    
    /// Absolute path to the analysis_cache subdirectory
    pub analysis_cache_directory: PathBuf,
}

/// Initializes the complete directory structure for the application
/// 
/// This function creates all necessary directories using binary-relative paths
/// and returns the absolute paths for use throughout the application. It ensures
/// the directory structure is ready for CSV data storage and analysis operations.
/// 
/// # Returns
/// * `RowsAndColumnsResult<ApplicationDirectoryPaths>` - All directory paths or error
/// 
/// # Errors
/// * `RowsAndColumnsError::FileSystemError` - If any directory creation fails
/// 
/// # Directory Structure Created
/// ```
/// executable_directory/
/// └── rows_columns_data/
///     ├── csv_imports/
///     └── analysis_cache/
/// ```
fn initialize_application_directory_structure() -> RowsAndColumnsResult<ApplicationDirectoryPaths> {
    // Get the executable's parent directory for reference
    let executable_parent_directory = get_absolute_path_to_executable_parentdirectory()
        .map_err(|io_error| {
            create_file_system_error(
                "Failed to determine executable parent directory",
                io_error
            )
        })?;
    
    // Create the main rows_columns_data directory
    let rows_columns_root_directory = make_verify_or_create_executabledirectoryrelative_canonicalized_dir_path(
        ROWS_COLUMNS_ROOT_DIRECTORY_NAME
    ).map_err(|io_error| {
        create_file_system_error(
            &format!("Failed to create main directory: {}", ROWS_COLUMNS_ROOT_DIRECTORY_NAME),
            io_error
        )
    })?;
    
    // Create the csv_imports subdirectory
    let csv_imports_relative_path = format!(
        "{}/{}",
        ROWS_COLUMNS_ROOT_DIRECTORY_NAME,
        CSV_IMPORTS_SUBDIRECTORY_NAME
    );
    
    let csv_imports_directory = make_verify_or_create_executabledirectoryrelative_canonicalized_dir_path(
        &csv_imports_relative_path
    ).map_err(|io_error| {
        create_file_system_error(
            &format!("Failed to create CSV imports directory: {}", csv_imports_relative_path),
            io_error
        )
    })?;
    
    // Create the analysis_cache subdirectory
    let analysis_cache_relative_path = format!(
        "{}/{}",
        ROWS_COLUMNS_ROOT_DIRECTORY_NAME,
        ANALYSIS_CACHE_SUBDIRECTORY_NAME
    );
    
    let analysis_cache_directory = make_verify_or_create_executabledirectoryrelative_canonicalized_dir_path(
        &analysis_cache_relative_path
    ).map_err(|io_error| {
        create_file_system_error(
            &format!("Failed to create analysis cache directory: {}", analysis_cache_relative_path),
            io_error
        )
    })?;
    
    // Return the complete directory structure information
    Ok(ApplicationDirectoryPaths {
        executable_parent_directory,
        rows_columns_root_directory,
        csv_imports_directory,
        analysis_cache_directory,
    })
}

/// Validates that the directory structure was created correctly
/// 
/// This function performs post-creation validation to ensure all directories
/// exist, are accessible, and have the expected properties. It provides an
/// additional safety check after directory creation.
/// 
/// # Arguments
/// * `directory_paths` - The directory paths to validate
/// 
/// # Returns
/// * `RowsAndColumnsResult<()>` - Success or validation error
/// 
/// # Errors
/// * `RowsAndColumnsError::ConfigurationError` - If validation fails
fn validate_directory_structure_initialization(
    directory_paths: &ApplicationDirectoryPaths
) -> RowsAndColumnsResult<()> {
    // Validate executable parent directory
    if !directory_paths.executable_parent_directory.exists() {
        return Err(create_configuration_error(
            "Executable parent directory does not exist after initialization"
        ));
    }
    
    if !directory_paths.executable_parent_directory.is_dir() {
        return Err(create_configuration_error(
            "Executable parent path exists but is not a directory"
        ));
    }
    
    // Validate main rows_columns_data directory
    if !directory_paths.rows_columns_root_directory.exists() {
        return Err(create_configuration_error(
            "Main rows_columns_data directory does not exist after creation"
        ));
    }
    
    if !directory_paths.rows_columns_root_directory.is_dir() {
        return Err(create_configuration_error(
            "Main rows_columns_data path exists but is not a directory"
        ));
    }
    
    // Validate csv_imports subdirectory
    if !directory_paths.csv_imports_directory.exists() {
        return Err(create_configuration_error(
            "CSV imports directory does not exist after creation"
        ));
    }
    
    if !directory_paths.csv_imports_directory.is_dir() {
        return Err(create_configuration_error(
            "CSV imports path exists but is not a directory"
        ));
    }
    
    // Validate analysis_cache subdirectory
    if !directory_paths.analysis_cache_directory.exists() {
        return Err(create_configuration_error(
            "Analysis cache directory does not exist after creation"
        ));
    }
    
    if !directory_paths.analysis_cache_directory.is_dir() {
        return Err(create_configuration_error(
            "Analysis cache path exists but is not a directory"
        ));
    }
    
    // All validations passed
    Ok(())
}

/// Displays success information about directory setup to the user
/// 
/// This provides clear feedback about what directories were created and where
/// they are located. Helps users understand the application's file organization.
/// 
/// # Arguments
/// * `directory_paths` - The successfully created directory paths
fn display_directory_setup_success(directory_paths: &ApplicationDirectoryPaths) {
    println!("✓ Directory structure initialized successfully:");
    println!();
    
    println!("  Executable Location:");
    println!("    {}", directory_paths.executable_parent_directory.display());
    println!();
    
    println!("  Data Storage Root:");
    println!("    {}", directory_paths.rows_columns_root_directory.display());
    println!();
    
    println!("  CSV Imports Directory:");
    println!("    {}", directory_paths.csv_imports_directory.display());
    println!();
    
    println!("  Analysis Cache Directory:");
    println!("    {}", directory_paths.analysis_cache_directory.display());
    println!();
    
    println!("═══════════════════════════════════════════════════════════════");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    /// Test that the directory structure constants are reasonable
    #[test]
    fn test_directory_constants() {
        // Directory names should not be empty
        assert!(!ROWS_COLUMNS_ROOT_DIRECTORY_NAME.is_empty());
        assert!(!CSV_IMPORTS_SUBDIRECTORY_NAME.is_empty());
        assert!(!ANALYSIS_CACHE_SUBDIRECTORY_NAME.is_empty());
        
        // Directory names should not contain path separators
        assert!(!ROWS_COLUMNS_ROOT_DIRECTORY_NAME.contains('/'));
        assert!(!ROWS_COLUMNS_ROOT_DIRECTORY_NAME.contains('\\'));
        assert!(!CSV_IMPORTS_SUBDIRECTORY_NAME.contains('/'));
        assert!(!CSV_IMPORTS_SUBDIRECTORY_NAME.contains('\\'));
        assert!(!ANALYSIS_CACHE_SUBDIRECTORY_NAME.contains('/'));
        assert!(!ANALYSIS_CACHE_SUBDIRECTORY_NAME.contains('\\'));
    }
    
    /// Test the ApplicationDirectoryPaths structure
    #[test]
    fn test_application_directory_paths_structure() {
        // Create a test instance with dummy paths
        let test_paths = ApplicationDirectoryPaths {
            executable_parent_directory: PathBuf::from("/test/exe"),
            rows_columns_root_directory: PathBuf::from("/test/exe/rows_columns_data"),
            csv_imports_directory: PathBuf::from("/test/exe/rows_columns_data/csv_imports"),
            analysis_cache_directory: PathBuf::from("/test/exe/rows_columns_data/analysis_cache"),
        };
        
        // Verify the structure can be created and accessed
        assert!(test_paths.executable_parent_directory.to_string_lossy().contains("exe"));
        assert!(test_paths.rows_columns_root_directory.to_string_lossy().contains("rows_columns_data"));
        assert!(test_paths.csv_imports_directory.to_string_lossy().contains("csv_imports"));
        assert!(test_paths.analysis_cache_directory.to_string_lossy().contains("analysis_cache"));
        
        // Test that the structure can be cloned
        let cloned_paths = test_paths.clone();
        assert_eq!(test_paths.executable_parent_directory, cloned_paths.executable_parent_directory);
    }
    
    /// Test directory initialization logic (without actually creating directories)
    #[test]
    fn test_directory_path_construction() {
        // Test path construction logic
        let csv_imports_path = format!(
            "{}/{}",
            ROWS_COLUMNS_ROOT_DIRECTORY_NAME,
            CSV_IMPORTS_SUBDIRECTORY_NAME
        );
        
        let analysis_cache_path = format!(
            "{}/{}",
            ROWS_COLUMNS_ROOT_DIRECTORY_NAME,
            ANALYSIS_CACHE_SUBDIRECTORY_NAME
        );
        
        // Verify paths are constructed correctly
        assert_eq!(csv_imports_path, "rows_columns_data/csv_imports");
        assert_eq!(analysis_cache_path, "rows_columns_data/analysis_cache");
    }
}
