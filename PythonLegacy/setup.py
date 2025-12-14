from setuptools import setup, find_packages

setup(
    name="mosaic",
    version="0.2.0",
    packages=find_packages(),          # important: include the mosaic package
    install_requires=[
        "pandas~=2.3",
        "numpy~=2.3",
        "tqdm~=4.67",
        "matplotlib~=3.10",
        "python-dateutil>=2.8.2",
        "pytz",
        "tzdata",
        "six",
    ],
    entry_points={
        "console_scripts": [
            "mosaic=mosaic.cli:main",        # <— FIXED
        ],
    },
    python_requires=">=3.12",
)