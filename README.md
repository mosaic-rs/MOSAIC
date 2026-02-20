<img src="./assets/logos/MOSAIC-logo-HIGH.png" width="200" alt="MOSAIC Logo">

[![Rust, Python](https://img.shields.io/badge/languages-Rust%2C_Python-Blue
)](https://www.rust-lang.org/)
[![Topic](https://img.shields.io/badge/field-Phonetics-blue.svg)]()



# Motor-Oral Speech Analysis & Integrated Computation (MOSAIC)

Motor-Oral Speech Analysis & Integrated Computation (MOSAIC) is a mathematical system which tracks mouth and jaw movement and correlates those measurements to a [parselmouth/praat](https://parselmouth.readthedocs.io/en/stable/#) analysis. MOSAIC has three measurement "units": core measurements, complex measurements, and praat based measurements. 


## Compiling MOSAIC:
Below is how to compile MOSAIC for your specific machine - and whether you want a GUI or headless version. (Currently just MacOS)

### MacOS
#### Requirements:
- Rust
- Python 3.11.9 (Must be that version)
#### Tauri/GUI Version
Clone this repo into a desired folder and open terminal. Navigate to the root folder (probably called "MOSAIC") and run:

``` shell
./builds/build_mac_release.sh
```

It will create a DMG in the root folder and unless you decide to sign/notarize it, it should skip that.

(You can also just download the pre-configure v0.3.4-alpha DMG [here](https://github.com/mosaic-rs/MOSAIC/releases/download/0.3.4-alpha/MOSAIC_v0.3.4.dmg))

#### Headless/CLI Version:
In the root folder, do the following commands:
``` shell
# adding Python 3.11 to pythin_lib
ln -s /Library/Frameworks/Python.framework/Versions/3.11/lib/python3.11 ./src/python_lib

cargo install --path .
```

MOSAIC (without a GUI) should be fully installed. To access MOSAIC, type "mosaic" into your terminal.


Windows stuff will come soon! 

## MOSAIC Cores

(tiles if you will)

MOSAIC is made up of 5 major cores: 
- Universal Measurement Datastructure (UMD) (Gotta add stuff) [ReadMe](./src/UMD/README.md).
- Core Measurements [ReadMe](./src/coreMeasurements/README.md).
- Complex Measurements (Gotta add stuff) [ReadMe](./src/complexMeasurements/README.md).
- Praat Analysis [ReadMe](./src/praatAnalysis/README.md)
- Statistics Engine [ReadMe](./src/statisticsEngine/README.md)
