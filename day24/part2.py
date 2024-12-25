def find_changed_lines(old_lines, new_lines):
    """
    Finds lines in the new file that have been changed from the old file.

    Parameters:
        old_lines (list of str): List of lines from the old file.
        new_lines (list of str): List of lines from the new file.

    Returns:
        list of tuple: A list of tuples where each tuple contains the line number (1-based) and the changed line.
    """
    changed_lines = []
    
    # Iterate through the lines, up to the length of the shorter list
    for i, (old_line, new_line) in enumerate(zip(old_lines, new_lines), start=1):
        if old_line != new_line:
            changed_lines.append((i, new_line))
    
    # Return the changed lines with their line numbers
    return changed_lines

def main() -> None:
    ls = find_changed_lines(open("src/input.txt").read().splitlines(), open("src/fixed_input.txt").read().splitlines())
    ls = [l.split(" -> ")[1] for _, l in ls]
    print(",".join(sorted(ls)))


if __name__ == "__main__":
    main()
