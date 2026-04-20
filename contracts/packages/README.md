## Releasing
!! If you update dependencies in the root and they are also used in the packages rememeber to update them !!
- cd to package folder
- npm run release
- look how gh actions do publishing for you

## Troubleshooting
If you're getting 
`fatal: ambiguous argument 'origin/HEAD': unknown revision or path not in the working tree.`

run

`git fetch origin && git remote set-head origin -a`

Thanks for you attention.