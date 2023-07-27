use filetime::{set_file_mtime, FileTime};
use std::env;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use std::process::Command;
use std::time::UNIX_EPOCH;
use threadpool::ThreadPool;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Wenn der Pfad nicht übergeben wurde.
    if args.len() < 2 {
        return;
    }

    let repo_exe_pfad = args[1].clone() + "/foxbin2prg.exe";
    let repo_forms_pfad = args[1].clone() + "/Masken";

    let paths = fs::read_dir(repo_forms_pfad).expect("Pfad konnte nicht gelesen werden.");

    let pool = ThreadPool::new(4);

    for path in paths {
        match path {
            Ok(wert) => {
                let path_copy = wert.path().clone();
                let repo_exe_pfad_copy = repo_exe_pfad.clone();
                pool.execute(move || {
                    match check_and_create_sc2_file(path_copy, &repo_exe_pfad_copy) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                });
            }
            Err(_) => {}
        }
    }
    pool.join();
}

fn check_and_create_sc2_file(path: PathBuf, repo_exe_pfad: &String) -> Result<(), io::Error> {
    let temp_path_scx = path;

    // Endung von der Datei holen und prüfen
    match temp_path_scx.extension() {
        Some(wert) => {
            if wert.to_ascii_lowercase() != "scx" {
                return Err(io::Error::new(
                    ErrorKind::Other,
                    "Datei hat nicht die richtige Endung",
                ));
            }
        }
        None => return Err(io::Error::new(ErrorKind::Other, "Datei hat keine Endung")),
    };

    // Hier haben wir nur Dateien mit einer scx Endung

    // Pfad zur sc2 Datei bauen
    let mut temp_path_sc2 = temp_path_scx.clone();
    temp_path_sc2.set_extension("sc2");

    let time_now = FileTime::now();
    // gibt es die Datei sc2
    if std::path::Path::new(&temp_path_sc2).exists() {
        // Hier das modification Date prüfen
        if get_last_edit_time(&temp_path_scx) != get_last_edit_time(&temp_path_sc2) {
            // Datei löschen
            fs::remove_file(&temp_path_sc2)?;

            // Neue SC2 Datei erstellen
            Command::new(&repo_exe_pfad).arg(&temp_path_scx).output()?;
            // Die Letzte Bearbeitungsezeit setzten
            set_file_mtime(&temp_path_sc2, time_now)?;
            set_file_mtime(&temp_path_scx, time_now)?;
        }
    } else {
        // Neue SC2 Datei erstellen
        Command::new(&repo_exe_pfad).arg(&temp_path_scx).output()?;
        // Die Letzte Bearbeitungsezeit setzten
        set_file_mtime(&temp_path_sc2, time_now)?;
        set_file_mtime(&temp_path_scx, time_now)?;
    }

    Ok(())
}

fn get_last_edit_time(pfad: &PathBuf) -> u64 {
    let mut temp: u64 = 0;
    fs::metadata(pfad).map_or((), |x| {
        x.modified().map_or((), |y| {
            y.duration_since(UNIX_EPOCH).map_or((), |z| {
                temp = z.as_secs();
            });
        })
    });
    temp
}
