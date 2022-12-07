from aoc import input_fpath_for_day


class FileType:
    def __init__(self, is_file: bool):
        self.__is_file = is_file

    def is_file(self):
        return self.__is_file

    def is_dir(self):
        return not self.is_file()


class File(FileType):
    def __init__(self, name: str, size: str):
        super().__init__(is_file=True)
        self.name = name
        self.size = size


class Dir(FileType):
    def __init__(self, name: str):
        super().__init__(is_file=False)
        self.name = name
        self.files = dict()

    def dir(self, dirname: str):
        return self.files[dirname]

    def mkdir(self, dirname: str):
        self.files[dirname] = dict()

    def touch(self, fname: str, size: int):
        self.files[fname] = File(fname, size)


class FileSystem:
    root = "/"

    def __init__(self):
        self.root = Dir(self.root)
        self.cwd = self.root

    def cd(self, dirname):
        self.cwd = self.cwd.dir(dirname)

    def mkdir(self, dirname):
        self.cwd.mkdir(dirname)

    def touch(self, fname: str, size: int):
        self.cwd.touch(fname, size)


def part_1(puzzle_input):
    pass


def part_2(puzzle_input):
    pass


if __name__ == "__main__":
    with input_fpath_for_day(7).open() as f:
        content = f.read()
        content = """\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"""
        print("Day 7")
        print("Part 1:", part_1(content))
        print("Part 2:", part_2(content))
