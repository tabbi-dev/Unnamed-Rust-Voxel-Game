# Project Overview
This is a long-term project where I will be writing my own rendering engine and building a voxel survival, town management, factory building, and exploration game entirely in Rust with Vulkan. This is gonna be a very long-term project, and I am doubtful I'll be able to complete it on my own.
I intend to leave this repository public with the MIT license, however as I get further into development I may just leave this repo as the game engine itself and create a private fork for the actual game.


# Roadmap
- [ ] Rendering engine
- [ ] World generation
- [ ] Basic player and game mechanics
- [ ] Menus and UIs
- [ ] Localisation - I will create a branch for locales, everybody is welcome to contribute.

## Gameplay
- Player spawns on a colonist ship on a beachside or riverside with a small crew of First Generation settlers.
- Players and NPCs will start with a basic shovel, pickaxe, axe, and handsaw. 
  - Saws cut branches faster than axes, but axes break whole trees, timber and glass voxels, and thick logs faster.
    - When cut with an axe, trees will be destroyed upwards from the point where the player broke. (this will probably lead to buggy behaviour but I'll cross that bridge when I get to it.)
  - Shovels are good for digging 3x3 (two voxels deep) voxel areas whereas pickaxes dig out individual voxels for more precision at a slower speed. Pickaxes can also be used for digging holes for seeds whereas shovels are better at digging irrigation lines.
  - After the player has built houses for themself and their colonists, each NPC will have the option to specialise in a specific skill, which makes 


## NPCs
- These settler npcs will include 1 Oracle (based on Terraria's Guide NPC, will provide information, recipes, and hints to the player), and 3-5 standard NPCs who can be assigned simple tasks

## The Player

## The World 
[//]: <> (TOKI WO TOMARE)
- One block is 24^3 voxels. The player will be exactly 23 voxels tall.
- Like other voxel games, the world will be broken down into chunks. However, 