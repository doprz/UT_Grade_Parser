use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
/// This module contains functions for parsing CSV files containing course information.
/// It provides functions to parse individual CSV files and directories containing multiple CSV files.
/// Parsed data is written to another CSV file.
///
/// The main functions in this module are:
/// - `parse_csv_file`: Parses a single CSV file and writes the parsed data to another CSV file.
/// - `parse_csv_directory`: Parses a directory containing multiple CSV files and writes the parsed data to corresponding output CSV files.
///
/// The module also defines two structs:
/// - `CourseInfo`: Represents the information of a course.
/// - `CourseInfoTokenized`: Represents the tokenized information of a course.
///
/// The module uses the `encoding_rs` crate for decoding UTF-16LE encoded files.
/// It also uses the `std::collections::HashMap` struct for storing and manipulating course information.
///
/// Example usage:
/// ```
/// use parse::{parse_csv_file, parse_csv_directory};
///
/// // Parse a single CSV file
/// parse_csv_file("input.csv", "output.csv");
///
/// // Parse a directory containing multiple CSV files
/// parse_csv_directory("input_directory", "output_directory");
/// ```

const CSV_HEADER: &str = "Semester\tSection\tDepartment\tDepartment Code\tCourse Number\tCourse Title\tCourse Full Title\tA\tA-\tB+\tB\tB-\tC+\tC\tC-\tD+\tD\tD-\tF\tOther";

const GRADE_NAMES: [&str; 13] = [
    "A", "A-", "B+", "B", "B-", "C+", "C", "C-", "D+", "D", "D-", "F", "Other",
];

/// Represents the information of a course.
#[derive(Serialize, Deserialize, Debug)]
struct CourseInfo {
    semester: String,
    section: u32,
    department: String,
    department_code: String,
    course_number: String,
    course_title: String,
    course_full_title: String,
    grade: HashMap<String, u16>,
}

/// Represents the tokenized information of a course.
#[derive(Serialize, Deserialize, Debug)]
struct CourseInfoTokenized {
    semester: String,
    section: u32,
    department: String,
    department_code: String,
    course_number: String,
    course_title: String,
    course_full_title: String,
    grade: String,
    grade_count: u16,
}

/// Parses the input string and returns a `CourseInfoTokenized` struct.
///
/// # Arguments
///
/// * `record` - A `Result<csv::StringRecord, csv::Error>` containing the input string.
///
/// # Returns
///
/// * `Ok(CourseInfoTokenized)` - If the input string is successfully parsed.
/// * `Err(())` - If the input string cannot be parsed.
fn parse_course_info(
    record: Result<csv::StringRecord, csv::Error>,
) -> Result<CourseInfoTokenized, Box<dyn std::error::Error>> {
    let tokens = record?;
    if tokens.len() == 9 {
        let semester: String = tokens[0].to_string();
        let section: u32 = tokens[1].parse::<u32>().unwrap();
        let department: String = tokens[2].to_string();
        let department_code: String = tokens[3].to_string();
        let course_number: String = tokens[4].trim().to_string();
        let course_title: String = tokens[5].to_string();
        let course_full_title: String = tokens[6].to_string();
        let grade: String = tokens[7].to_string();
        let grade_count: u16 = tokens[8].replace(",", "").parse::<u16>().unwrap();

        Ok(CourseInfoTokenized {
            semester,
            section,
            department,
            department_code,
            course_number,
            course_title,
            course_full_title,
            grade,
            grade_count,
        })
    } else {
        Err(format!(
            "Invalid number of tokens. Expected 9, received {}",
            tokens.len()
        )
        .into())
    }
}

/// Parses a CSV file containing course information and writes the parsed data to another CSV file.
///
/// # Arguments
///
/// * `input_file` - The path to the input CSV file.
/// * `output_file` - The path to the output CSV file.
///
/// # Example
///
/// ```
/// use parse::parse_csv_file;
///
/// parse_csv_file("input.csv", "output.csv");
/// ```
pub fn parse_csv_file(
    input_file: &str,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut csv_reader = csv::Reader::from_path(input_file)?;
    let mut course_info_map: HashMap<String, CourseInfo> = HashMap::new();

    for record in csv_reader.records() {
        let course_info = parse_course_info(record)?;
        let course_full_title: String = course_info.course_full_title.clone();

        if let Some(existing_course_info) = course_info_map.get_mut(&course_full_title) {
            existing_course_info
                .grade
                .insert(course_info.grade, course_info.grade_count);
        } else {
            let mut new_course_info = CourseInfo {
                semester: course_info.semester,
                section: course_info.section,
                department: course_info.department,
                department_code: course_info.department_code,
                course_number: course_info.course_number,
                course_title: course_info.course_title,
                course_full_title: course_info.course_full_title,
                grade: {
                    let mut grade_map = HashMap::new();
                    grade_map.insert("A".to_string(), 0);
                    grade_map.insert("A-".to_string(), 0);
                    grade_map.insert("B+".to_string(), 0);
                    grade_map.insert("B".to_string(), 0);
                    grade_map.insert("B-".to_string(), 0);
                    grade_map.insert("C+".to_string(), 0);
                    grade_map.insert("C".to_string(), 0);
                    grade_map.insert("C-".to_string(), 0);
                    grade_map.insert("D+".to_string(), 0);
                    grade_map.insert("D".to_string(), 0);
                    grade_map.insert("D-".to_string(), 0);
                    grade_map.insert("F".to_string(), 0);
                    grade_map.insert("Other".to_string(), 0);
                    grade_map
                },
            };

            // Edge-case: UT doesn't have an A+ grade but it's in the data
            if course_info.grade == "A+" {
                // Add the A+ grade to the A grade
                let a_grade_count =
                    course_info.grade_count + new_course_info.grade.get("A").unwrap();
                new_course_info.grade.insert("A".to_string(), a_grade_count);
            } else {
                new_course_info
                    .grade
                    .insert(course_info.grade, course_info.grade_count);
            }
            course_info_map.insert(course_full_title, new_course_info);
        }
    }

    let mut csv_output_file: File =
        File::create(output_file).expect(&format!("Failed to create output file: {}", output_file));

    csv_output_file
        .write_all(CSV_HEADER.as_bytes())
        .expect(&format!("Failed to write header to file: {}", output_file));

    for (_, course_info) in course_info_map.iter() {
        let mut output_line: String = format!(
            "\n{}\t{}\t{}\t{}\t{}\t{}\t{}",
            course_info.semester,
            course_info.section,
            course_info.department,
            course_info.department_code,
            course_info.course_number,
            course_info.course_title,
            course_info.course_full_title
        );

        for grade_name in GRADE_NAMES.iter() {
            let grade_count = course_info.grade.get(*grade_name).unwrap();
            output_line.push_str(&format!("\t{}", grade_count));
        }

        csv_output_file
            .write_all(output_line.as_bytes())
            .expect(&format!("Failed to write output line: {}", output_line));
    }

    Ok(())
}

/// Parses a directory containing multiple CSV files and writes the parsed data to corresponding output CSV files.
///
/// # Arguments
///
/// * `input_directory` - The path to the input directory.
/// * `output_directory` - The path to the output directory.
///
/// # Example
///
/// ```
/// use parse::parse_csv_directory;
///
/// parse_csv_directory("input_directory", "output_directory");
/// ```
pub fn parse_csv_directory(input_directory: &str, output_directory: &str) {
    // Create the output directory if it doesn't exist
    std::fs::create_dir_all(output_directory).unwrap();

    let paths = std::fs::read_dir(input_directory).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let output_file = format!("{}/{}", output_directory, file_name);
        if let Err(err) = parse_csv_file(path.to_str().unwrap(), &output_file) {
            eprintln!("Failed to parse CSV file: {}", err);
        }
    }
}
