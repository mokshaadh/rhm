use std::collections::VecDeque;
use std::path::PathBuf;
use std::{fs, io};

struct Searcher<'a> {
    dirs: Vec<PathBuf>,
    excluded_dirs: Vec<PathBuf>,
    search_term: &'a str,
    max_results: i32,
    max_depth: i32,
}

impl Searcher<'_> {
    fn search_dir(&self, dir: PathBuf, num_results: &mut i32) -> io::Result<Vec<PathBuf>> {
        if !dir.is_dir() || *num_results >= self.max_results {
            return Ok(vec![]);
        }

        let mut retr = Vec::new();
        let mut to_search = VecDeque::new();
        to_search.push_front((dir, 0));

        while !to_search.is_empty() && *num_results < self.max_results {
            let (current_dir, current_depth) = to_search.pop_back().unwrap();
            for entry in fs::read_dir(current_dir)? {
                let entry = entry?;
                let path = entry.path();

                if current_depth < self.max_depth
                    && path.is_dir()
                    && !self.excluded_dirs.contains(&path)
                {
                    to_search.push_back((path.clone(), current_depth + 1));
                }
                if path.ends_with(self.search_term) {
                    retr.push(path);
                    *num_results += 1;
                }
            }
        }

        Ok(retr)
    }

    pub fn search(&self) -> io::Result<Vec<PathBuf>> {
        let mut num_results = 0;
        let mut retr = Vec::new();

        for dir in &self.dirs {
            retr.extend(self.search_dir(dir.clone(), &mut num_results)?);
        }

        Ok(retr)
    }
}

fn main() -> io::Result<()> {
    let home_directory = home::home_dir().unwrap();
    let library_directory = {
        let mut tmp = home_directory.clone();
        tmp.push("Library");
        tmp
    };

    let srchr = Searcher {
        dirs: vec![home_directory],
        excluded_dirs: vec![library_directory],
        search_term: "Dev",
        max_results: 20,
        max_depth: 100,
    };
    println!("{:?}", srchr.search()?);

    let srchr2 = Searcher {
        dirs: vec![
            PathBuf::from("/Applications"),
            PathBuf::from("/System/Library/CoreServices/Applications"),
            PathBuf::from("/Applications/Xcode.app/Contents/Applications"),
        ],
        excluded_dirs: vec![],
        search_term: "Create ML.app",
        max_results: 2,
        max_depth: 1,
    };
    println!("{:?}", srchr2.search()?);

    Ok(())
}
