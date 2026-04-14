![chiyocore](logo.png)

an implementation of [meshcore](https://meshcore.co.uk/) for ESP32s, written in rust!

## warning labels

### radio settings are currently hardcoded
they're set to max tx/rx power and the US/Canada meshcore preset. alter as you wish/need

### be respectful to your mesh
don't spam flood packets, be careful with how you run it!

### general disclaimer
this is extremely experimental and not guaranteed to work or not fry your radio/board. i have had success running it on my xiaos3 kit, but please tread with care and do not assume things will work! 

## how to get it running

### set up a board definition
you'll need to set up a board definition file with the pin-out for the LoRa module, plus ram size for heap allocations. [you can see an example here](blossoms/boards/xiao-s3-kit.toml)

### configure your firmware build
the firmware is generated off a TOML file [here's an example](blossoms/setups/sample.toml)

```toml
[firmware]
stack_size = 32768
psram_mode = "quad"

[chiyocore]
config = { "wifi.pw" = "nya", "wifi.ssid" = "nya" }
default_channels = ["#test", "#emitestcorner"]

[stackup.chiyo0]
id = "chiyo0"

[stackup.chiyo0.layers.companion-0]
type = "chiyocore_companion::companionv2::Companion" # specifies layer rust type
id = "chiyocompanion0"
tcp_port = 5000

[stackup.chiyo0.layers.ping_bot]
type = "chiyocore::ping_bot::PingBot"
name = "cafe / chiyobot 🌃☕"
channels = ["#test", "#emitestcorner"]

[stackup.chiyo0.layers.ttc_bot]
deps = { chiyocore-ttc = { git = "https://codeberg.org/emisignet/chiyocore-ttc.git" } } # ping bot and companion are built-in, but here, we're adding a module from git!
type = "chiyocore_ttc::TTCBot"
```

### generate your firmware
`cargo run -- --firmware blossoms/setups/<yr-firmware>.toml --board blossoms/boards/<yr-board>.toml --out generated-firmware`

### run it!
`cd generated-firmware`
`DEFMT_LOG=info cargo run`

### if you're running a companion, connect to it:

with meshcore-cli:
`meshcore-cli -t <board-ip-address> -p <companion-port>`

## why chiyocore?
[i think sakura chiyono o is neat](https://www.youtube.com/watch?v=e3YcYLE90po)

## development notes!

### project architecture
the main meshcore logic is written in [firmware/chiyocore](firmware/chiyocore/). the goals are that this should provide a framework that handles:
- keeping track of contacts, channels and received packets
- most packet sending logic
- most packet decoding logic
so that layers built on top of it can remain as high-level as possible.

said layers are configured by the [builder](builder/) tool, which takes a board configuration plus a firmware setup config and generates a temporary binary crate linking all the configured handler layers together with the specified board pinout. it currently relies on statically knowing all possible layers and configurations (via the shared [chiyocore-config](chiyocore-config/) crate), though this should become more flexible in future.

example implementations of handler layers are the [companion](firmware/companion/) implementation, as well as the example [TTC bus arrival time bot](firmware/chiyo-ttc/).

### todos & random thoughts
- more radio support!!
- partition tables need to be configurable
- is packet delaying logic correct?
- need a reorg/cleanup pass
- stack usage could likely be improved
- currently, the firmware builder & runtime crates all depend on a shared config crate. this creates a little bit of lock-in that i don't love
