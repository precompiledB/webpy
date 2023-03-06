import inspect


class CheckResult:
    success = "Success"
    v_not_found = "VarNotFound"
    v_wrong_val = "VarWrongValue"
    v_wrong_type = "VarWrongType"
    v_not_fun = "VarNotAFunction"
    f_blackbox_fail = "FunBlackBoxFailed"
    c_not_class = "NotAClass"
    c_no_attrib = "ClsNotAnAttr"


class VarTester:
    def __init__(self, item):
        self.var = item

    def var_from(self, name):
        if hasattr(self.var, '__dict__') and \
                name in self.var:
            return VarTester(self.var[name])
        else:
            return VarTester(None)

    def check_value(self, val) -> str:
        if self.var is None:
            return CheckResult.v_not_found

        if self.var == val:
            return CheckResult.success
        else:
            return CheckResult.v_wrong_val

    def check_type(self, *types) -> str:
        if self.var is None:
            return CheckResult.v_not_found

        is_types = [isinstance(self.var, t) for t in types]
        if any(is_types):
            return CheckResult.success
        else:
            return CheckResult.v_wrong_type

    def fun_black_box_test(self, fun_in: list, fun_out) -> str:
        if self.var is None:
            return CheckResult.v_not_found

        if not callable(self.var):
            return CheckResult.v_not_fun
        elif self.var(*fun_in) != fun_out:
            return CheckResult.f_blackbox_fail
        else:
            return CheckResult.success


class ClassTester:
    def __init__(self, item):
        self.cls = item

    def has_attribute(self, name) -> str:
        if self.cls is None:
            return CheckResult.c_not_class

        if name in self.cls.__dict__:
            return CheckResult.success
        else:
            return CheckResult.c_no_attrib

    def var_from(self, name) -> VarTester:
        if name in self.cls.__dict__:
            return VarTester(self.cls[name])
        else:
            return VarTester(None)

    # TODO: Attribute can be in class and object scope

class Tester:
    def __init__(self, lcl: dict):
        self.input = lcl

    def var_from(self, name: str) -> VarTester:
        if name in self.input:
            return VarTester(self.input[name])
        else:
            return VarTester(None)

    def cls_from(self, name: str) -> ClassTester:
        if name in self.input and \
                (inspect.isclass(cls := self.input[name])):
            return ClassTester(cls)
        else:
            return ClassTester(None)


def is_successful(executed) -> bool:
    return executed == CheckResult.success

