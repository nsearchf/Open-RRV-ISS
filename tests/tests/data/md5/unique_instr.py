#!/usr/bin/env python3

def process_file(input_file, output_file):
    # Set to store unique values encountered in the third column
    seen_third_columns = set()
    # List to store lines that should be written to the output file
    lines_to_keep = []

    with open(input_file, 'r') as file:
        for line in file:
            # Strip newline characters and split the line
            parts = line.strip().split()

            if len(parts) < 3:
                # If the line has fewer than three columns, skip it
                # lines_to_keep.append(line)
                continue

            # Get the value of the third column
            third_column = parts[2]
            # Skip if the third column is not alphabetic
            if not third_column.isalpha():
                continue

            if third_column not in seen_third_columns:
                # If the value in the third column hasn't been seen before, add it to the set and keep the line
                seen_third_columns.add(third_column)
                lines_to_keep.append(line)

    with open(output_file, 'w') as file:
        for line in lines_to_keep:
            file.write(line)


input_file = 'md5_bare.asm'
output_file = 'md5_bare_unique.asm'

process_file(input_file, output_file)
