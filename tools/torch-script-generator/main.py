import torch


@torch.jit.script
def add(x, y):
    return x + y


@torch.jit.script
def sub(x, y):
    return x - y


@torch.jit.script
def mul(x, y):
    return x * y


@torch.jit.script
def div(x, y):
    return x / y


with open('../../example/torchscript/add', 'w') as f:
    f.write(add.code)
with open('../../example/torchscript/sub', 'w') as f:
    f.write(sub.code)
with open('../../example/torchscript/mul', 'w') as f:
    f.write(mul.code)
with open('../../example/torchscript/div', 'w') as f:
    f.write(div.code)
