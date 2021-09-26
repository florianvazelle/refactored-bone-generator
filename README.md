# refactored-bone-generator

This is a reworked version of my [bone generator addon](https://github.com/florianvazelle/bone-generator), in which the backend is now written in Rust.  
It is an addon based on a [client-server](https://grpc.io) model using [grpc](https://grpc.io) and [tonic](https://github.com/hyperium/tonic).

## Getting start 

### Install addon

To develop, or to use the blender addon, create a symbolic link in the blender addons folder :
```bash
ln -s $(pwd)/frontend/src/ ~/.config/blender/2.91/scripts/addons/bone-generator
```
or, for windows :
```bash
mklink /D "%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\2.91\scripts\addons\bone-generator" "frontend\src"
```

#### Python requirements

I install blender 2.91 with snap :
```
snap install blender --channel=2.91/stable --classic
```

To install python dependencies, go to blender terminal and run :
```python
>>> import sys
>>> print(sys.executable)
'/snap/blender/65/2.91/python'
```
and run :
```bash
/snap/blender/65/2.91/python/bin/python3.7m -m ensurepip --user
/snap/blender/65/2.91/python/bin/python3.7m -m pip install -r frontend/requirements.txt
```

### Run local server

To run local server, go to `backend/` folder and simply run :

```bash
cargo run
```

## Developement

### Protobuf 

To generate python proto models, run :
```bash
pip install -r frontend/requirements.txt
python -m grpc_tools.protoc -I . --python_betterproto_out=frontend/src/proto proto/service.proto
```

