use super::load_file;
use std::cell::RefCell;
use std::rc::Rc;

/// --- Day 7: No Space Left On Device ---
/// You can hear birds chirping and raindrops hitting leaves as the expedition proceeds.
/// Occasionally, you can even hear much louder sounds in the distance; how big do the animals get
/// out here, anyway?
///
/// The device the Elves gave you has problems with more than just its communication system. You try
/// to run a system update:
///
/// $ system-update --please --pretty-please-with-sugar-on-top
/// Error: No space left on device
/// Perhaps you can delete some files to make space for the update?
///
/// You browse around the filesystem to assess the situation and save the resulting terminal output
/// (your puzzle input). For example:
///
/// $ cd /
/// $ ls
/// dir a
/// 14848514 b.txt
/// 8504156 c.dat
/// dir d
/// $ cd a
/// $ ls
/// dir e
/// 29116 f
/// 2557 g
/// 62596 h.lst
/// $ cd e
/// $ ls
/// 584 i
/// $ cd ..
/// $ cd ..
/// $ cd d
/// $ ls
/// 4060174 j
/// 8033020 d.log
/// 5626152 d.ext
/// 7214296 k
/// The filesystem consists of a tree of files (plain data) and directories (which can contain other
/// directories or files). The outermost directory is called /. You can navigate around the
/// filesystem, moving into or out of directories and listing the contents of the directory you're
/// currently in.
///
/// Within the terminal output, lines that begin with $ are commands you executed, very much like
/// some modern computers:
///
/// cd means change directory. This changes which directory is the current directory, but the
/// specific result depends on the argument: cd x moves in one level: it looks in the current
/// directory for the directory named x and makes it the current directory. cd .. moves out one
/// level: it finds the directory that contains the current directory, then makes that directory the
/// current directory. cd / switches the current directory to the outermost directory, /.
/// ls means list. It prints out all of the files and directories immediately contained by the
/// current directory: 123 abc means that the current directory contains a file named abc with size
/// 123. dir xyz means that the current directory contains a directory named xyz.
/// Given the commands and output in the example above, you can determine that the filesystem looks
/// visually like this:
///
/// - / (dir)
///   - a (dir)
///     - e (dir)
///       - i (file, size=584)
///     - f (file, size=29116)
///     - g (file, size=2557)
///     - h.lst (file, size=62596)
///   - b.txt (file, size=14848514)
///   - c.dat (file, size=8504156)
///   - d (dir)
///     - j (file, size=4060174)
///     - d.log (file, size=8033020)
///     - d.ext (file, size=5626152)
///     - k (file, size=7214296)
/// Here, there are four directories: / (the outermost directory), a and d (which are in /), and e
/// (which is in a). These directories also contain files of various sizes.
///
/// Since the disk is full, your first step should probably be to find directories that are good
/// candidates for deletion. To do this, you need to determine the total size of each directory. The
/// total size of a directory is the sum of the sizes of the files it contains, directly or
/// indirectly. (Directories themselves do not count as having any intrinsic size.)
///
/// The total sizes of the directories above can be found as follows:
///
/// The total size of directory e is 584 because it contains a single file i of size 584 and no
/// other directories. The directory a has total size 94853 because it contains files f (size
/// 29116), g (size 2557), and h.lst (size 62596), plus file i indirectly (a contains e which
/// contains i). Directory d has total size 24933642.
/// As the outermost directory, / contains every file. Its total size is 48381165, the sum of the
/// size of every file. To begin, find all of the directories with a total size of at most 100000,
/// then calculate the sum of their total sizes. In the example above, these directories are a and
/// e; the sum of their total sizes is 95437 (94853 + 584). (As in this example, this process can
/// count files more than once!)
///
/// Find all of the directories with a total size of at most 100000. What is the sum of the total
/// sizes of those directories?
///
/// --- Part Two ---
/// Now, you're ready to choose a directory to delete.
///
/// The total disk space available to the filesystem is 70000000. To run the update, you need unused
/// space of at least 30000000. You need to find a directory you can delete that will free up enough
/// space to run the update.
///
/// In the example above, the total size of the outermost directory (and thus the total amount of
/// used space) is 48381165; this means that the size of the unused space must currently be
/// 21618835, which isn't quite the 30000000 required by the update. Therefore, the update still
/// requires a directory with total size of at least 8381165 to be deleted before it can run.
///
/// To achieve this, you have the following options:
///
/// Delete directory e, which would increase unused space by 584.
/// Delete directory a, which would increase unused space by 94853.
/// Delete directory d, which would increase unused space by 24933642.
/// Delete directory /, which would increase unused space by 48381165.
/// Directories e and a are both too small; deleting them would not free up enough space. However,
/// directories d and / are both big enough! Between these, choose the smallest: d, increasing
/// unused space by 24933642.
///
/// Find the smallest directory that, if deleted, would free up enough space on the filesystem to
/// run the update. What is the total size of that directory?
pub fn day_07() {
    let data = load_file(7);

    #[derive(Debug)]
    struct File<'a> {
        #[allow(dead_code)]
        name: &'a str,
        size: usize,
    }

    impl<'a> File<'a> {
        pub fn new(name: &'a str, size: usize) -> Self {
            File { name, size }
        }

        #[allow(dead_code)]
        pub fn name(&self) -> &str {
            self.name
        }

        pub fn size(&self) -> usize {
            self.size
        }
    }

    #[derive(Debug)]
    struct Dir<'a> {
        name: &'a str,
        size: RefCell<Option<usize>>,
        content: Vec<Rc<RefCell<FsElement<'a>>>>,
    }

    impl<'a> Dir<'a> {
        pub fn new(name: &'a str) -> Self {
            Dir {
                name,
                size: RefCell::new(None),
                content: Default::default(),
            }
        }

        pub fn name(&self) -> &str {
            self.name
        }

        pub fn size(&self) -> usize {
            {
                let mut size = self.size.borrow_mut();
                if size.is_none() {
                    let new_size = self
                        .content
                        .iter()
                        .fold(0, |acc, x| acc + x.borrow().size());
                    *size = Some(new_size);
                }
            }

            return self.size.borrow().unwrap();
        }

        pub fn add_element(&mut self, element: Rc<RefCell<FsElement<'a>>>) {
            self.content.push(element);
        }

        pub fn get_dir_by_name(&self, name: &'a str) -> Option<Rc<RefCell<FsElement<'a>>>> {
            // Linear dumb search because fighting with hashing does not appeal to me yet
            for element in self.content.iter() {
                let elt = element.borrow();
                let is_correct_dir = elt.is_dir() && elt.as_dir().name() == name;

                if is_correct_dir {
                    return Some(element.clone());
                }
            }

            None
        }
    }

    #[derive(Debug)]
    enum FsElement<'a> {
        File(File<'a>),
        Dir(Dir<'a>),
    }

    impl<'a> FsElement<'a> {
        pub fn is_dir(&self) -> bool {
            match self {
                FsElement::File(_) => false,
                FsElement::Dir(_) => true,
            }
        }

        pub fn as_dir(&self) -> &Dir<'a> {
            match self {
                FsElement::File(_) => panic!("Not a dir"),
                FsElement::Dir(dir) => dir,
            }
        }

        pub fn as_mut_dir(&mut self) -> &mut Dir<'a> {
            match self {
                FsElement::File(_) => panic!("Not a dir"),
                FsElement::Dir(dir) => dir,
            }
        }

        pub fn size(&self) -> usize {
            match self {
                FsElement::File(file) => file.size(),
                FsElement::Dir(dir) => dir.size(),
            }
        }
    }

    let root_dir = Rc::new(RefCell::new(FsElement::Dir(Dir::new("/"))));
    let mut all_dirs = Vec::<Rc<RefCell<FsElement>>>::new();
    all_dirs.push(root_dir.clone());

    let mut location = Vec::<Rc<RefCell<FsElement>>>::new();

    let mut lines: std::collections::VecDeque<_> = data.split('\n').into_iter().collect();

    while !lines.is_empty() {
        let line = lines.pop_front().unwrap();
        if line.starts_with('$') {
            let (_, cmd) = line.split_once(' ').unwrap();
            let mut cmd_with_args = cmd.split_whitespace();

            let cmd = cmd_with_args.next().unwrap();

            match cmd {
                "cd" => {
                    let dest_dir = cmd_with_args.next().unwrap();
                    if dest_dir != "/" {
                        if dest_dir == ".." {
                            location.pop();
                            continue;
                        }
                        let curr_dir = location.last().unwrap().clone();
                        let dest_dir = curr_dir
                            .borrow()
                            .as_dir()
                            .get_dir_by_name(dest_dir)
                            .unwrap();
                        location.push(dest_dir);
                    } else {
                        location.clear();
                        location.push(root_dir.clone());
                    }
                }
                "ls" => {
                    let mut curr_dir_rc = location.last().unwrap().borrow_mut();
                    let curr_dir = curr_dir_rc.as_mut_dir();
                    while !lines.is_empty() && !lines.front().unwrap().starts_with('$') {
                        let line = lines.pop_front().unwrap();
                        if let Some((dir_or_size, name)) = line.trim().split_once(' ') {
                            let new_fs_element = match dir_or_size {
                                "dir" => {
                                    let new_dir =
                                        Rc::new(RefCell::new(FsElement::Dir(Dir::new(name))));
                                    all_dirs.push(new_dir.clone());
                                    new_dir
                                }
                                size => {
                                    let size: usize = size.parse().unwrap();
                                    Rc::new(RefCell::new(FsElement::File(File::new(name, size))))
                                }
                            };
                            curr_dir.add_element(new_fs_element);
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    assert!(all_dirs.iter().all(|x| x.borrow().is_dir()));

    let sum_size_dirs_under_100_000: usize = all_dirs
        .iter()
        .filter(|&x| x.borrow().size() <= 100_000)
        .fold(0, |acc, x| acc + x.borrow().size());

    println!("Part 1: {sum_size_dirs_under_100_000}");

    const TOTAL_SPACE: usize = 70_000_000;
    const REQ_SPACE: usize = 30_000_000;
    let remaining_space = TOTAL_SPACE - root_dir.borrow().size();
    let space_to_free = REQ_SPACE - remaining_space;

    let dirs_freeing_enough_space: Vec<_> = all_dirs
        .iter()
        .filter(|&x| x.borrow().size() >= space_to_free)
        .collect();

    let mut min_dir = root_dir;
    for dir in dirs_freeing_enough_space {
        if dir.borrow().size() < min_dir.borrow().size() {
            min_dir = dir.clone();
        }
    }

    println!("Part 2: {}", min_dir.borrow().size());
}
