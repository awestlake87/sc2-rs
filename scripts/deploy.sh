cargo publish --token $CARGO_TOKEN || exit 1

VERSION=`cargo pkgid | sed -E 's/.*#(.*:)?(.+)/\2/'`
git tag v$VERSION || exit 2
git push https://awestlake87:$GITHUB_TOKEN@github.com/awestlake87/sc2-rs.git v$VERSION || exit 3
