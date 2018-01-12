if [ `travis-cargo --only stable pkgid` -eq `cargo pkgid` ]; then
  export VERSION=`cargo pkgid | sed -E 's/.*#(.*:)?(.+)/\2/'` &&
  git tag v$VERSION &&
  git push https://awestlake87:$GITHUB_TOKEN@github.com/awestlake87/sc2-rs v$VERSION
fi
