use std::collections::HashMap;
use std::collections::HashSet;
use std::{fs, io};
use std::path::PathBuf;
use std::string::*;


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

fn main() {
    let mut all_paths: HashMap<String, HashSet<String>> = HashMap::new();
    let samples_path = PathBuf::from(r"/home/bindi/kroko_packs/Hard House");
    let sample_folders = get_paths(samples_path).unwrap();
    for e in sample_folders {
        let f = get_paths(e).unwrap();
        for p in f {
            let path = p.into_os_string().into_string().unwrap();
            let path_segments = path.as_str().split("/").collect::<Vec<_>>(); 
            let extension = path_segments.last().unwrap().split(".").collect::<Vec<_>>();
            if extension.last().unwrap().to_string() == "wav".to_string() {
                //println!("{:?}", path_segments);
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
    //println!("\n{:?}", all_paths);
    //println!("\n");

    //these where from the for loop below
    //println!("{:?}: {:?}", k, v.len());
    //println!("{:?}", k.split(".").collect::<Vec<_>>().first());

    match fs::create_dir("other") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    for (k, v) in all_paths.iter() {
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
            // copy the single value to the only once folder
        }
    }
}

