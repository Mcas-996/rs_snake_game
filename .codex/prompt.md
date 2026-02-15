You are in VS Code on Windows.
NEVER use powershell, Get-Content, type, cat, or any shell command to read files.
ALWAYS use your built-in file read tool / internal filesystem API to read file contents.
Do NOT execute any shell commands just to read or search files — that requires unnecessary approval.
If you need file content, read it directly without shell.
Ask the user to execute for things that truly need external commands (like git, npm install, etc.).