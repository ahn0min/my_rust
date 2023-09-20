/**
 *  현재 루트경로를 기준으로 폴더명만 추출하여 README.md를 생성하고 마크다운 문법을 이용하여 
 *  table의 header 즉 표 제목에 넣어주는 유틸함수입니다.
 */

use std::fs;

fn main() {
    let mut folder_names = Vec::new();
    
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries  {
            if let Ok(entry) = entry {
                if entry.file_type().unwrap().is_dir() {
                    if let Some(folder_name) = entry.file_name().to_str() {
                        folder_names.push(folder_name.to_string())
                    }
                }
            }
            
        }
    }

    if let Err(err) = create_readme(folder_names) {
        eprintln!("Readme 파일 생성 중 오류 발생: {}", err);
    }
}


use std::fs::File;
use std::io::{self, Write};



fn create_readme(folder_names:Vec<String>) -> io::Result<()> {
    // Readme 파일 생성 또는 열기
    let mut readme_file = File::create("Readme2.md")?;

    for folder_name in &folder_names {
        let table_head = format!("| {} |\n|----------|\n| value |\n\n", folder_name);
        readme_file.write_all(table_head.as_bytes())?;
    }

    println!("Readme 파일이 생성되었습니다.");
    Ok(())
}
