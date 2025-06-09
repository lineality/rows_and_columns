
// src/csv_processor_module.rs

/// CSV file processing and metadata analysis for rows_and_columns
/// 
/// This module handles CSV file parsing, column type detection, and metadata TOML
/// file creation and management. It analyzes CSV structure without loading entire
/// datasets into memory, following the scalable design philosophy.
/// 
/// # Core Responsibilities
/// - Parse CSV headers and detect column structure
/// - Analyze column data types (bool, int, float, string)
/// - Create and manage CSV metadata TOML files
/// - Validate CSV format and accessibility
/// - Generate column analysis reports
/// 
/// # Design Philosophy
/// - Sample-based analysis: analyze first N rows for type detection
/// - Memory-efficient: don't load entire CSV into memory
/// - Metadata-driven: persistent TOML files track column information
/// - Fallback handling: graceful handling of missing headers or mixed types

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

// Import our error handling system
use super::error_types_module::{
    RowsAndColumnsError,
    RowsAndColumnsResult,
    create_file_system_error,
    create_csv_processing_error,
    create_metadata_error,
    create_configuration_error,
};

// Import path management for metadata file operations
use super::manage_absolute_executable_directory_relative_paths::{
    make_input_path_name_abs_executabledirectoryrelative_nocheck,
    abs_executable_directory_relative_exists,
    prepare_file_parent_directories_abs_executabledirectoryrelative,
};

/// Configuration constants for CSV processing
const CSV_SAMPLE_ROWS_FOR_TYPE_DETECTION: usize = 10;
const METADATA_FILE_EXTENSION: &str = "csv_metadata.toml";

/// Represents the detected data type for a CSV column
/// 
/// This enum covers the MVP data types that the system can detect
/// and handle in the directory-based storage system.
#[derive(Debug, Clone, PartialEq)]
pub enum CsvColumnDataType {
    /// Boolean values (true/false, yes/no, 1/0)
    Boolean,
    
    /// Integer values (whole numbers)
    Integer,
    
    /// Floating point values (decimal numbers)
    Float,
    
    /// String/text values (fallback for anything not clearly typed)
    String,
}

impl CsvColumnDataType {
    /// Converts the data type to a string representation for TOML storage
    /// 
    /// # Returns
    /// * `&str` - String representation of the data type
    pub fn to_toml_string(&self) -> &str {
        match self {
            CsvColumnDataType::Boolean => "boolean",
            CsvColumnDataType::Integer => "integer", 
            CsvColumnDataType::Float => "float",
            CsvColumnDataType::String => "string",
        }
    }
    
    /// Creates a data type from a TOML string representation
    /// 
    /// # Arguments
    /// * `toml_string` - The string representation from TOML file
    /// 
    /// # Returns
    /// * `Option<CsvColumnDataType>` - The data type or None if invalid
    pub fn from_toml_string(toml_string: &str) -> Option<CsvColumnDataType> {
        match toml_string.to_lowercase().as_str() {
            "boolean" | "bool" => Some(CsvColumnDataType::Boolean),
            "integer" | "int" => Some(CsvColumnDataType::Integer),
            "float" | "decimal" | "number" => Some(CsvColumnDataType::Float),
            "string" | "text" | "str" => Some(CsvColumnDataType::String),
            _ => None,
        }
    }
}

/// Information about a detected CSV column
/// 
/// This structure holds comprehensive information about each column
/// discovered during CSV analysis.
#[derive(Debug, Clone)]
pub struct CsvColumnInformation {
    /// Index of the column (0-based)
    pub column_index: usize,
    
    /// Name of the column (from header or generated)
    pub column_name: String,
    
    /// Detected data type for this column
    pub detected_data_type: CsvColumnDataType,
    
    /// Number of non-empty values found during analysis
    pub non_empty_value_count: usize,
    
    /// Number of empty/null values found during analysis
    pub empty_value_count: usize,
    
    /// Sample values from this column (for user review)
    pub sample_values: Vec<String>,
}

/// Complete analysis results for a CSV file
/// 
/// This structure contains all information discovered during CSV analysis,
/// including file structure, column details, and metadata file status.
#[derive(Debug)]
pub struct CsvAnalysisResults {
    /// Absolute path to the original CSV file
    pub csv_file_path: PathBuf,
    
    /// Whether the CSV file has a header row
    pub has_header_row: bool,
    
    /// Total number of columns detected
    pub total_column_count: usize,
    
    /// Total number of data rows (excluding header)
    pub total_data_row_count: usize,
    
    /// Information about each column
    pub column_information_list: Vec<CsvColumnInformation>,
    
    /// Path to the metadata TOML file (existing or to-be-created)
    pub metadata_file_path: PathBuf,
    
    /// Whether a metadata file already existed
    pub metadata_file_already_existed: bool,
}

/// Analyzes a CSV file and detects column structure and data types
/// 
/// This function performs comprehensive CSV analysis including header detection,
/// column type analysis, and metadata file management. It creates or updates
/// the corresponding metadata TOML file.
/// 
/// # Arguments
/// * `csv_file_path` - Absolute path to the CSV file to analyze
/// 
/// # Returns
/// * `RowsAndColumnsResult<CsvAnalysisResults>` - Complete analysis results or error
/// 
/// # Errors
/// * `RowsAndColumnsError::FileSystemError` - If file access fails
/// * `RowsAndColumnsError::CsvProcessingError` - If CSV parsing fails
/// * `RowsAndColumnsError::MetadataError` - If metadata file operations fail
pub fn analyze_csv_file_structure_and_types(csv_file_path: &PathBuf) -> RowsAndColumnsResult<CsvAnalysisResults> {
    println!("🔍 Analyzing CSV file structure...");
    
    // Step 1: Read and analyze the CSV file structure
    let (has_header_row, column_count, data_row_count) = analyze_csv_basic_structure(csv_file_path)?;
    
    println!("  ✓ Basic structure detected:");
    println!("    Columns: {}", column_count);
    println!("    Data rows: {}", data_row_count);
    println!("    Has header: {}", has_header_row);
    
    // Step 2: Analyze column data types and content
    let column_information_list = analyze_csv_column_types_and_content(
        csv_file_path, 
        has_header_row, 
        column_count
    )?;
    
    println!("  ✓ Column types analyzed");
    
    // Step 3: Determine metadata file path and check if it exists
    let metadata_file_path = determine_metadata_file_path(csv_file_path)?;
    let metadata_file_already_existed = metadata_file_path.exists();
    
    if metadata_file_already_existed {
        println!("  ✓ Found existing metadata file: {}", metadata_file_path.display());
    } else {
        println!("  ✓ Will create metadata file: {}", metadata_file_path.display());
    }
    
    // Step 4: Create or update metadata file
    create_or_update_metadata_file(&metadata_file_path, &column_information_list)?;
    
    println!("  ✓ Metadata file updated");
    
    // Return complete analysis results
    Ok(CsvAnalysisResults {
        csv_file_path: csv_file_path.clone(),
        has_header_row,
        total_column_count: column_count,
        total_data_row_count: data_row_count,
        column_information_list,
        metadata_file_path,
        metadata_file_already_existed,
    })
}

/// Analyzes basic CSV file structure (row count, column count, header detection)
/// 
/// This function reads through the CSV file to determine fundamental structure
/// without performing detailed type analysis.
/// 
/// # Arguments
/// * `csv_file_path` - Path to the CSV file to analyze
/// 
/// # Returns
/// * `RowsAndColumnsResult<(bool, usize, usize)>` - (has_header, column_count, data_rows)
fn analyze_csv_basic_structure(csv_file_path: &PathBuf) -> RowsAndColumnsResult<(bool, usize, usize)> {
    let csv_file = File::open(csv_file_path)
        .map_err(|io_error| {
            create_file_system_error(
                &format!("Failed to open CSV file for analysis: {}", csv_file_path.display()),
                io_error
            )
        })?;
    
    let csv_reader = BufReader::new(csv_file);
    let mut csv_lines = csv_reader.lines();
    
    // Read the first line to determine column count
    let first_line = csv_lines.next()
        .ok_or_else(|| {
            create_csv_processing_error(
                "CSV file appears to be empty",
                Some(1),
                None
            )
        })?
        .map_err(|io_error| {
            create_file_system_error(
                "Failed to read first line of CSV file",
                io_error
            )
        })?;
    
    let column_count = count_csv_columns_in_line(&first_line);
    
    // Check if first line looks like a header by analyzing the second line
    let has_header_row = detect_csv_header_row(&mut csv_lines, column_count, &first_line)?;
    
    // Count total data rows (excluding header if present)
    let total_rows = count_remaining_csv_lines(csv_lines)?;
    let data_row_count = if has_header_row { total_rows } else { total_rows + 1 };
    
    Ok((has_header_row, column_count, data_row_count))
}

/// Counts the number of CSV columns in a line by splitting on commas
/// 
/// This handles basic CSV comma separation. For MVP, we assume simple comma
/// separation without escaped commas in quoted fields.
/// 
/// # Arguments
/// * `csv_line` - The CSV line to analyze
/// 
/// # Returns
/// * `usize` - Number of columns detected
fn count_csv_columns_in_line(csv_line: &str) -> usize {
    // Simple comma split for MVP - could be enhanced for quoted fields later
    csv_line.split(',').count()
}
/// Detects whether the CSV file has a header row
/// 
/// This function uses heuristics to determine if the first row contains
/// column headers rather than data.
/// 
/// # Arguments
/// * `remaining_lines` - Iterator of remaining lines after the first
/// * `expected_column_count` - Expected number of columns
/// * `first_line` - The first line content for analysis
/// 
/// # Returns
/// * `RowsAndColumnsResult<bool>` - True if header row detected
fn detect_csv_header_row(
    remaining_lines: &mut std::io::Lines<BufReader<File>>,
    expected_column_count: usize,
    first_line: &str,
) -> RowsAndColumnsResult<bool> {
    // Read the second line for comparison
    let second_line = match remaining_lines.next() {
        Some(line_result) => {
            line_result.map_err(|io_error| {
                create_file_system_error("Failed to read second line of CSV", io_error)
            })?
        }
        None => {
            // Only one line in file - assume it's data, not header
            return Ok(false);
        }
    };
    
    // Split both lines into fields
    let first_fields: Vec<&str> = first_line.split(',').collect();
    let second_fields: Vec<&str> = second_line.split(',').collect();
    
    // Check if field count matches expected column count
    if first_fields.len() != expected_column_count || 
       second_fields.len() != expected_column_count {
        // Inconsistent column counts - this is suspicious but proceed
        println!("  Warning: Inconsistent column counts detected");
    }
    
    // Heuristic: if first line contains non-numeric values and second line
    // contains more numeric values, first line is likely a header
    let first_line_numeric_fields = count_numeric_fields(&first_fields);
    let second_line_numeric_fields = count_numeric_fields(&second_fields);
    
    // If first line has fewer numeric fields than second line, it's likely a header
    let likely_header = first_line_numeric_fields < second_line_numeric_fields;
    
    Ok(likely_header)
}

/// Counts how many fields in a list appear to be numeric (int or float)
/// 
/// # Arguments
/// * `fields` - List of field values to analyze
/// 
/// # Returns
/// * `usize` - Number of fields that appear numeric
fn count_numeric_fields(fields: &[&str]) -> usize {
    fields.iter()
        .filter(|field| {
            let trimmed_field = field.trim();
            // Try to parse as integer or float
            trimmed_field.parse::<i64>().is_ok() || 
            trimmed_field.parse::<f64>().is_ok()
        })
        .count()
}

/// Counts remaining lines in the CSV file
/// 
/// # Arguments
/// * `lines_iterator` - Iterator over remaining lines
/// 
/// # Returns
/// * `RowsAndColumnsResult<usize>` - Number of remaining lines or error
fn count_remaining_csv_lines(
    lines_iterator: std::io::Lines<BufReader<File>>
) -> RowsAndColumnsResult<usize> {
    let mut line_count = 0;
    
    for line_result in lines_iterator {
        line_result.map_err(|io_error| {
            create_file_system_error("Failed to read CSV line during counting", io_error)
        })?;
        line_count += 1;
    }
    
    Ok(line_count)
}

/// Analyzes column data types and content by sampling CSV data
/// 
/// This function reads sample rows from the CSV to determine the most likely
/// data type for each column based on the values found.
/// 
/// # Arguments
/// * `csv_file_path` - Path to the CSV file
/// * `has_header_row` - Whether the file has a header row to skip
/// * `column_count` - Expected number of columns
/// 
/// # Returns
/// * `RowsAndColumnsResult<Vec<CsvColumnInformation>>` - Column information list
fn analyze_csv_column_types_and_content(
    csv_file_path: &PathBuf,
    has_header_row: bool,
    column_count: usize,
) -> RowsAndColumnsResult<Vec<CsvColumnInformation>> {
    let csv_file = File::open(csv_file_path)
        .map_err(|io_error| {
            create_file_system_error(
                &format!("Failed to open CSV file for type analysis: {}", csv_file_path.display()),
                io_error
            )
        })?;
    
    let csv_reader = BufReader::new(csv_file);
    let mut csv_lines = csv_reader.lines();
    
    // Initialize column information structures
    let mut column_info_list = Vec::new();
    let mut column_sample_values: Vec<Vec<String>> = vec![Vec::new(); column_count];
    let mut column_non_empty_counts = vec![0usize; column_count];
    let mut column_empty_counts = vec![0usize; column_count];
    
    // Read header row if it exists to get column names
    let column_names = if has_header_row {
        match csv_lines.next() {
            Some(header_line_result) => {
                let header_line = header_line_result.map_err(|io_error| {
                    create_file_system_error("Failed to read header line", io_error)
                })?;
                parse_csv_line_into_fields(&header_line)
            }
            None => {
                return Err(create_csv_processing_error(
                    "CSV file appears empty when trying to read header",
                    Some(1),
                    None
                ));
            }
        }
    } else {
        // Generate column names: column_1, column_2, etc.
        (0..column_count)
            .map(|index| format!("column_{}", index + 1))
            .collect()
    };
    
    // Sample data rows for type detection
    let mut rows_processed = 0;
    for (line_number, line_result) in csv_lines.enumerate() {
        if rows_processed >= CSV_SAMPLE_ROWS_FOR_TYPE_DETECTION {
            break;
        }
        
        let csv_line = line_result.map_err(|io_error| {
            create_file_system_error(
                &format!("Failed to read CSV line {}", line_number + if has_header_row { 2 } else { 1 }),
                io_error
            )
        })?;
        
        let field_values = parse_csv_line_into_fields(&csv_line);
        
        // Process each field in this row
        for (column_index, field_value) in field_values.iter().enumerate() {
            if column_index >= column_count {
                // More fields than expected - skip extras
                continue;
            }
            
            let trimmed_value = field_value.trim();
            
            if trimmed_value.is_empty() {
                column_empty_counts[column_index] += 1;
            } else {
                column_non_empty_counts[column_index] += 1;
                
                // Store sample values (limit to prevent memory issues)
                if column_sample_values[column_index].len() < 5 {
                    column_sample_values[column_index].push(trimmed_value.to_string());
                }
            }
        }
        
        rows_processed += 1;
    }
    
    // Analyze data types for each column based on samples
    for column_index in 0..column_count {
        let column_name = column_names.get(column_index)
            .cloned()
            .unwrap_or_else(|| format!("column_{}", column_index + 1));
        
        let detected_data_type = detect_column_data_type(&column_sample_values[column_index]);
        
        let column_info = CsvColumnInformation {
            column_index,
            column_name,
            detected_data_type,
            non_empty_value_count: column_non_empty_counts[column_index],
            empty_value_count: column_empty_counts[column_index],
            sample_values: column_sample_values[column_index].clone(),
        };
        
        column_info_list.push(column_info);
    }
    
    Ok(column_info_list)
}

/// Parses a CSV line into individual field values
/// 
/// For MVP, this uses simple comma splitting. Could be enhanced later
/// for proper CSV parsing with quoted fields and escaped commas.
/// 
/// # Arguments
/// * `csv_line` - The CSV line to parse
/// 
/// # Returns
/// * `Vec<String>` - List of field values
fn parse_csv_line_into_fields(csv_line: &str) -> Vec<String> {
    csv_line.split(',')
        .map(|field| field.to_string())
        .collect()
}

/// Detects the most likely data type for a column based on sample values
/// 
/// This function analyzes sample values and determines the most appropriate
/// data type using heuristics and type parsing attempts.
/// 
/// # Arguments
/// * `sample_values` - List of sample values from the column
/// 
/// # Returns
/// * `CsvColumnDataType` - The detected data type
fn detect_column_data_type(sample_values: &[String]) -> CsvColumnDataType {
    if sample_values.is_empty() {
        return CsvColumnDataType::String;
    }
    
    let mut boolean_count = 0;
    let mut integer_count = 0;
    let mut float_count = 0;
    let total_samples = sample_values.len();
    
    for sample_value in sample_values {
        let trimmed_value = sample_value.trim().to_lowercase();
        
        // Check if it's a boolean value
        if is_boolean_value(&trimmed_value) {
            boolean_count += 1;
        }
        // Check if it's an integer
        else if trimmed_value.parse::<i64>().is_ok() {
            integer_count += 1;
        }
        // Check if it's a float
        else if trimmed_value.parse::<f64>().is_ok() {
            float_count += 1;
        }
        // Otherwise it's a string
    }
    
    // Determine type based on majority of samples
    // Require at least 70% of samples to match a type
    let threshold = (total_samples * 7) / 10; // 70% threshold
    
    if boolean_count >= threshold {
        CsvColumnDataType::Boolean
    } else if integer_count >= threshold {
        CsvColumnDataType::Integer
    } else if float_count >= threshold {
        CsvColumnDataType::Float
    } else {
        CsvColumnDataType::String
    }
}

/// Checks if a value represents a boolean
/// 
/// # Arguments
/// * `value` - The value to check (should be trimmed and lowercase)
/// 
/// # Returns
/// * `bool` - True if the value appears to be boolean
fn is_boolean_value(value: &str) -> bool {
    matches!(value, "true" | "false" | "yes" | "no" | "1" | "0" | "t" | "f" | "y" | "n")
}

/// Determines the path for the metadata TOML file based on CSV file path
/// 
/// # Arguments
/// * `csv_file_path` - Path to the CSV file
/// 
/// # Returns
/// * `RowsAndColumnsResult<PathBuf>` - Path to metadata file or error
fn determine_metadata_file_path(csv_file_path: &PathBuf) -> RowsAndColumnsResult<PathBuf> {
    let csv_filename_stem = csv_file_path.file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| {
            create_configuration_error(
                &format!("Cannot determine filename from CSV path: {}", csv_file_path.display())
            )
        })?;
    
    let metadata_filename = format!("{}.{}", csv_filename_stem, METADATA_FILE_EXTENSION);
    
    // Place metadata file in same directory as CSV file
    let csv_directory = csv_file_path.parent()
        .ok_or_else(|| {
            create_configuration_error(
                &format!("Cannot determine directory from CSV path: {}", csv_file_path.display())
            )
        })?;
    
    let metadata_file_path = csv_directory.join(metadata_filename);
    
    Ok(metadata_file_path)
}

/// Creates or updates the metadata TOML file with column information
/// 
/// # Arguments
/// * `metadata_file_path` - Path where metadata file should be created/updated
/// * `column_information_list` - List of column information to store
/// 
/// # Returns
/// * `RowsAndColumnsResult<()>` - Success or error
fn create_or_update_metadata_file(
    metadata_file_path: &PathBuf,
    column_information_list: &[CsvColumnInformation],
) -> RowsAndColumnsResult<()> {
    // Prepare parent directories if needed
    if let Some(parent_dir) = metadata_file_path.parent() {
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir).map_err(|io_error| {
                create_file_system_error(
                    &format!("Failed to create metadata file parent directory: {}", parent_dir.display()),
                    io_error
                )
            })?;
        }
    }
    
    // Create TOML content
    let mut toml_content = String::new();
    toml_content.push_str("# CSV Metadata File\n");
    toml_content.push_str("# Generated by rows_and_columns\n\n");
    
    toml_content.push_str(&format!("total_columns = {}\n", column_information_list.len()));
    toml_content.push_str("\n");
    
    // Add column information
    for column_info in column_information_list {
        let column_section = format!("column_{}", column_info.column_index + 1);
        toml_content.push_str(&format!("[{}]\n", column_section));
        toml_content.push_str(&format!("name = \"{}\"\n", column_info.column_name));
        toml_content.push_str(&format!("data_type = \"{}\"\n", column_info.detected_data_type.to_toml_string()));
        toml_content.push_str(&format!("column_index = {}\n", column_info.column_index));
        toml_content.push_str(&format!("non_empty_values = {}\n", column_info.non_empty_value_count));
        toml_content.push_str(&format!("empty_values = {}\n", column_info.empty_value_count));
        toml_content.push_str("\n");
    }
    
    // Write the file
    std::fs::write(metadata_file_path, toml_content)
        .map_err(|io_error| {
            create_file_system_error(
                &format!("Failed to write metadata file: {}", metadata_file_path.display()),
                io_error
            )
        })?;
    
    Ok(())
}