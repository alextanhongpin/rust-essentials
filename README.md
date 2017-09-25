# Exclude all binary
find . -type f -perm -111 >> .gitignore  # or: find . -type f -perm -a=x