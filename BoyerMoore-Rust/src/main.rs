use std::ffi::CString;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Error, Read};
use std::ptr::null_mut;
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};
use winapi::um::fileapi::{CreateFileA, OPEN_EXISTING};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::memoryapi::{CreateFileMappingW, MapViewOfFile, UnmapViewOfFile, FILE_MAP_READ};
use winapi::um::winnt::{GENERIC_READ, HANDLE, IMAGE_DOS_HEADER, IMAGE_NT_HEADERS64, IMAGE_DOS_SIGNATURE, IMAGE_NT_SIGNATURE, PAGE_READONLY, IMAGE_FILE_MACHINE_I386, IMAGE_FILE_MACHINE_AMD64};

const NO_OF_CHARS: usize = 256;

// Preprocessing function for Boyer-Moore's bad character heuristic
fn bad_char_heuristic(pattern: &str) -> [i32; NO_OF_CHARS] {
    let mut badchar = [-1; NO_OF_CHARS];
    for (i, c) in pattern.chars().enumerate() {
        badchar[c as usize] = i as i32;
    }
    badchar
}

// Boyer-Moore search function
fn bm_search(file_path: &str, pattern: &str) {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut txt = Vec::new();
    file.read_to_end(&mut txt).expect("Failed to read file");
    
    let m = pattern.len() as isize;
    let n = txt.len() as isize;
    
    let badchar = bad_char_heuristic(pattern);

    let mut s = 0;
    while s <= n - m {
        let mut j = m - 1;

        while j >= 0 && pattern.chars().nth(j as usize).unwrap() as u8 == txt[(s + j) as usize] {
            j -= 1;
        }

        if j < 0 {
            println!("Pattern occurs at shift = {}", s);
            let shift = if s + m < n {
                let badchar_index = txt[(s + m) as usize] as usize;
                let badchar_value = badchar[badchar_index] as isize;
                (m - badchar_value).max(1) as usize
            } else {
                1
            };
            s += shift as isize; // Convert shift to isize before adding
        } else {
            let badchar_index = txt[(s + j) as usize] as usize;
            let badchar_value = badchar[badchar_index] as isize;
            s += (j - badchar_value).max(1) as isize; // Convert result to isize before adding
        }
    }
}

// Function to get file size
fn read_all_bytes(path: &str) -> Result<Vec<u8>, Error> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn list_files(path: &str) -> Result<Vec<String>, Error> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path.display().to_string());
        } else if path.is_dir() {
            files.extend(list_files(&path.display().to_string())?);
        }
    }
    Ok(files)
}

fn get_nt_header_signature(file_path: &str) -> Result<String, Error> {
    unsafe {
        let file_name = CString::new(file_path).unwrap();
        let file_handle = CreateFileA(
            file_name.as_ptr(),
            GENERIC_READ,
            0,
            null_mut(),
            OPEN_EXISTING,
            0,
            null_mut(),
        );

        if file_handle == INVALID_HANDLE_VALUE {
            return Err(Error::last_os_error());
        }

        let mapping_handle = CreateFileMappingW(
            file_handle,
            null_mut(),
            PAGE_READONLY,
            0,
            0,
            null_mut(),
        );

        if mapping_handle == null_mut() {
            CloseHandle(file_handle);
            return Err(Error::last_os_error());
        }

        let base_address = MapViewOfFile(
            mapping_handle,
            FILE_MAP_READ,
            0,
            0,
            0,
        );

        if base_address == null_mut() {
            CloseHandle(mapping_handle);
            CloseHandle(file_handle);
            return Err(Error::last_os_error());
        }

        let dos_header = &*(base_address as *const IMAGE_DOS_HEADER);
        if dos_header.e_magic != IMAGE_DOS_SIGNATURE {
            UnmapViewOfFile(base_address);
            CloseHandle(mapping_handle);
            CloseHandle(file_handle);
            return Err(Error::from_raw_os_error(87)); // ERROR_INVALID_PARAMETER
        }

        let nt_headers = &*((base_address as *const u8).offset(dos_header.e_lfanew as isize) as *const IMAGE_NT_HEADERS64);
        if nt_headers.Signature != IMAGE_NT_SIGNATURE {
            UnmapViewOfFile(base_address);
            CloseHandle(mapping_handle);
            CloseHandle(file_handle);
            return Err(Error::from_raw_os_error(87)); // ERROR_INVALID_PARAMETER
        }

        let file_header = &nt_headers.FileHeader;
        let machine = file_header.Machine;
        if machine != IMAGE_FILE_MACHINE_I386 && machine != IMAGE_FILE_MACHINE_AMD64 {
            UnmapViewOfFile(base_address);
            CloseHandle(mapping_handle);
            CloseHandle(file_handle);
            return Err(Error::from_raw_os_error(87)); // ERROR_INVALID_PARAMETER
        }

        let signature = nt_headers.Signature.to_le_bytes();
        let nt_signature_str = signature.iter().map(|b| *b as char).collect();

        UnmapViewOfFile(base_address);
        CloseHandle(mapping_handle);
        CloseHandle(file_handle);

        Ok(nt_signature_str)
    }
}

fn main() {
    let start_time = Instant::now(); // Start measuring time

    println!("Klasör yolunu gir: ");
    let mut directory = String::new();
    io::stdin().read_line(&mut directory).expect("Failed to read line");
    let directory = directory.trim();

    println!("CSV dosyasının yolunu gir: ");
    let mut csv_path = String::new();
    io::stdin().read_line(&mut csv_path).expect("Failed to read line");
    let csv_path = csv_path.trim();

    let dir_arr = match list_files(directory) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Klasördeki dosyalar listelenemedi: {}", e);
            return;
        }
    };

    let file = match File::open(csv_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("CSV dosyası açılamadı: {}", e);
            return;
        }
    };
    let reader = BufReader::new(file);
    let keywords: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    // Create a progress bar
    let pb = ProgressBar::new(dir_arr.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} [{elapsed_precise}] [{bar:40}] {percent}%")
        .progress_chars("#>-"));

    for entry in dir_arr {
        match read_all_bytes(&entry) {
            Ok(arr) => {
                if arr.len() > 1 && arr[0] == b'M' && arr[1] == b'Z' {
                    match get_nt_header_signature(&entry) {
                        Ok(nt_signature) => {
                            println!("\n{} NT header signature found (ASCII): {}", entry, nt_signature);
                            for keyword in &keywords {
                                bm_search(&entry, &keyword);
                            }
                            println!("\n\n");
                        },
                        Err(e) => eprintln!("NT header signature alınamadı: {}", e),
                    }
                }
            },
            Err(e) => eprintln!("Dosya okunamadı: {}", e),
        }
        pb.inc(1); 
    }

    pb.finish_with_message("Done"); 

    // Print elapsed time
    println!("Toplam süre: {:?}", start_time.elapsed());
}
