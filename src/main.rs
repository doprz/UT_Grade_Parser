use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use encoding_rs::*;

const CSV_HEADER: &str = "Year\tSemester\tSection\tDepartment\tDepartment Code\tCourse Number\tCourse Title\tCourse Full Title\tA\tA-\tB+\tB\tB-\tC+\tC\tC-\tD+\tD\tD-\tF\tOther";

#[derive(Debug)]
struct CourseInfo {
    year: String,
    semester: String,
    section: u16,
    department: String,
    department_code: String,
    course_number: String,
    course_title: String,
    course_full_title: String,
    grade: HashMap<String, u16>,
}

#[derive(Debug)]
struct CourseInfoTokenized {
    // Example
    // ["2021-2022", "Spring 2022", "180", "Study Abroad Exchange Progs", 
    // "SAB", " 300", "STUDY ABROAD EXCHANGE PROGRAMS", 
    // "SAB-300: STUDY ABROAD EXCHANGE PROGRAMS, Section no. 180", "A", "38"]
    year: String,
    semester: String,
    section: u16,
    department: String,
    department_code: String,
    course_number: String,
    course_title: String,
    course_full_title: String,
    grade: String,
    grade_count: u16,
}

fn parse_course_info(input: &str) -> Result<CourseInfoTokenized, ()>{
    let tokens: Vec<&str> = input.split('\t').collect::<Vec<&str>>();

    if tokens.len() == 10 {
        let year: String = tokens[0].to_string();
        let semester: String = tokens[1].to_string();
        let section: u16 = tokens[2].parse::<u16>().unwrap();
        let department: String = tokens[3].to_string();
        let department_code: String = tokens[4].to_string();
        let course_number: String = tokens[5].trim().to_string();
        let course_title: String = tokens[6].to_string();
        let course_full_title: String = tokens[7].to_string();
        let grade: String = tokens[8].to_string();
        let grade_count: u16 = tokens[9].replace(",", "").parse::<u16>().unwrap();

        Ok(CourseInfoTokenized {
            year,
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
        Err(())
    }
}

fn main() {
    let mut file: File = File::open("input.csv").expect("Failed to open file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    // UT uses UTF-16LE encoding
    let (decoded_string, _, _) = UTF_16LE.decode(&buffer);

    let mut course_info_map: HashMap<String, CourseInfo> = HashMap::new();

    for line in decoded_string.lines() {
        let course_info: CourseInfoTokenized = parse_course_info(line).expect("Failed to parse course info");
        let course_full_title: String = course_info.course_full_title.clone();

        if let Some(existing_course_info) = course_info_map.get_mut(&course_full_title) {
            existing_course_info.grade.insert(course_info.grade, course_info.grade_count);
        } else {
            let mut new_course_info = CourseInfo {
                year: course_info.year,
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
            new_course_info.grade.insert(course_info.grade, course_info.grade_count);
            course_info_map.insert(course_full_title, new_course_info);
        }
    }

    // println!("{:#?}", course_info_map);

    let mut csv_output_file: File = File::create("output.csv").expect("Failed to create output file");

    csv_output_file.write_all(CSV_HEADER.as_bytes()).expect("Failed to write header");

    for (_, course_info) in course_info_map.iter() {
        let mut output_line: String = format!(
            "\n{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            course_info.year,
            course_info.semester,
            course_info.section,
            course_info.department,
            course_info.department_code,
            course_info.course_number,
            course_info.course_title,
            course_info.course_full_title
        );

        for (_, grade_count) in course_info.grade.iter() {
            output_line.push_str(&format!("\t{}", grade_count));
        }

        csv_output_file.write_all(output_line.as_bytes()).expect("Failed to write output line");
    }
}