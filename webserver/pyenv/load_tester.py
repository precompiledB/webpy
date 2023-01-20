from sys import argv

def read_file(file_path) -> str:
    file = file_path
    with open(file, "r", encoding="utf-8") as user_code:
        payload = user_code.read()
        print('Success, reading:\n***\n', payload, '\n***')
        return payload

def create_globals() -> dict:
    # copy builtins so to not overwrite the current global __builtins__
    from copy import copy
    blt = copy(__builtins__.__dict__)

    # overwrite the print func
    def custom_print(*val):
        print('CONSOLE>', *val)

    blt['print'] = custom_print

    # TODO: overwrite __import__

    return {
        '__name__': '__exec__',
        '__builtins__': blt,
    }


def var_exists(locals, var_name):
    return True if var_name in locals else False

def var_checkval(locals, var_name, expected):
    return True if var_exists(locals, var_name) and locals[var_name] == expected else False

def var_checktype(locals, var_name, *types):
    if var_exists(locals, var_name):
        is_types = [isinstance(locals[var_name], t) for t in types]
        return any(is_types)
    else:
        return False

def main() -> None:
    user_code = read_file(file_path=argv[1])
    gbl = create_globals()
    lcl = {}
    exec(user_code, gbl, lcl)


    # --- testing code

    if 'a' in lcl:
        print('Bravo!')

    print(lcl)

    lcl['hello']()

    print(var_checktype(lcl, "a", bool, dict, float, int))

if __name__ == "__main__":
    main()
