import os, glob, re

def find_matching_brace(text, start_idx):
    depth = 1
    for i in range(start_idx, len(text)):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                return i
    return -1

def main():
    count = 0
    for filepath in glob.glob("src/**/*.rs", recursive=True):
        with open(filepath, "r") as f:
            content = f.read()
        
        orig = content
        
        # Match `unsafe fn ... {`
        pattern = re.compile(r"unsafe\s+fn\s+[^{]+\{")
        
        pos = 0
        while True:
            m = pattern.search(content, pos)
            if not m:
                break
            
            fn_start = m.start()
            brace_open = m.end() - 1
            
            brace_close = find_matching_brace(content, brace_open + 1)
            if brace_close == -1:
                pos = brace_open + 1
                continue
            
            fn_body = content[brace_open+1:brace_close]
            
            # Check if fn_body consists ONLY of an unsafe block
            body_match = re.fullmatch(r"\s*unsafe\s*\{([\s\S]*)\}\s*", fn_body)
            
            if body_match:
                inner_content = body_match.group(1)
                depth = 0
                valid = True
                for char in inner_content:
                    if char == "{": depth += 1
                    elif char == "}":
                        depth -= 1
                        if depth < 0:
                            valid = False
                            break
                
                if valid:
                    unsafe_line_match = re.search(r"^(\s*)unsafe\s*\{", fn_body, re.MULTILINE)
                    if unsafe_line_match:
                        indent = unsafe_line_match.group(1)
                        lines = inner_content.split("\n")
                        new_lines = []
                        for line in lines:
                            if line.strip() == "":
                                new_lines.append("")
                            elif line.startswith(indent + "    "):
                                new_lines.append(line[len(indent)+4:])
                            else:
                                new_lines.append(line)
                        inner_content = "\n".join(new_lines)
                    
                    content = content[:brace_open+1] + inner_content + content[brace_close:]
                    count += 1
                    pos = 0
                    continue

            pos = brace_open + 1
            
        if orig != content:
            with open(filepath, "w") as f:
                f.write(content)
            
    print(f"Cleaned up redundant unsafe blocks in {count} functions.")

if __name__ == "__main__":
    main()