#
# python3 gen_instr_entry.py ../execute/rv_i.rs > tmp.txt
# python3 gen_instr_entry.py ../execute/rv_system.rs >> tmp.txt
# python3 gen_instr_entry.py ../execute/rv_zicsr.rs >> tmp.txt
# python3 gen_instr_entry.py ../execute/rv32_i.rs >> tmp.txt
#
import sys
import os
import re


def find_and_transform(file_path, module_name):
    # Define the pattern to search for
    pattern = r'pub\(crate\) fn execute_(\w+)'

    # Initialize an empty list to store the transformed strings
    results = []
    # print(file_path, module_name)

    # Open the file and read each line
    with open(file_path, 'r') as file:
        for line in file:
            # Search for the pattern in the current line
            match = re.search(pattern, line)

            # If a match is found, process it
            if match:
                # Extract the function name from the match
                func_name = match.group(1).upper()

                # Generate the new string
                new_line = (
                    f'    InstructionsEntry {{\n'
                    f'        name: "{func_name}",\n'
                    f'        mask: MASK_{func_name},\n'
                    f'        match_val: MATCH_{func_name},\n'
                    f'        execute: {module_name}::execute_{
                        func_name.lower()},\n'
                    f'    }},\n'
                )

                # Append the new string to the results list
                results.append(new_line)

    # Return the list of transformed strings
    return results


# Check if a file path is provided as a command-line argument
if len(sys.argv) != 2:
    print("Usage: python script.py <file_path>")
    sys.exit(1)

# Get the file path from the command-line argument
file_path = sys.argv[1]

# Extract the module name from the file path
module_name = os.path.splitext(os.path.basename(file_path))[0]

# print(module_name)

# Call the function and print the results
results = find_and_transform(file_path, module_name)
for result in results:
    print(result)
