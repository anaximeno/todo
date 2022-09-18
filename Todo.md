# To Do CLI

## Use cases:
1. [ ] add
	- [ ] `todo add <todo name> -d <description>`
	- [ ] `todo task add <id> -t <task>`
	- [ ] `todo task add <id>` -> get task as input
2. [ ] list
	- [ ] `todo list` -> todos and tasks
	- [ ] `todo list <id>` -> tasks on todo
	- [ ] `todo list <id> --done` -> tasks done
	- [ ] `todo list --done` -> todos done
3. [ ] done
	- [ ] `todo set done -i <id>` -> sets all tasks as done
	- [ ] `todo set done -i <id> -t <id>` -> set task as done
4. [ ] drop
	- [ ] `todo drop -i `
	- [ ] `todo <id> task <id> drop`
5. [ ] update
	- [ ] `todo <id> set name <name>`
	- [ ] `todo <id> set description <description>`
	- [ ] `todo <id> task <id> set to <task>`