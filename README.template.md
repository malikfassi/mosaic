# Pixel Canvas

[![Frontend CI](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/malikfassi/1ff46a4915f58fa0fce5cab7577f94f1/raw/frontend-ci.json)](https://github.com/malikfassi/mosaic/actions/workflows/pixel-canvas.yml)
[![Mosaic Tile CI](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/malikfassi/1ff46a4915f58fa0fce5cab7577f94f1/raw/mosaic-tile-ci.json)](https://github.com/malikfassi/mosaic/actions/workflows/pixel-canvas.yml)

A decentralized pixel art canvas powered by Stargaze NFTs.

## Component Hashes

*Last updated: {{ .LastUpdated }}*

| Component | Hash |
|-----------|------|
| Frontend | `{{ .Hashes.Frontend }}` |
| Mosaic Tile Contract | `{{ .Hashes.MosaicTile }}` |

## Latest Deployment

*Deployed at: {{ .Deploy.Timestamp }}*

| Contract | Address |
|----------|---------|
| Mosaic Tile | `{{ .Deploy.MosaicTileAddress }}` |

## Account Balances

| Role | Address | Balance |
|------|---------|---------|
{{- range $role, $data := .Balances }}
| {{ $role }} | `{{ $data.Address }}` | ![{{ $role }} balance](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/raw/c67eb85b7002c9e7746d744ce70acbfb/{{ $role }}-balance.json) |
{{- end }}

**Total Balance:** ![Total balance](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/raw/c67eb85b7002c9e7746d744ce70acbfb/total-balance.json)

[Rest of the original README content...] 