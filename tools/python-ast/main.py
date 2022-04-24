import ast
import os

EXAMPLE_PATH = 'example'
TARGET_PATH = 'target'

if not os.path.exists(TARGET_PATH):
    os.makedirs(TARGET_PATH)

files = os.listdir(EXAMPLE_PATH)

for file in files:
    input_name = os.path.join(EXAMPLE_PATH, file)
    output_name = os.path.join(TARGET_PATH, file.replace('.py', '.ast'))
    with open(input_name) as input:
        code = input.read()
        with open(output_name, 'w') as output:
            output.write(ast.dump(ast.parse(code)))
    print('compile {} to {} is finished.'.format(input_name, output_name))
