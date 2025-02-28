# Extract the first argument after 'run-local'
ARGV := $(word 2, $(MAKECMDGOALS))

# Count total arguments (excluding 'run-local')
ARGC := $(words $(MAKECMDGOALS))

# Enforce exactly one argument
ifeq ($(ARGC),1)
  $(error Please provide folder path. Usage: make run-local <folder_path>)
endif
ifneq ($(ARGC),2)
  $(error Too many arguments. Usage: make run-local <folder_path>)
endif

run-local:
	LOCAL=1 cargo run $(ARGV)