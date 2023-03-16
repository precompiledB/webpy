from sys import argv
import tester

def read_file(file_path) -> str:
    file = file_path
    with open(file, "r", encoding="utf-8") as user_code:
        payload = user_code.read()
        #print('Success, reading:\n***\n', payload, '\n***')
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


def fnc_checkval(locals, fnc_name, expected_output):
    return True if var_exists(locals, fnc_name) and locals[fnc_name]() == expected_output else False


def cls_checkattrib(locals, cls_name, attrib):
    if var_exists(locals, cls_name):
        return attrib in locals[cls_name].__dict__
    return False


def cls_checkattribval(locals, cls_name, attrib, val):
    if cls_checkattrib(locals, cls_name, attrib):
        return locals[cls_name].__dict__[attrib] == val
    else:
        return False


def a1a(lcl):
    if "greeting" in lcl:
        if lcl["greeting"].upper() == "HELLO, WORLD!":
            return True
    else:
        return False


def a1b(lcl):
    if "awake" in lcl:
        if lcl["awake"] == True:
            return True
    else:
        return False


def a1c(lcl):
    return var_checktype(lcl, "caffine", float)


def a2b(lcl):
    t = tester.Tester(lcl)
    a_da = t.var_from('a').check_type(object)
    b_da = t.var_from('b').check_type(object)

    is_da = lambda x: True if x is "Success" else False
    a_da = is_da(a_da)
    b_da = is_da(b_da)

    print(f"\nIs a existent? {a_da}\nIs b existent? {b_da}\n")

    return a_da and b_da

def main() -> None:
    user_code = read_file(file_path=argv[1])
    gbl = create_globals()
    lcl = {}
    exec(user_code, gbl, lcl)

    # --- testing code
    #print("#########################################")
    test_dict = {
        (0, 0): "intro",
        (1, 0): a1a,
        (1, 1): a1b,
        (1, 2): a1c,
        (2, 0): a1a,
        (2, 1): a2b,
    }

    current = tuple(map(int, argv[2].split("_")))
    print("Did the test pass?", test_dict[current](lcl))

    #print("#########################################")

    # if 'a' in lcl:
    #     print('Bravo!')

    # print(lcl)

    # print("fnc?", fnc_checkval(lcl, "hello", False))

    # print("checkval?", var_checktype(lcl, "a", bool, dict, float, int))

    # print("attrib?", cls_checkattrib(lcl, "A", "a"))
    # print("attrib_val?", cls_checkattribval(lcl, 'A', 'b', "test"))


if __name__ == "__main__":
    main()
