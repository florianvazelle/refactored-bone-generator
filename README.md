# refactored-bone-generator

```bash
ln -s $(pwd)/frontend/src/ ~/.config/blender/2.91/scripts/addons/bone-generator
```

I install blender 2.91 with snap :
```
snap install blender --channel=2.91/stable --classic
```

To install python dependencies, go to blender terminal and run :
```python
>>> import sys
>>> sys.exec_prefix
'/snap/blender/65/2.91/python'
```
```bash
/snap/blender/65/2.91/python/bin/python3.7m -m ensurepip --user
/snap/blender/65/2.91/python/bin/python3.7m -m pip install -r frontend/requirements.txt
```

- http://www.codeplastic.com/2019/03/12/how-to-install-python-modules-in-blender/
- https://blender.stackexchange.com/a/107381