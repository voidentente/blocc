# Blocc

## Logging

### ex `RUST_LOG=wgpu=error,bevy_render=info,bevy_ecs=trace bloccgame`

### or `set RUST_LOG=wgpu=error,bevy_render=info,bevy_ecs=trace (CRLF) bloccgame.exe`

###

## TPS (Sided.SERVER)

### (Only Server runs world updates - Hollow internal server (?))

## GameState (Sided.CLIENT)

### ModDiscovery

### ModLoading

### AssetLoading

### MainMenu

### Connecting

### Playing

### Paused

## GameState (Sided.SERVER)

### ModDiscovery

### ModLoading

### WorldGeneration

### Playing

### Paused

## ModAPI

### Load Order

### Versioning

## Items

### Item ID

## Blocks

### Air as 0 ID (not None) (https://doc.rust-lang.org/reference/type-layout.html)

### Block State

### Texture Atlas (see Rendering)

## Entities

### Player

#### Signature (ID)

#### Name

#### Skin

### Mobs

### TileEntities

## Rendering (Sided.CLIENT)

On Linux, Bevy currently requires Vulkan for graphics.

On Windows, either Vulkan or DirectX 12 can be used.

macOS/iOS should work without any special driver setup, using Metal.

OpenGL should work as a fallback, for systems that do not support other APIs.

### Culling

### Render Distance

### Lighting

#### Light Level

##### Transparency

##### Light Sources

## World

### Chunks

### Height Limit

### Sky

### Day/Night Cycle

## World Generation (Sided.SERVER)

### Terrain: Perlin Noise with Seed

### Caves: Perlin Worms with Seed

### Biomes: Temp/Humid Lookup Chart or River Separator

## Netcode

### Backend

#### QUINN/QUIC (https://github.com/Henauxg/bevy_quinnet)

#### NAIA (https://github.com/naia-lib/naia)

##### Architecture (https://www.gabrielgambetta.com/client-server-game-architecture.html)

###### Predicion & Rollback: Wait until possible rollback before applying noticable changes (Analysis: Why Rollback Netcode Is Better (https://youtu.be/0NLe4IpdS1w))

###### Rift: Clients should not be able to roll back server (i.e. late commands get dropped)

## Publishing

### Standalone

### Steam

#### Steamworks Integration (https://github.com/HouraiTeahouse/bevy-steamworks)

## Misc

### Rich Presence (https://github.com/jewlexx/bevy-discord-rpc)

### Note: Currently broken

## Resources (Bevy)

- https://bevyengine.org/learn/book/introduction/

- https://bevy-cheatbook.github.io/

- https://docs.rs/bevy/latest/bevy/

- https://bevyengine.org/assets/

- https://github.com/bevyengine/bevy

## Resources (Rust)

- https://doc.rust-lang.org/reference/type-layout.html

## Resources (Minecraft)

- Making Minecraft from scratch in 48 hours (https://youtu.be/4O0_-1NaWnY)

## MSRV 1.64
