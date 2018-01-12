travis-cargo --only stable publish -- --token $CARGO_TOKEN &&
export VERSION=`cargo pkgid | sed -E 's/.*#(.*:)?(.+)/\2/'` &&
git tag v$VERSION &&
git push https://awestlake87:$GITHUB_TOKEN@github.com/awestlake87/sc2-rs v$VERSION
