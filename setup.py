from setuptools import setup, find_packages
from setuptools_rust import Binding, RustExtension

# This is super cute
try:
    from setuptools_rust import RustExtension
except ImportError:
    import subprocess

    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools-rust"])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension

setup(
    name="rsplanner",
    version="0.1",
    author="Left Hand Robotics",
    description="LHR Robot Code",
    rust_extensions=[RustExtension("pathplanning", binding=Binding.PyO3)],
    # packages=find_packages(),

    # install_requires=[
    #     'docopt==0.6.2',
    #     'colorama==0.3.9',
    #     'pytz==2017.3',
    #     'PyYAML==5.3.1',
    #     'aioreactive==0.5.0',
    #     'websockets==8.0.2',
    #     'aiohttp==3.6.2',
    #     'protobuf==3.11.1',
    #     'transitions==0.7.1',
    #     'passlib==1.7.1',
    #     'pymap3d==1.8.1',
    #     'aiofiles==0.5.0',
    #     'dubins==1.0.1',
    #     'rdp==0.8',
    #     'Shapely==1.7a1',
    #     # 'scipy==1.2.0',
    #     'numpy==1.16.0',
    #     'prettyprinter==0.18.0'
    #     # 'scipy==1.2.0',
    #     # pymap3d==1.8.1
    #     # Shapely==1.7a1
    #     # numpy==1.16.0
    # ],

    # entry_points={
    #     'console_scripts': [
    #         'rpc=common.rpc.rpc_cli:main',
    #     ],
    # },
)
