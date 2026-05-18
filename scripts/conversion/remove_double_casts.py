import os, glob, re

def main():
    double_cast_pattern = re.compile(r"as\s+[a-zA-Z0-9_:]+\s+as\s+([a-zA-Z0-9_:]+)")
    count = 0
    
    for filepath in glob.glob("src/**/*.rs", recursive=True):
        with open(filepath, "r") as f:
            content = f.read()
        
        orig = content
        
        # Replace instances of `as TypeA as TypeB` with `as TypeB`
        content = double_cast_pattern.sub(r"as \1", content)
        
        if orig != content:
            with open(filepath, "w") as f:
                f.write(content)
            count += 1
            
    print(f"Removed double casts from {count} files.")

if __name__ == "__main__":
    main()