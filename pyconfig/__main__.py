"""

Civa uses python as the config system backend.

The rational is to write the complete configurations for
civa in python.

The pyconfig app provides a minimal library that allows
the user the easily configure this shell.


"""

import argparse


def main():

    parser = argparse.ArgumentParser(description="The config reader for civa")
    parser.add_argument(
        "config_dir",
        type=str,
        help="The root dir for all configs for civa. Usually under $HOME/.config/civa",
    )

    parser.add_argument(
        "out",
        type=str,
        help="Destination of the Intermediate Representation File (IRF)",
    )

    print("Hello World")


if __name__ == "__main__":
    main()
