# Notes

## Publishing to crates.io

Publish the crates in the following order:
 1. sauron-core
 2. sauron-parse
 3. sauron-node-macro
 4. sauron-markdown
 5. sauron



# Dependency graph
 sauron-parse depends on sauron-core
 sauron-node-macro depends on sauron-parse
 sauron-markdown depends on sauron-parse
 sauron depends on sauron-core, sauron-node-macro, sauron-markdown (optional), sauron-parse (optional)


                sauron-core
              /     |
             /      |
            /       V
           /   sauron-parse -----+
          /         |             \
         /          |              \
        |           V               |
        |    sauron-node-macro      |
        |           |        |      |
        |           |        |      |
        |           V        |      |
        \    sauron-markdown |      |
         \          !        |     /
          \         !       /     /
           \        !      /     /
            V       V     V     /
                sauron  <------+


