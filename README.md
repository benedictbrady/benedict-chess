# Benedict Chess Engine

A search-based chess engine for [Benedict chess](https://www.chessvariants.com/difftaking.dir/benedict.html), a variant invented by W.D. Troyka where pieces are never captured — instead, all enemy pieces attacked by a moved piece are **converted** (flipped) to the mover's color. The game ends when a king is flipped.

Named after Benedict Arnold, the infamous traitor — fitting for a game where pieces switch allegiance.

## The Rules

Benedict chess uses the standard board and starting position, but with fundamentally different mechanics:

- **No capturing.** All 32 pieces remain on the board for the entire game.
- **Conversion.** When a piece moves, all enemy pieces it attacks from its new square are flipped to the mover's color.
- **Win condition.** Flip the enemy king.
- **No check or checkmate.** The only way to win is to directly convert the king.
- **Only the moved piece converts.** No discovered attacks, no chain reactions.
- **Pawns move forward only.** Their diagonal attacks are used solely for flipping, not movement.
- **Castling.** Legal (no "through check" restriction since check doesn't exist), but only the king causes flips — the rook does not.

Games are typically very short — often under 20 moves. The queen is the strongest piece. The known strongest opening is **1.e3**, deploying the queen as quickly as possible.

## Architecture

```
benedict-chess/
├── engine/          # Core engine library
│   ├── bitboard     # Bitboard(u64) with magic bitboards for sliding pieces
│   ├── board        # Board state with make/unmake and flip logic
│   ├── movegen      # Benedict-specific move generation + perft
│   ├── search       # Lazy SMP multi-threaded alpha-beta with LMR, aspiration windows
│   ├── eval         # Material, PST, threats, king danger heuristic
│   ├── tt           # Lock-free transposition table (AtomicU64) with Zobrist hashing
│   └── ...          # types, FEN, UCI protocol, zobrist keys
├── cli/             # Binaries
│   ├── main         # UCI-compatible engine (stdin/stdout)
│   └── match_runner # Self-play framework with ELO calculation
├── api/             # HTTP API server (Axum)
└── Dockerfile       # Multi-stage build for deployment
```

## Usage

### UCI Engine

```bash
cargo run --release --bin benedict
```

Then interact via UCI protocol:

```
setoption name Threads value 4
position startpos
go movetime 5000
```

Additional commands: `d` (display board), `perft <depth>`, `moves` (list legal moves).

**UCI options:**
- `Threads` (1-256, default 1) — number of search threads (Lazy SMP)
- `Hash` (1-1024, default 64) — transposition table size in MB

### Self-Play / Benchmarking

Test evaluation improvements against the baseline:

```bash
cargo run --release --bin benedict-match -- --test king_danger --games 20 --time 1000
```

Available tests: `baseline`, `mobility`, `knight_threats`, `king_shield`, `tempo`, `queen_threat_high`, `combined`, `king_danger`, `all`.

### HTTP API

```bash
cargo run --release --bin benedict-api
# Listening on 0.0.0.0:3001
```

**Endpoints:**

**`POST /api/analyze`** — Get the engine's best move.
```json
{
  "position": "startpos",
  "moves": ["e2e3", "e7e6"],
  "time_ms": 1000
}
```

**`POST /api/legal-moves`** — List all legal moves for a position.
```json
{
  "position": "startpos"
}
```

**`POST /api/make-move`** — Apply a move and see which pieces were flipped.
```json
{
  "position": "startpos",
  "moves": ["e2e3", "e7e6"],
  "move": "d1g4"
}
```

**`GET /health`** — Health check.

## Performance

| Metric | Value |
|---|---|
| Nodes per second | ~9M/thread (release, Apple Silicon) |
| Depth from startpos (1 thread, 5s) | 9 |
| Depth from startpos (8 threads, 5s) | 11 |
| Perft(5) from startpos | 4,606,766 nodes in 0.06s |

### Search Features

- Lazy SMP multi-threaded search (lock-free shared transposition table)
- Iterative deepening with alpha-beta (negamax)
- Transposition table with Zobrist hashing
- Late Move Reductions (LMR)
- Aspiration windows
- Flip-value-based move ordering
- Killer moves and history heuristic
- King danger evaluation (piece pressure on enemy king zone, +191 ELO)

## References

- [Benedict Chess rules](https://www.chessvariants.com/difftaking.dir/benedict.html) — Original page by W.D. Troyka
- [White Opening Study](https://www.schemingmind.com/home/journalarticle.aspx?article_id=183) — Statistical analysis of the 1.e3 system
- [Black's Response to 1.e3](https://www.schemingmind.com/home/journalarticle.aspx?article_id=189) — Defensive theory
