#! /bin/bash
git checkout source
./compile.sh
git checkout master

git checkout source htmlout
mv htmlout/* ./
rmdir htmlout
