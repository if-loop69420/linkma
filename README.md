# Linkma
Linkma is a config management system, that uses symlinks to place configs where they need to go.

# What is Linkma?
Linkma config files, admin!

# TODOs
- [X] Define config format
- [X] Read Config correctly
- [ ] Check if output_path is passed and if it exists. If it is not passed set it to the default path (/linkma/) and create it
- [ ] Make a new folder in the output_path with the current timestamp
- [ ] Output all files, that need to go there with an identifier so we know where to link them (UUID or something)
- [ ] Once all files have been written add an init script to the folder, that links all the files (with their identifiers) to the correct paths
- [ ] Set the folder to Read-only
- [ ] Link the folder to something like /linkma/current_system
- [ ] (Add /linkma/current_system/init to the startup somehow (init system agnostic))
