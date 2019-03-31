#!/bin/bash

DIRECTORY="vendor/verovio"

#check if verovio already cloned
if [ -d "$DIRECTORY" ]; then
  echo "Already cloned. Updating."
  cd $DIRECTORY
  git checkout develop
  git reset --hard HEAD
  git pull
  cd tools
  ./get_git_commit.sh

  #fix CMake recipe to make a static library instead of shared
  sed -i -e 's/SHARED/STATIC/g' CMakeLists.txt
  exit
fi

echo "Cloning git repo"
#if not then we clone repo
git clone --depth 1 https://github.com/rism-ch/verovio.git $DIRECTORY
cd $DIRECTORY
git checkout develop
cd tools
./get_git_commit.sh

#fix CMake recipe to make a static library instead of shared
sed -i -e 's/SHARED/STATIC/g' CMakeLists.txt
