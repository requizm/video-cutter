#This code is written in Python and is used to create a zip file containing an executable and any included files. The code first runs the command "cargo build --release" which builds the executable. It then reads the last_build.json file which contains information about the version, executable, release path, and included files. The code then creates a zip file with the name of the executable and version number and adds the executable and included files to the folder within the zip file. Finally, if the user chooses to create a tag and release, the code checks if the user is authenticated with GitHub, creates a git tag, pushes the tag to remote, and creates a release on GitHub.


# Run "cargo build --release"
# Read last_build.json. Example:
"""
{
  "version": "1.3.0",
  "executable": "ipfs_downloader",
  "executable_with_ext": "ipfs_downloader.exe",
  "release_path": "target/release/",
  "included_files": [
    "config.json"
  ]
}
"""
# Create a zip file that contains folder. That folder contains:
# - executable_with_ext
# - included_files

import json
import os
import zipfile

def main():
    createTagAndRelease: bool = input("Create tag and release? (y/n): ").lower() == "y"

    if os.path.exists("last_build.json"):
        os.remove("last_build.json")

    command("cargo clean")
    command("cargo build --release")

    with open("last_build.json", "r") as f:
        last_build: dict = json.load(f)

    version: str = last_build["version"]
    executable: str = last_build["executable"]
    executable_with_ext: str = last_build["executable_with_ext"]
    release_path: str = last_build["release_path"]
    included_files: list[str] = last_build["included_files"]

    # Create a zip file. Example zip file:
    # -- ipfs_downloader-1.3.0.zip
    # ---- ipfs_downloader
    # ------ ipfs_downloader.exe
    # ------ config.json
    zip_file_name: str = f"{executable}-{version}.zip"
    zipfile_path: str = os.path.join(release_path, zip_file_name)
    with zipfile.ZipFile(zipfile_path, "w") as f:
        # Create a folder in zip file. Name of folder is executable.
        f.write(os.path.join(release_path, executable_with_ext), os.path.join(executable, executable_with_ext))

        # Add included files  to folder in zip file.
        for included_file in included_files:
            f.write(included_file, os.path.join(executable, included_file))

    if createTagAndRelease:
      # Check if authenticated with GitHub.
      command("gh auth status")

      # Create a git tag.
      command(f"git tag {version}")

      # Push tag to remote.
      command(f"git push origin {version}")

      # Create a release on GitHub.
      command(f"gh release create {version} --latest --verify-tag --generate-notes {zipfile_path}")

def command(cmd: str, print_cmd: bool = True, exit_on_error: bool = True):
    if print_cmd:
        print(cmd)

    status = os.system(cmd)

    if status != 0 and exit_on_error:
        print(f"Error: {status}")
        exit(status)

if __name__ == "__main__":
    main()
