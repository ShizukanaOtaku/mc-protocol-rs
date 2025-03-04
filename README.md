This crate is very early in development, it is not ready to be used for anything yet!!!
You can watch this repository if you want to be notified about the first release.

# Features to implement:
- [ ] Compression
- [ ] All the data types

# Packets:
# Handshaking

### Serverbound
- [x] Handshake
- [ ] Legacy Server List Ping

# Status

### Clientbound
- [x] Status Response
- [ ] Pong Response (status)
### Serverbound
- [ ] Status Request
- [ ] Ping Request (status)

# Login

### Clientbound
- [ ] Disconnect (login)
- [ ] Encryption Request
- [ ] Login Success
- [ ] Set Compression
- [ ] Login Plugin Request
- [ ] Cookie Request (login)
### Serverbound
- [ ] Login Start
- [ ] Encryption Response
- [ ] Login Plugin Response
- [ ] Login Acknowledged
- [ ] Cookie Response (login)

# Configuration

### Clientbound
- [ ] Cookie Request (configuration)
- [ ] Clientbound Plugin Message (configuration)
- [ ] Disconnect (configuration)
- [ ] Finish Configuration
- [ ] Clientbound Keep Alive (configuration)
- [ ] Ping (configuration)
- [ ] Reset Chat
- [ ] Registry Data
- [ ] Remove Resource Pack (configuration)
- [ ] Add Resource Pack (configuration)
- [ ] Store Cookie (configuration)
- [ ] Transfer (configuration)
- [ ] Feature Flags
- [ ] Update Tags (configuration)
- [ ] Clientbound Known Packs
- [ ] Custom Report Details (configuration)
- [ ] Server Links (configuration)
### Serverbound
- [ ] Client Information (configuration)
- [ ] Cookie Response (configuration)
- [ ] Serverbound Plugin Message (configuration)
- [ ] Acknowledge Finish Configuration
- [ ] Serverbound Keep Alive (configuration)
- [ ] Pong (configuration)
- [ ] Resource Pack Response (configuration)
- [ ] Serverbound Known Packs

# Play

### Clientbound
- [ ] Bundle Delimiter
- [ ] Spawn Entity
- [ ] Spawn Experience Orb
- [ ] Entity Animation
- [ ] Award Statistics
- [ ] Acknowledge Block Change
- [ ] Set Block Destroy Stage
- [ ] Block Entity Data
- [ ] Block Action
- [ ] Block Update
- [ ] Boss Bar
- [ ] Change Difficulty
- [ ] Chunk Batch Finished
- [ ] Chunk Batch Start
- [ ] Chunk Biomes
- [ ] Clear Titles
- [ ] Command Suggestions Response
- [ ] Commands
- [ ] Close Container
- [ ] Set Container Content
- [ ] Set Container Property
- [ ] Set Container Slot
- [ ] Cookie Request (play)
- [ ] Set Cooldown
- [ ] Chat Suggestions
- [ ] Clientbound Plugin Message (play)
- [ ] Damage Event
- [ ] Debug Sample
- [ ] Delete Message
- [ ] Disconnect (play)
- [ ] Disguised Chat Message
- [ ] Entity Event
- [ ] Teleport Entity
- [ ] Explosion
- [ ] Unload Chunk
- [ ] Game Event
- [ ] Open Horse Screen
- [ ] Hurt Animation
- [ ] Initialize World Border
- [ ] Clientbound Keep Alive (play)
- [ ] Chunk Data and Update Light
- [ ] World Event
- [ ] Particle
- [ ] Update Light
- [ ] Login (play)
- [ ] Map Data
- [ ] Merchant Offers
- [ ] Update Entity Position
- [ ] Update Entity Position and Rotation
- [ ] Move Minecart Along Track
- [ ] Update Entity Rotation
- [ ] Move Vehicle
- [ ] Open Book
- [ ] Open Screen
- [ ] Open Sign Editor
- [ ] Ping (play)
- [ ] Ping Response (play)
- [ ] Place Ghost Recipe
- [ ] Player Abilities (clientbound)
- [ ] Player Chat Message
- [ ] End Combat
- [ ] Enter Combat
- [ ] Combat Death
- [ ] Player Info Remove
- [ ] Player Info Update
- [ ] Look At
- [ ] Synchronize Player Position
- [ ] Player Rotation
- [ ] Recipe Book Add
- [ ] Recipe Book Remove
- [ ] Recipe Book Settings
- [ ] Remove Entities
- [ ] Remove Entity Effect
- [ ] Reset Score
- [ ] Remove Resource Pack (play)
- [ ] Add Resource Pack (play)
- [ ] Respawn
- [ ] Set Head Rotation
- [ ] Update Section Blocks
- [ ] Select Advancements Tab
- [ ] Server Data
- [ ] Set Action Bar Text
- [ ] Set Border Center
- [ ] Set Border Lerp Size
- [ ] Set Border Size
- [ ] Set Border Warning Delay
- [ ] Set Border Warning Distance
- [ ] Set Camera
- [ ] Set Center Chunk
- [ ] Set Render Distance
- [ ] Set Cursor Item
- [ ] Set Default Spawn Position
- [ ] Display Objective
- [ ] Set Entity Metadata
- [ ] Link Entities
- [ ] Set Entity Velocity
- [ ] Set Equipment
- [ ] Set Experience
- [ ] Set Health
- [ ] Set Held Item (clientbound)
- [ ] Update Objectives
- [ ] Set Passengers
- [ ] Set Player Inventory Slot
- [ ] Update Teams
- [ ] Update Score
- [ ] Set Simulation Distance
- [ ] Set Subtitle Text
- [ ] Update Time
- [ ] Set Title Text
- [ ] Set Title Animation Times
- [ ] Entity Sound Effect
- [ ] Sound Effect
- [ ] Start Configuration
- [ ] Stop Sound
- [ ] Store Cookie (play)
- [ ] System Chat Message
- [ ] Set Tab List Header And Footer
- [ ] Tag Query Response
- [ ] Pickup Item
- [ ] Synchronize Vehicle Position
- [ ] Set Ticking State
- [ ] Step Tick
- [ ] Transfer (play)
- [ ] Update Advancements
- [ ] Update Attributes
- [ ] Entity Effect
- [ ] Update Recipes
- [ ] Update Tags (play)
- [ ] Projectile Power
- [ ] Custom Report Details
- [ ] Server Links
### Serverbound
- [ ] Confirm Teleportation
- [ ] Query Block Entity Tag
- [ ] Bundle Item Selected
- [ ] Change Difficulty
- [ ] Acknowledge Message
- [ ] Chat Command
- [ ] Signed Chat Command
- [ ] Chat Message
- [ ] Player Session
- [ ] Chunk Batch Received
- [ ] Client Status
- [ ] Client Tick End
- [ ] Client Information (play)
- [ ] Command Suggestions Request
- [ ] Acknowledge Configuration
- [ ] Click Container Button
- [ ] Click Container
- [ ] Close Container
- [ ] Change Container Slot State
- [ ] Cookie Response (play)
- [ ] Serverbound Plugin Message (play)
- [ ] Debug Sample Subscription
- [ ] Edit Book
- [ ] Query Entity Tag
- [ ] Interact
- [ ] Jigsaw Generate
- [ ] Serverbound Keep Alive (play)
- [ ] Lock Difficulty
- [ ] Set Player Position
- [ ] Set Player Position and Rotation
- [ ] Set Player Rotation
- [ ] Set Player Movement Flags
- [ ] Move Vehicle
- [ ] Paddle Boat
- [ ] Pick Item From Block
- [ ] Pick Item From Entity
- [ ] Ping Request (play)
- [ ] Place Recipe
- [ ] Player Abilities (serverbound)
- [ ] Player Action
- [ ] Player Command
- [ ] Player Input
- [ ] Player Loaded
- [ ] Pong (play)
- [ ] Change Recipe Book Settings
- [ ] Set Seen Recipe
- [ ] Rename Item
- [ ] Resource Pack Response (play)
- [ ] Seen Advancements
- [ ] Select Trade
- [ ] Set Beacon Effect
- [ ] Set Held Item (serverbound)
- [ ] Program Command Block
- [ ] Program Command Block Minecart
- [ ] Set Creative Mode Slot
- [ ] Program Jigsaw Block
- [ ] Program Structure Block
- [ ] Update Sign
- [ ] Swing Arm
- [ ] Teleport To Entity
- [ ] Use Item On
- [ ] Use Item
