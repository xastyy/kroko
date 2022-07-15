use std::collections::HashMap;
use std::collections::HashSet;
use std::{fs, io};
use std::path::PathBuf;
use std::string::*;
use std::env;


fn get_paths(p: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let mut entries = fs::read_dir(p)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();
    Ok(entries)
}


#[allow(dead_code)]
fn print_paths(paths: Vec<PathBuf>) {
    for e in paths {
        let fucku = e.into_os_string().into_string().unwrap();
        println!("{:?}", fucku.as_str().split("/").collect::<Vec<_>>().last().unwrap());
    }
}


fn add_number_to_filename(path: String, number: i32) -> String {
    let split_path = path.split(".").collect::<Vec<_>>();
    let first_half: String = match number.cmp(&10) {
        std::cmp::Ordering::Less => split_path.first()
                                              .unwrap()
                                              .to_string() + "0" + &number.to_string(),
        std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => split_path.first()
                                                                             .unwrap()
                                                                             .to_string() 
                                                                             + &number.to_string()
    };
    first_half + "." + &split_path.last().unwrap().to_string()
}




fn get_all_paths_in_map(sp: PathBuf) -> HashMap<String, HashSet<String>>{
    let mut all_paths: HashMap<String, HashSet<String>> = HashMap::new();
    let sample_folders = get_paths(sp).unwrap();
    for e in sample_folders {
        let f = get_paths(e).unwrap();
        for p in f {
            let path = p.into_os_string().into_string().unwrap();
            let path_segments = path.as_str().split("/").collect::<Vec<_>>(); 
            println!("{:?}", path_segments);
            let filename = &path_segments.last().unwrap().to_string();
            if !all_paths.contains_key(filename) {
                all_paths.insert(filename.to_string(),HashSet::new());
                all_paths.get_mut(filename).unwrap().insert(path);

            } else {
                all_paths.get_mut(filename).unwrap().insert(path);
            }
        }
    }


    all_paths
}


fn get_all_paths_in_map_with_ext(sp: PathBuf, ext: String) -> HashMap<String, HashSet<String>>{
    let mut all_paths: HashMap<String, HashSet<String>> = HashMap::new();
    let sample_folders = get_paths(sp).unwrap();
    for e in sample_folders {
        let f = get_paths(e).unwrap();
        for p in f {
            let path = p.into_os_string().into_string().unwrap();
            let path_segments = path.as_str().split("/").collect::<Vec<_>>(); 
            let extension = path_segments.last().unwrap().split(".").collect::<Vec<_>>();
            if extension.last().unwrap().to_string() == ext.to_string() {
                println!("{:?}", path_segments);
                let filename = &path_segments.last().unwrap().to_string();
                if !all_paths.contains_key(filename) {
                    all_paths.insert(filename.to_string(),HashSet::new());
                    all_paths.get_mut(filename).unwrap().insert(path);

                } else {
                    all_paths.get_mut(filename).unwrap().insert(path);
                }
            } 
        }
    }
    all_paths
}




fn grab_files_into_folders(makedirs: bool, paths: HashMap<String, HashSet<String>>) {
    if makedirs {
        match fs::create_dir("other") {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {},
        }
    }

    for (k, v) in paths.iter() {
        if v.len() > 1 {
            // make directory for key
            println!("{:#?}", k.split(".").collect::<Vec<_>>().first());
            let mut cnt: i32 = 1;
            for i in v {
                println!("{}", add_number_to_filename(i.to_string(), cnt));
                cnt += 1;
                // copy i to new path
            }
        } else {
            // copy the single value to the 'other' folder
        }
    }
}


fn main() {
    let mut createdirs: bool = false;
    let samples_path: PathBuf; //= PathBuf::from(r"/home/bindi/private/code/rust/kroko_crawler/test"); // /home/bindi/kroko_packs/Hard House
    let filetype: String;
    let args: Vec<String> = env::args().collect();
    match args.len() {
        // no arguments passed
        1 => {
            println!("kroko file grabber:\nUsage: kroko [-c][-f FILETYPE] PATH\n\n-c ... copy the file into folders \n-f ... only use files from this type");
        },
        // one argument passed
        2 => {
            let p = &args[1];
            //samples_path = p.split("/").collect();
            samples_path = PathBuf::from(p);
            println!("{:?}", samples_path);
            grab_files_into_folders(createdirs, get_all_paths_in_map(fs::canonicalize(&samples_path).unwrap()));
            
        },
        3 => {
            let cmd = &args[1];
            let p = &args[2];
            match &cmd[..] {
                "-c" => createdirs = true,
                _ => {
                    eprintln!("error: invalid command");
                    println!("kroko file grabber:\nUsage: kroko [-c][-f FILETYPE] PATH\n\n-c ... copy the file into folders \n-f ... only use files from this type");
                },
            }
            samples_path = PathBuf::from(p);
            grab_files_into_folders(createdirs, get_all_paths_in_map(fs::canonicalize(samples_path).unwrap()));
        },
        // one command and one argument passed kroko
        4 => {
            let cmd = &args[1];
            let t = &args[2];
            let p = &args[3];
            match &cmd[..] {
                "-f" => {
                    filetype = t.to_string();
                    samples_path = PathBuf::from(p);
                    grab_files_into_folders(createdirs, get_all_paths_in_map_with_ext(fs::canonicalize(samples_path).unwrap(), filetype));
                },
                _ => {
                    eprintln!("error: invalid command");
                    println!("kroko file grabber:\nUsage: kroko [-c][-f FILETYPE] PATH\n\n-c ... copy the file into folders \n-f ... only use files from this type");
                },
            }
        },
        5 => {
            let cmd = &args[1];
            let cmd2 = &args[2];
            let t = &args[3];
            let p =  &args[4];
            match &cmd[..] {
                "-c" => createdirs = true,
                _ => {
                    eprintln!("error: invalid command");
                    println!("kroko file grabber:\nUsage: kroko [-c][-f FILETYPE] PATH\n\n-c ... copy the file into folders \n-f ... only use files from this type");
                },
            }
            match &cmd2[..] {
                "-f" => {
                    filetype = t.to_string();
                    samples_path = PathBuf::from(p);
                    grab_files_into_folders(createdirs, get_all_paths_in_map_with_ext(fs::canonicalize(samples_path).unwrap(), filetype));
                },
                _ => {
                    eprintln!("error: invalid command");
                    println!("kroko file grabber:\nUsage: kroko [-c][-f FILETYPE] PATH\n\n-c ... copy the file into folders \n-f ... only use files from this type");
                },
            }
        },
        // all the other cases
        _ => {
            // show a help message
            println!("kroko file grabber:\nUsage: kroko [-c][-f FILETYPE] PATH\n\n-c ... copy the file into folders \n-f ... only use files from this type");
        }
    }
    
}