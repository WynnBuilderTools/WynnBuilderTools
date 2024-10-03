import json
import sys
from collections import defaultdict


def build_attribute_tree(data):
    tree = defaultdict(set)
    for _, attributes in data.items():
        build_tree_recursive(attributes, tree, [])
    return {k: list(v) for k, v in tree.items()}


def build_tree_recursive(obj, tree, path):
    if isinstance(obj, dict):
        for key, value in obj.items():
            new_path = path + [key]
            tree[".".join(new_path)] = set()
            build_tree_recursive(value, tree, new_path)
    elif isinstance(obj, list):
        for item in obj:
            build_tree_recursive(item, tree, path)
    else:
        tree[".".join(path)].add(type(obj).__name__)


def main():
    if len(sys.argv) != 2:
        print("Usage: python script.py <input_json_file>")
        sys.exit(1)

    input_file = sys.argv[1]

    try:
        with open(input_file, "r") as f:
            data = json.load(f)
    except FileNotFoundError:
        print(f"Error: File '{input_file}' not found.")
        sys.exit(1)
    except json.JSONDecodeError:
        print(f"Error: Invalid JSON in file '{input_file}'.")
        sys.exit(1)

    tree = build_attribute_tree(data)
    output = json.dumps(tree, indent=2)
    print(output)


if __name__ == "__main__":
    main()
