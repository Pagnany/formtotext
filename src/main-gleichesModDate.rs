use filetime::{set_file_mtime, FileTime};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::UNIX_EPOCH;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Wenn der Pfad nicht übergeben wurde.
    if args.len() < 2 {
        return;
    }

    let repo_exe_pfad = args[1].clone() + "/foxbin2prg.exe";
    let repo_forms_pfad = args[1].clone() + "/Masken";

    let paths = fs::read_dir(repo_forms_pfad).expect("Pfad konnte nicht gelesen werden.");

    for path in paths {
        let temp_path_scx = match path {
            Ok(wert) => wert.path(),
            _ => continue,
        };

        // Endung von der Datei holen und prüfen
        match temp_path_scx.extension() {
            Some(wert) => {
                if wert.to_ascii_lowercase() != "scx" {
                    continue;
                }
            }
            _ => continue,
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
                match fs::remove_file(&temp_path_sc2) {
                    Ok(_wert) => (),
                    _ => ()
                }
                // Neue SC2 Datei erstellen
                match Command::new(&repo_exe_pfad).arg(&temp_path_scx).output() {
                    Ok(_wert) => (),
                    _ => ()
                }
                // Die Letzte Bearbeitungsezeit setzten
                match set_file_mtime(&temp_path_sc2, time_now) {
                    Ok(_wert) => (),
                    _ => {continue;}
                }
                match set_file_mtime(&temp_path_scx, time_now) {
                    Ok(_wert) => (),
                    _ => {continue;}
                }
            }
        } else {
            // Neue SC2 Datei erstellen
            match Command::new(&repo_exe_pfad).arg(&temp_path_scx).output() {
                Ok(_wert) => (),
                _ => (),
            }
            // Die Letzte Bearbeitungsezeit setzten
            match set_file_mtime(&temp_path_sc2, time_now) {
                Ok(_wert) => (),
                _ => {continue;}
            }
            match set_file_mtime(&temp_path_scx, time_now) {
                Ok(_wert) => (),
                _ => {continue;}
            }
        }
    }
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
