__author__ = "Alexander Velez"
__license__ = "MIT"
__version__ = "0.2.0"
__maintainer__ = "Alexander Velez"
__email__ = "alexandervelez7245@gmail.com"

import sys
import struct

def num_to_bits(args: list[str]):
    if len(args) != 2:
        std_usage(["usage", args[0]])
        sys.exit()
    num = int(args[1])
    width = 64
    raw_tunc = bin(num)[2:][::-1]
    raw_tunc += "0" * (width - len(raw_tunc))
    # print(raw_tunc)
    s_raw = ']['.join(raw_tunc[i:i+8] for i in range(0, len(raw_tunc), 8))
    # print('[' + s_raw + ']')
    bits = s_raw.split("][")
    b_bits = [str(int(bits[i][::-1], 2)) for i in range(len(bits))]
    print(" ".join(b_bits))
    
# stc <>
def str_to_cells(args: list[str]):
    if len(args) != 2:
        std_usage(["usage", args[0]])
        sys.exit()
    command, string = args[0], args[1]
    [print(f"[{ord(ch)}]", end="") for ch in string]
    print()

# fc <>
def func_code(args: list[str]):
    if len(args) != 2:
        std_usage(["usage", args[0]])
        sys.exit()
    command, ordinal = args[0], int(args[1])
    zordinal = ordinal - 1
    match zordinal // 256:
        case 0:print("+- :", zordinal % 256)
        case 1:print("-+ :", zordinal % 256)
        case 2:print("<> :", zordinal % 256)
        case 3:print(">< :", zordinal % 256)
        case _:print("NULL")

# mini <> <>
def mini(args: list[str]):
    if len(args) != 3:
        std_usage(["usage", args[0]])
        sys.exit()
    command, rbf_file, new_file = args
    f = open(rbf_file, "r")
    file = f.read()
    f.close()
    cleaned_source = ""
    sl_comment = False
    ml_comment = True
    for ch in file:
        match ch:
            case ';':
                sl_comment = True
            case '{':
                ml_comment = True
            case '}':
                ml_comment = False
            case '|' | ' ' | '\n':
                if ch == '\n':
                    sl_comment = False
                if not (sl_comment or ml_comment) and len(cleaned_source) > 0 and cleaned_source[-1] != '|':
                    cleaned_source += '|'
            case '<' | '>' | '+' | '-' | ',' | '.' | '[' | ']' | '?' | '#' | '$' | '!':
                if not (sl_comment or ml_comment):
                    cleaned_source += ch
    cleaned_source = cleaned_source.replace("+|-", "+ -")
    cleaned_source = cleaned_source.replace("-|+", "- +")
    cleaned_source = cleaned_source.replace("<|>", "< >")
    cleaned_source = cleaned_source.replace(">|<", "> <")
    cleaned_source = cleaned_source.replace("|+-|", " +- ")
    cleaned_source = cleaned_source.replace("|-+|", " -+ ")
    cleaned_source = cleaned_source.replace("|<>|", " <> ")
    cleaned_source = cleaned_source.replace("|><|", " >< ")
    cleaned_source = cleaned_source.replace("|", "")
    ccs = ""
    line_len = 0
    for ch in cleaned_source:
        ccs += ch
        line_len += 1
        if line_len >= 98:
            ccs += '\n'
            line_len = 0
    new = open(new_file, "w+")
    new.write(ccs)
    new.close()

def largest_factor_pair(number: int) -> tuple[int, int]:
    max_factor = int( number**0.5) + 1
    largest_pair = (1, number)
    for i in range(2, max_factor):
        if number % i == 0:
            factor_pair = (i, number // i)
            largest_pair = max(largest_pair, factor_pair)
    return largest_pair

def generate_bf_num(num: int, op: str = "+") -> str:
    lfp = largest_factor_pair(num)
    if lfp[0] == 1:return f"{op*lfp[1]}>"
    else:return f">{"+"*lfp[0]}[-<{op*lfp[1]}>]"

# bfsg <>
def bf_str_gen(args: list[str]):
    if len(args) != 2:
        std_usage(["usage", args[0]])
        sys.exit()
    command, string = args
    for ch in string:
        code_list = []
        pair = largest_factor_pair(ord(ch))
        code_list.append(f">{"+"*pair[0]}[<{"+"*pair[1]}>-]")
        for i in range(1, 10+1):
            after_pair = largest_factor_pair(ord(ch)+i)
            code_list.append(f">{"+"*after_pair[0]}[<{"+"*after_pair[1]}>-]<{"-"*i}>")
        for i in range(1, 10+1):
            before_pair = largest_factor_pair(ord(ch)-i)
            code_list.append(f">{"+"*before_pair[0]}[<{"+"*before_pair[1]}>-]<{"+"*i}>")
        print(min(code_list, key=len))

# bfng <> <>
def bf_num_gen(args: list[str]):
    if len(args) != 3:
        std_usage(["usage", args[0]])
        sys.exit()
    command, start, end = args[0], int(args[1]), int(args[2])
    diff_inc = (end - start) % 256
    diff_dec = (start - end) % 256
    distance = min(diff_inc, diff_dec)
    if (start - distance) % 256 == end:print(f"{start} -> {end} : (-{distance}) :", generate_bf_num(distance, "-"))
    elif (start + distance) % 256 == end:print(f"{start} -> {end} : (+{distance}) :",generate_bf_num(distance, "+"))
    else:print("NULL")

def b10_to_b256(num: int, base: int, width: int = 4) -> list[int]:
    rems = []
    while num != 0:
        rems.append(num % base) 
        num //= base
    return [rems[i] if i < len(rems) else 0 for i in range(width)]

# cti <> <> <> <>
def cells_to_i32(args: list[str]):
    if len(args) != 5:
        std_usage(["usage", args[0]])
        sys.exit()
    command, cells = args[0], [int(x) for x in  args[1]]
    print("cells:", cells)
    print("u32:", sum([x*(256**i) for (i, x) in enumerate(cells)]))

# itc <>
def i32_to_cells(args: list[str]):
    if len(args) != 2:
        std_usage(["usage", args[0]])
        sys.exit()
    command, i32 = args[0], int(args[1])
    IMAX, IMIN = (((256 ** 4) / 2)-1), -((256 ** 4) / 2)
    if i32 > IMAX or i32 < IMIN:
        print("NULL")
        sys.exit()
    sub = b10_to_b256(abs(i32), 256, 4)
    if i32 < 0:
        cells = [x - sub[i] for (i, x) in enumerate([255 for x in range(4)])]
        cells[0] += 1
        print("cells:", cells)
    else:
        print("cells:", sub)

# ctu <> <> <> <>
def cells_to_u32(args: list[str]):
    if len(args) != 5:
        std_usage(["usage", args[0]])
        sys.exit()
    command, cells = args[0], [int(x) for x in args[1:]]
    print("cells:", cells)
    print("u32:", sum([x*(256**i) for (i, x) in enumerate(cells)]))

# utc <>
def u32_to_cells(args: list[str]):
    if len(args) != 2:
        std_usage(["usage", args[0]])
        sys.exit()
    command, u32 = args[0], int(args[1])
    UMAX, UMIN = ((256**4)-1), 0
    if u32 < UMIN or u32 > UMAX:
        print("NULL")
        sys.exit()
    cells = b10_to_b256(u32, 256, 4)
    print("cells:", cells)

def binary(num) -> str:
  return ''.join('{:0>8b}'.format(c) for c in struct.pack('!f', num))

# ctf <> <> <> <>
def cells_to_f32(args: list[str]):
    if len(args) != 5:
        std_usage(["usage", args[0]])
        sys.exit()
    command, cells = args[0], [int(x) for x in args[1:]]
    print("cells:", cells)
    bytes = ['{0:08b}'.format(x) for x in cells]
    print("bytes:", bytes)
    bits = ''.join(bytes)
    print("bits:", bits)
    s = bits[0]
    sign = (-1)**int(s)
    print("S:", s, "=>", ("+" if sign > 0 else "") + str(sign))
    e = bits[1:9]
    exp = int(e, 2)
    print("E:", e, "=>", exp)
    m = bits[9:]
    mantissa = sum([int(x)*(2**(-i-1)) for (i, x) in enumerate(m)])
    print("M:", m, "=>", mantissa)
    f32 = sign * (1 + mantissa) * (2**(exp - 127))
    print("f32:", f32)

#ftc <>
def f32_to_cells(args: list[str]):
    if len(args) != 2:
        std_usage(["usage", args[0]])
        sys.exit()
    command, f32 = args[0], float(args[1])
    print("f32:", f32)
    bits = binary(f32)
    print("bits:", bits)
    bytes = [bits[i:i+8] for i in range(0, len(bits), 8)]
    print("bytes:", bytes)
    cells = [int(x, 2) for x in bytes]
    print("cells:", cells)

COMMANDS = {
    "help (h)" : ["Show usage for bf_tools.py", "help ~<command>"],
    "commands (c)" : ["List commands", "commands"],
    "usage (u)" : ["Show usage for specified command", "usage <command>"],
    "ctf" : ["Convert cells to f32", "ctf <byte> <byte> <byte> <byte>"],
    "ftc" : ["Convert f32 to cells", "ftc <f32>"],
    "ctu" : ["Convert cells to u32", "ctu <byte> <byte> <byte> <byte>"],
    "utc" : ["Convert u32 to cells", "utc <u32>"],
    "cti" : ["Convert cells to i32", "itf <byte> <byte> <byte> <byte>"],
    "itc" : ["Convert i32 to cells", "itc <i32>"],
    "bfng" : ["Generate BF code to add up to a number", "bfng <start> <end>"],
    "bfsg" : ["Generate BF code for given text", "bfsg <text>"],
    "mini" : ["Minify BF Code", "mini <input_file> <output_file>"],
    "fc" : ["Converts ordinal of Raylib function to its flip and id equivalent", "fc <ordinal>"],
    "stc" : ["Converts text to cells", "stc <text>"],
    "ntb": ["Converts decimal to 64-bit arch little endian bits"]
}

# help
# help <>
def std_help():
    match len(args):
        case 1: print("Brainfuck Tools:\n~ is optional\nUSAGE: bf_tools.py <command> ~<arg1> ~<arg2> ...")
        case 2: std_usage([args[0], args[1]])
        case _: std_usage(["usage", "help (h)"])
    
# commands
def std_command(args: list[str]):
    if len(args) != 1:
        std_usage(["usage", "commands (c)"])
        sys.exit()
    for command in COMMANDS:
        print("{0:12} : {1}".format(command, COMMANDS[command][0]))

# usage <>
def std_usage(args: list[str]):
    if len(args) != 2:
        print("USAGE: bf_tools.py usage <command>")
        sys.exit()
    if args[1] in COMMANDS: print("USAGE: bf_tools.py", COMMANDS[args[1]][1])
    else: print(f"[{args[0]}]: '{args[1]}' not a command")

if __name__ == "__main__":
    args = sys.argv[1:]
    
    if len(args) < 1:
        print("USAGE: bf_tools.py <command> <arg1> <arg2> ...")
        sys.exit()
    
    match args[0]:
        case "help" | "h": std_help()
        case "commands" | "c": std_command(args)
        case "usage" | "u": std_usage(args)
        case "ctf": cells_to_f32(args)
        case "ftc": f32_to_cells(args)
        case "utc": u32_to_cells(args)
        case "ctu": cells_to_u32(args)
        case "itc": i32_to_cells(args)
        case "cti": cells_to_i32(args)
        case "bfng": bf_num_gen(args)
        case "bfsg": bf_str_gen(args)
        case "mini": mini(args)
        case "fc": func_code(args)
        case "stc": str_to_cells(args)
        case "ntb": num_to_bits(args)
        case _: print(f"'{args[0]}' not a command")