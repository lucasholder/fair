#!/bin/bash

name="fair"
repo="lucasholder/fair"

get_latest_release() {
  curl --silent "https://api.github.com/repos/$1/releases/latest" | # Get latest release from GitHub api
    grep '"tag_name":' |                                            # Get tag line
    sed -E 's/.*"([^"]+)".*/\1/'                                    # Pluck JSON value
}

get_os() {
  UNAME=$(uname)
  if [ "$UNAME" == "Linux" ] ; then
    echo "unknown-linux-gnu"
  elif [ "$UNAME" == "Darwin" ] ; then
    echo "apple-darwin"
  elif [[ "$UNAME" == CYGWIN* || "$UNAME" == MINGW* ]] ; then
    echo "windows"
  fi
}

get_arch() {
  echo $(uname -m)
}

install() {
  tag=$(get_latest_release ${repo})
  os=$(get_os)
  arch=$(get_arch)
  if [ "${os}" = "windows" ]; then
    echo "No precompiled binaries for windows. Use \"cargo install ${name}\"."
    exit 1;
  fi;
  url="https://github.com/${repo}/releases/download/${tag}/${name}-${tag}-${arch}-${os}.tar.gz"

  curl -sL ${url} | tar xz
  mv ${name} /usr/local/bin
}

install
