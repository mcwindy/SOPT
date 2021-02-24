# SOPT
A light-weighted yet powerful PT framework.

## How to run

1. Rename `.env.example` to `.env` and set your postgres name and password.

2. Run following commands in terminal:

```bash
createdb sopt
cargo install sqlx-cli
sqlx migrate run
cargo run
```

## Roadmap

- [ ] User
    - [x] Register
    - [x] Login
    - [ ] Information update
    - [x] Invite
    - [ ] Rank
    
- [ ] Torrent
    - [ ] Add
    - [ ] Generate with passkey
    - [ ] Sync with tracker
    - [ ] Search and filter
    
- [ ] Admin
    - [ ] Ban user
    - [ ] Site general setting
    - [ ] Torrent info update
    - [ ] Open feature
    
- [ ] Anti-Cheating
    - [ ] IP limitation
    - [ ] Client Ban
    - [ ] Monitor
  
## Known issues
   
1. Support for TLS is needed.
