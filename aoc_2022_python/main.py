import subprocess
import pathlib
import os

def run_all():
    for fpath in pathlib.Path(__file__).parent.rglob(r"*day[0-9]*.py"):
        subprocess.call(["python3", fpath.resolve().as_posix()], env=os.environ)


if __name__ == "__main__":
    run_all()
