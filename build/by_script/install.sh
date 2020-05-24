#!/bin/sh
install_directory=/usr/bin

if ! [ -x `command -v git` ]; then
  echo "git command not found (please install git)";
  exit 1;
fi

if ! [ -x `command -v aws` ]; then
  echo "aws command not found (please install aws cli)";
  exit 1;
fi

if ! [ -x `command -v cargo` ]; then
  echo "cargo command not found (please install rust/cargo)";
  exit 1;
fi

if ! [ -d short ]; then
  git clone https://github.com/vincent-herlemont/short.git
fi

release_file=target/release/short

cd short

if ! [ -f $release_file ]; then
  cargo build --release --frozen --all-features
fi

if [ -d $install_directory ]; then
  sudo chmod 755 $release_file
  sudo cp $release_file $install_directory
fi

