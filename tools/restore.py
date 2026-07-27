from json import dump, load
from shutil import copy

hopper = lambda _: True

def main():
    with open("state.json") as f:
        data = load(f)

    print("DATA RESTORATION & BACKUP TOOL")

    move: int = int(input("""A wrong step could mean disaster. Be careful!
    1) Save backup
    2) Load backup
    3) Add entry
    4) Delete entry
    5) Exit
""")) - 1

    print("Success" if [
        lambda: hopper(copy("state.json", "state~.json")),
        lambda: hopper(copy("state~.json", "state.json")),
        lambda: hopper(data.update({input("Entry name: "): {
            "bool": False,
            "HashMap": {},
            "int": 0,
            "String": "",
        }[input("Entry type: ")]})),
        lambda: hopper(data.pop(input("Entry name: "))),
        lambda: True,
    ][move]() else "Failure")

    if move > 1:
        with open("state.json", "w") as f:
            dump(data, f, separators=(",",":"))

if __name__ == "__main__":
    main()
