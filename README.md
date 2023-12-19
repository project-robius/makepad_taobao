# makepad_taobao

TaoBao-like application implemented with Makepad

## Build Instructions


## 1. Setup Makepad

### Clone the Makepad repository
```
git clone git@github.com:makepad/makepad.git
```

### Change to latest 'rik' branch (Optional)
```
git branch rik
```

### Install makepad subcommand for cargo
```
cd ~/makepad
cargo install --path ./tools/cargo_makepad
```

## 2. Get Project

### Clone the makepad_taobao repo
```
git clone https://github.com/project-robius/makepad_taobao
```

## 3. Android Build

### Install Android toolchain (First time)
```
rustup toolchain install nightly
cargo makepad android install-toolchain
```

### Install app on Android devivce or Android emulator
Open either the Android emulator or connect to a real Android device
use `adb` command to make sure there's a device connected properly
```
cd ~/makepad_taobao
cargo makepad android run -p makepad_taobao --release
```

## 4. iOS Setup & Install

### Install IOS toolchain (First time)
```
rustup toolchain install nightly
cargo makepad ios install-toolchain
```

### Install app on Apple devivce or iOS simulator

### iOS Setup

For iOS, the process is slightly more complicated. The steps involved are:
1. Enable your iPhone's Developer Mode, please see instructions here: [Enable Developer Mode](https://www.delasign.com/blog/how-to-turn-on-developer-mode-on-an-iphone/)
1. Setup an Apple Developer account
1. Setup an empty skeleton project in XCode
    1. File -> New -> Project to create a new "App"
    1. Set the Product Name as **`TaoBao`**  (used in --org later)
    1. Set the Organization Identifier to a value of your choice, for this example we will use **`rs.robius`**. (used in --app later)
    1. Setup the Project Signing & Capabilities to select the proper team account 
1. In XCode, Build/Run this project to install and run the app on the simulator and device
1. Once the simulator and device has the "skeleton" app installed and running properly, then it is ready for Makepad to install its application.

### Makepad Install
We will run the `cargo makepad ios` command, similar to Android build above, but there are some 3 to 4 additional parameters that need to be filled in:

`--org-id`

This is the <string> value of the ApplicationIdentifierPrefix <key> in the `**.mobileprovision` file located in the `~/Library/MobileDevice/Provisioning Profiles` directory.
It should be a 10 digit alphanumeric value.

`--org`
    
First few parts of the organization identifier (which makes up the Bundle Identifier). Usually in the form of **com.somecompany** or **org.someorg**
This is the same value used to setup the initial skeleton app above. For this example:
> `rs.robius`
    
`--app`

The name of the application or the project. This is the same as the Product Name used to setup the initial skeleton app above. In this case:
> `TaoBao`

### Example

For this example, we have the Bundle Identifier of **`rs.robius.TaoBao`**

### Install app on IOS simulator
```
cd ~/makepad_taobao
cargo makepad apple ios --org=rs.robius --app=TaoBao run-sim -p makepad_taobao --release
```

### Install app on IOS device
```
cd ~/makepad_taobao
cargo makepad apple ios --org-id=<ORGIDVALUE> --org=rs.robius --app=TaoBao run-device -p makepad_taobao --release
```

## 5. WASM Build

*Coming Soon*

