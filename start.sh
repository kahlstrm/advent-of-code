#!/bin/bash -e
# check if the first argument passed in exists
if [ -z "$1" ]; then
  FOLDER_NAME=$(date +"%y/%d")
else
  FOLDER_NAME=$1
fi

# check if folder exists, if exists, don't do anything
if [ -d "$FOLDER_NAME" ]; then
  echo "Folder $FOLDER_NAME already exists"
  exit 1
fi

echo "Creating folder $FOLDER_NAME"
cp -r template $FOLDER_NAME
#replace "REPLACE_PACKAGE_NAME_HERE" with the folder name in Cargo.toml file
sed -i '' "s|REPLACE_PACKAGE_NAME_HERE|aoc-$(echo $FOLDER_NAME | tr -d \/)|g" $FOLDER_NAME/Cargo.toml
sed -i '' "s|REPLACE_PACKAGE_NAME_HERE|aoc-$(echo $FOLDER_NAME | tr -d \/)|g" $FOLDER_NAME/Cargo.lock
echo "Done"
