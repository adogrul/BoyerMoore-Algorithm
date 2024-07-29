use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::cmp;
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};

const NO_OF_CHARS: usize = 256;

fn bad_char_heuristic(pat: &str, badchar: &mut [i32; NO_OF_CHARS]) {
    for i in 0..NO_OF_CHARS {
        badchar[i] = -1;
    }

    for (i, &byte) in pat.as_bytes().iter().enumerate() {
        badchar[byte as usize] = i as i32;
    }
}

fn read_all_bytes(path: &str) -> io::Result<Vec<u8>> {
    fs::read(path)
}

fn search(file_path: &str, pat: &str) -> io::Result<()> {
    let txt = read_all_bytes(file_path)?;
    let m = pat.len();
    let n = txt.len();

    if m == 0 || n == 0 || m > n {
        return Ok(());
    }

    let mut badchar = [-1; NO_OF_CHARS];
    bad_char_heuristic(pat, &mut badchar);

    let mut s: usize = 0;
    while s <= n - m {
        let mut j = (m - 1) as i32;

        while j >= 0 && pat.as_bytes()[j as usize] == txt[s + j as usize] {
            j -= 1;
        }

        if j < 0 {
            println!("Pattern '{}' occurs at shift = {} in file '{}'", pat, s, file_path);
            s = s.saturating_add(if s + m < n {
                m - badchar[txt[s + m] as usize] as usize
            } else {
                1
            });
        } else {
            s = s.saturating_add(cmp::max(1, (j - badchar[txt[s + j as usize] as usize]) as usize));
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut directory = String::new();
    println!("Klasör yolunu gir(enter folder path): ");
    io::stdin().read_line(&mut directory)?;
    let directory = directory.trim();

    let mut csv_path = String::new();
    println!("CSV dosyasının yolunu gir(enter CSV file path): ");
    io::stdin().read_line(&mut csv_path)?;
    let csv_path = csv_path.trim();

    let dir_entries = fs::read_dir(directory)?
        .filter_map(Result::ok)
        .map(|entry| entry.path().to_string_lossy().to_string())
        .collect::<Vec<_>>();

    let file = File::open(csv_path)?;
    let reader = io::BufReader::new(file);

    let keywords: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .collect();

    let pb = ProgressBar::new(dir_entries.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} [{elapsed_precise}] [{bar:40}] {percent}%")
        .progress_chars("#>-"));

    let start_time = Instant::now();

    for entry in &dir_entries {
        for keyword in &keywords {
            pb.set_message(format!("Searching for '{}' in file '{}'", keyword, entry));
            search(entry, keyword)?;
        }
        pb.inc(1);
    }

    pb.finish_with_message("Done");
    let total_duration = start_time.elapsed();
    println!("Total duration: {:.2?}", total_duration);

    Ok(())
}
