set -ex 

HASH=$(git rev-parse --verify HEAD)
docker build --progress=plain -t sheet-to-meilisearch:$HASH .
docker tag sheet-to-meilisearch:$HASH arranf/sheet-to-meilisearch:$HASH
docker push arranf/sheet-to-meilisearch:$HASH
echo arranf/sheet-to-meilisearch:$HASH