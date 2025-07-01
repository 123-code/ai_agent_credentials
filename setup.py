from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    rust_extensions=[
        RustExtension(
            "ai_credentials.ai_credentials",
            binding=Binding.PyO3,
            debug=False,
        )
    ],
    zip_safe=False,
) 