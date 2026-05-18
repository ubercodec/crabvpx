import os, glob, re

def find_closing_paren(text, start_idx):
    depth = 0
    for i in range(start_idx, len(text)):
        if text[i] == '(':
            depth += 1
        elif text[i] == ')':
            depth -= 1
            if depth == 0:
                return i
    return -1

def parse_args(call_body):
    args = []
    current_arg = []
    depth = 0
    for char in call_body:
        # Ignore < and > since they are used for shifts in Rust/C
        if char in "([{": depth += 1
        elif char in ")]}": depth -= 1
        
        if char == "," and depth == 0:
            args.append("".join(current_arg).strip())
            current_arg = []
        else:
            current_arg.append(char)
    if current_arg:
        args.append("".join(current_arg).strip())
    return args

def main():
    count_memcpy = 0
    count_memset = 0
    
    for filepath in glob.glob("src/**/*.rs", recursive=True):
        with open(filepath, "r") as f:
            content = f.read()
        
        orig = content
        
        # We need a way to advance pos if parse fails instead of breaking
        pos_memcpy = 0
        while True:
            m = re.search(r"memcpy\s*\(", content[pos_memcpy:])
            if not m: break
            
            idx = pos_memcpy + m.start()
            start_paren = pos_memcpy + m.end() - 1
            
            end_idx = find_closing_paren(content, start_paren)
            if end_idx == -1: 
                pos_memcpy = idx + 6
                continue
            
            call_body = content[start_paren+1:end_idx]
            args = parse_args(call_body)
            if len(args) >= 3:
                replacement = f"core::ptr::copy_nonoverlapping(({args[1]}) as *const u8, ({args[0]}) as *mut u8, ({args[2]}) as usize)"
                content = content[:idx] + replacement + content[end_idx+1:]
                pos_memcpy = idx + len(replacement)
                count_memcpy += 1
            else:
                pos_memcpy = end_idx + 1
                
        pos_memset = 0
        while True:
            m = re.search(r"memset\s*\(", content[pos_memset:])
            if not m: break
            
            idx = pos_memset + m.start()
            start_paren = pos_memset + m.end() - 1
            
            end_idx = find_closing_paren(content, start_paren)
            if end_idx == -1: 
                pos_memset = idx + 6
                continue
            
            call_body = content[start_paren+1:end_idx]
            args = parse_args(call_body)
            if len(args) >= 3:
                replacement = f"core::ptr::write_bytes(({args[0]}) as *mut u8, ({args[1]}) as u8, ({args[2]}) as usize)"
                content = content[:idx] + replacement + content[end_idx+1:]
                pos_memset = idx + len(replacement)
                count_memset += 1
            else:
                pos_memset = end_idx + 1
        
        if orig != content:
            with open(filepath, "w") as f:
                f.write(content)
            
    print(f"Replaced {count_memcpy} memcpy and {count_memset} memset calls.")

if __name__ == "__main__":
    main()