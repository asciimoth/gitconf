# Gitconf
Overlay utility for more convenient and flexible configuration of git

# Description
Git allows you to set local and global settings, but the management of these settings leaves much to be desired.   
Gitconf tool works as an overlay that adds a configuration profile mechanism to git.
With gitconf, you can create multiple git settings profiles in the system and switch between these profiles in each specific repository with one command.  
  
Since gitconf is an overlay, you can execute all classic git commands through it.
For example, `gitconf clone` instead of `git clone`.
In this case, gitconf will make sure that the git settings match the selected profile before executing the command.
You can also set up an interactive profile selection dialog that is displayed when you start working with a new repository.

# Usage
Gitconfig inherits all existing git commands but also adds a few of its own.  
## gitconf show-profiles
Shows a list of all settings profiles visible from the current directory.
## gitconf show-profile
Shows settings contained in the selected profile. Accepts the profile name as a single argument.
## gitconf set-profile
Sets the selected profile in the current directory. Accepts the profile name as a single argument.
## gitconf set-profile-path
Installs the profile file located at the specified path in the current directory.  
Accepts the path to the profile file as a single argument.

# Configurarion Mechanism
Gitconf stores configuration data in directories named **.gitconf**.
Gitconf is looking for a directory **.gitconf** by the path in which it is launched and by each parent path.
Profiles are stored in **.gitconf/profiles** as toml files.
A copy of the currently selected profile is stored in **.gitconf/current**.
Configs located on the closer path takes precedence over what is further away.
Similar to how it is done in git, the global settings file is read first, and then all the lower ones in turn.
Each config overrides or extends parameters of the higher one.
Also the config located in **/etc/.gitconf** is considered the most global.  
  
Usually, when installing giconf, the default config is created in "/etc/.gitconf/current/DEFAULT".  
You can use it as an example.
## Configurarion Options
Profile file can contain the following parameters
### Strict
Boolean option telling gitconf to ignore all higher-level configs. By default, **false**.
### StrictGit
A boolean option instructing gitconf to overwrite any git settings made not through gitconf itself. By default, **true**.
### SelectProfileOnFirstUse
Show the profile dialog with the profile selection when starting work in a new repository. By default, **false**.
### ShowCurrentProfile
Print the name of the current profile in stdout when calling the git command. By default, **true**.
### Interactive
Enables showing interactive dialogs. By default, **false**.
### Config
Key-value dictionary. The gle key is a git parameter and the value is its value.

# Instalation
## From source
Require
+ rust toolcahin
+ make
+ gzip

```
# Clone project
git clone https://github.com/DomesticMoth/gitconf.git
cd gitconf
# Build native
make build
# Install
sudo make install
```
## Download deb package
You may download prebuilt deb packages from [releases page](https://github.com/DomesticMoth/gitconf/releases)   
v0.1.0 sha1 checksums 
``` 
gitconf-arm7.gz      f597a8d3ae7416713364b5be9221f0cef5608c02
gitconf-x64.gz       1c917fad94e14c5672f0f8272ff1b7bba7803a1a
gitconf-deb-arm7.deb b2b4bc614a88466c44012bf0ee5f700f93c4e146
gitconf-deb-x64.deb  6d61db782b4ac1a74aca1f80ea6c91547f7c7642
```
## Buld deb package
You can also build a deb package from the source code yourself  
Require
+ rust toolcahin
+ make
+ gzip
+ dpkg

```
# Clone project
git clone https://github.com/DomesticMoth/gitconf.git
cd gitconf
# Build for x86-64
make build-deb-x64
# or for arm7
make build-deb-arm7
# Builded packages are saved in the ./out directory
```
