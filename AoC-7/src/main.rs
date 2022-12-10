#![allow(non_snake_case)]

// Abstractions to work with multiple input formats.
const COMMAND_PREFIX: &str = "$ ";
const COMMAND_CD_PREFIX: &str = "cd ";
const COMMAND_CD_UP: &str = "..";
const COMMAND_LS: &str = "ls";
const COMMAND_LS_DIR_PREFIX: &str = "dir ";
const ROOT_DIR_SYMBOL: &str = "/";
// const DIR_SEPARATOR: &str = "/";
const FILESYSTEM_SIZE: usize = 70_000_000;
const UPDATE_SIZE: usize = 30_000_000;

use std::cell::RefCell;
use std::rc::Rc;

type DirRef = Rc<RefCell<Directory>>;
type FileRef = Rc<RefCell<File>>;

fn main() {
    let input = include_str!("input.txt");
    let mut state = State::default();

    for line in input.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with(COMMAND_PREFIX) {
            // Not printing if entering a command.
            state.printing = false;
            let command = trimmed.strip_prefix(COMMAND_PREFIX).unwrap();
            state.last_command = command.to_owned();

            if command.starts_with(COMMAND_CD_PREFIX) {
                // Strips the command prompt prefix.
                let directory_name = command
                    .strip_prefix(COMMAND_CD_PREFIX)
                    .expect("No directory was provided to move into.");

                change_directory(&mut state, directory_name);
                continue;
            }

            if command.starts_with(COMMAND_LS) {
                // Running the list command enters printing mode.
                state.printing = true;
            }

            continue;
        }

        if state.printing {
            // Only command that prints at the moment is list.
            process_ls_entry(&mut state, trimmed);
        }
    }

    let directories = state.filesystem.borrow().all_subdirectories();
    let mut part_one_total = 0;
    
    for directory in &directories {
        let size = directory.borrow().size();

        if size <= 100_000 {
            part_one_total += size;
        }
    }

    let current_fs_size = state.filesystem.borrow().size();
    let mut candidate_size = 0;
    let space_needed = FILESYSTEM_SIZE - UPDATE_SIZE;

    for directory in &directories {
        let size = directory.borrow().size();

        if current_fs_size - size < space_needed {
            if candidate_size != 0 {
                if candidate_size > size {
                    candidate_size = size;
                }

                continue;
            }

            candidate_size = size;
        }
    }

    println!("Part One Total: {part_one_total}");
    println!("Part Two Directory Size: {candidate_size}");
}

struct State {
    filesystem: DirRef,
    working_directory: DirRef,
    printing: bool,
    last_command: String,
}

struct Directory {
    // Empty name is reserved for the root directory.
    name: String,
    subdirectories: Vec<DirRef>,
    files: Vec<FileRef>,
    parent: Option<DirRef>,
}

struct File {
    // name: String,
    size: usize,
}

impl Default for State {
    fn default() -> Self {
        let root = Directory {
            name: String::new(),
            subdirectories: vec![],
            files: vec![],
            parent: None,
        };

        let filesystem = Rc::new(RefCell::new(root));
        let working_directory: DirRef = filesystem.clone();

        // Encapsulate!
        State {
            filesystem,
            working_directory,
            printing: false,
            last_command: String::new(),
        }
    }
}

impl Directory {
    fn all_subdirectories(&self) -> Vec<DirRef> {
        let mut collection = self.subdirectories.clone();

        let mut super_subdirectories = self
            .subdirectories
            .iter()
            .flat_map(|d| d.borrow().all_subdirectories())
            .collect::<Vec<DirRef>>();

        collection.append(&mut super_subdirectories);
        collection
    }

    fn all_files(&self) -> Vec<FileRef> {
        let mut collection = self.files.clone();

        let mut subfiles = self
            .subdirectories
            .iter()
            .flat_map(|d| d.borrow().all_files())
            .collect::<Vec<FileRef>>();

        collection.append(&mut subfiles);
        collection
    }

    fn size(&self) -> usize {
        let files = self.all_files();
        files.into_iter().map(|f| f.borrow().size).sum()
    }
}

// fn qualified_name(path: DirRef) -> String {
    // let first_name = path.borrow().name.clone();
    // let mut name_chain = vec![first_name];
    // let mut working_directory = path;

    // while working_directory.borrow().parent.is_some() {
        // let parent = working_directory.borrow().parent.as_ref().unwrap().clone();
        // let name = parent.borrow().name.clone();
        // name_chain.insert(0, name);
        // working_directory = parent.clone();
    // }

    // name_chain.join(DIR_SEPARATOR)
// }

fn change_directory(state: &mut State, directory_name: &str) {
    if directory_name == COMMAND_CD_UP {
        let parent_reference = state.working_directory.borrow().parent.clone();

        if let Some(directory) = parent_reference {
            state.working_directory = directory;
            return;
        }

        panic!("Attempting to find parent of root directory.");
    }

    if directory_name == ROOT_DIR_SYMBOL {
        state.working_directory = state.filesystem.clone();
        return;
    }

    let subdir_reference = state
        .working_directory
        .borrow()
        .subdirectories
        .iter()
        .find(|d| d.borrow().name == directory_name)
        .cloned();

    if let Some(directory) = subdir_reference {
        state.working_directory = directory;
        return;
    }

    let new_directory = Directory {
        name: directory_name.to_string(),
        subdirectories: vec![],
        files: vec![],
        parent: Some(state.working_directory.clone()),
    };

    let new_dir_reference = Rc::new(RefCell::new(new_directory));
    state
        .working_directory
        .borrow_mut()
        .subdirectories
        .push(new_dir_reference.clone());
    state.working_directory = new_dir_reference;
}

fn process_ls_entry(state: &mut State, entry: &str) {
    if entry.starts_with(COMMAND_LS_DIR_PREFIX) {
        let directory_name = entry.strip_prefix(COMMAND_LS_DIR_PREFIX).unwrap();

        // Check whether the directory already exists.
        // If it does, we don't create a copy.
        for directory in &state.working_directory.borrow().subdirectories {
            if directory.borrow().name == directory_name {
                return;
            }
        }

        let new_directory = Directory {
            name: directory_name.to_string(),
            subdirectories: vec![],
            files: vec![],
            parent: Some(state.working_directory.clone()),
        };

        let new_dir_reference = Rc::new(RefCell::new(new_directory));
        state
            .working_directory
            .borrow_mut()
            .subdirectories
            .push(new_dir_reference);
        return;
    }

    // Find space separating file size and name.
    let err_message = "List command produced unexpect output.";
    let space_index = entry.find(' ').expect(err_message);
    let split = entry.split_at(space_index);
    let size_string = split.0;
    let size: usize = size_string.parse::<usize>().expect(err_message);
    // let name = split.1.trim().to_string();

    let file = File { size };

    let file_reference = Rc::new(RefCell::new(file));
    state
        .working_directory
        .borrow_mut()
        .files
        .push(file_reference);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_recursion_works() {
        let mkdir = |name: &str| Directory {
            name: name.to_string(),
            subdirectories: vec![],
            files: vec![],
            parent: None,
        };

        let touch = |name: &str, size: usize| File {
            // name: name.to_string(),
            size,
        };

        let mut a = mkdir("a");
        let mut b = mkdir("b");
        let mut c = mkdir("c");
        let mut d = mkdir("d");
        let mut f = mkdir("f");
        d.files
            .push(Rc::new(RefCell::new(touch("The Beatles Biography", 35000))));
        f.files.push(Rc::new(RefCell::new(touch(
            "Beatles: What are they?",
            5000,
        ))));
        f.files.push(Rc::new(RefCell::new(touch(
            "Magnifience of the Micro: Bugs",
            10000,
        ))));
        b.subdirectories.push(Rc::new(RefCell::new(d)));
        c.subdirectories.push(Rc::new(RefCell::new(f)));
        a.subdirectories.push(Rc::new(RefCell::new(b)));
        a.subdirectories.push(Rc::new(RefCell::new(c)));

        let result = a.size();
        let expected = 50000;
        assert_eq!(result, expected, "Size totalling failed.");

        let result = a.all_subdirectories().len();
        let expected = 4;
        assert_eq!(result, expected, "Subdirectory recursion failed.");
    }
}
