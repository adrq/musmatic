#!/bin/bash

DIRECTORY="vendor/verovio"

COMMIT="d58523025b6b6b72eaf68beb050d47f4c775d7f8"

#check if verovio already cloned
if [ -d "$DIRECTORY" ]; then
  echo "Already cloned. Updating."
  cd $DIRECTORY
  git checkout develop
  git reset --hard HEAD
  git pull
  git checkout $COMMIT
else
  echo "Cloning git repo"
  #if not then we clone repo
  git clone --depth 1 https://github.com/adrq/verovio.git $DIRECTORY
  cd $DIRECTORY
  git checkout $COMMIT
fi

cd tools
./get_git_commit.sh

#fix CMake recipe to make a static library instead of shared
cd ../cmake
sed -i -e 's/SHARED/STATIC/g' CMakeLists.txt

