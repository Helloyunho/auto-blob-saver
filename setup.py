import setuptools


def long_description():
    readme = ""
    with open("README.md", "r", encoding="utf8") as r:
        readme = r.read()
    return readme


setuptools.setup(
    name="auto_blob_saver",
    version="0.1.2",
    description="Save your blobs automatically.",
    long_description=long_description(),
    long_description_content_type="text/markdown",
    url="https://github.com/Helloyunho/auto-blob-saver",
    author="Helloyunho",
    author_email="yunho050840@gmail.com",
    packages=setuptools.find_packages(),
    python_requires=">=3.8",
    scripts=["bin/auto_blob_saver"],
)
