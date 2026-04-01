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

Games are typically very short — often under 20 moves, sometimes under 10. The queen is overwhelmingly the strongest piece. The known strongest opening is **1.e3**, aiming to deploy the queen as quickly as possible.

## Architecture

```
benedict-chess/
├── engine/          # Core engine library
│   ├── bitboard     # Bitboard(u64) with magic bitboards for sliding pieces
│   ├── board        # Board state with make/unmake and flip logic
│   ├── movegen      # Benedict-specific move generation + perft
│   ├── search       # Alpha-beta with iterative deepening, LMR, aspiration windows
│   ├── eval         # Tunable evaluation (material, PST, threats, mobility, king safety)
│   ├── tt           # Transposition table with Zobrist hashing
│   └── ...          # types, FEN, UCI protocol, zobrist keys
├── cli/             # Binaries
│   ├── main         # UCI-compatible engine (stdin/stdout)
│   └── match_runner # Self-play framework with ELO calculation
├── api/             # HTTP API server (Axum)
└── Dockerfile       # Multi-stage build for deployment
```

## Performance

| Metric | Value |
|---|---|
| Nodes per second | ~10M (release, Apple Silicon) |
| Depth 10 from startpos | ~2.5 seconds |
| Perft(5) from startpos | 4,606,766 nodes in 0.06s |

### Search Features

- Iterative deepening with alpha-beta (negamax)
- Transposition table with Zobrist hashing
- Late Move Reductions (LMR) — ~40% tree size reduction
- Aspiration windows
- Flip-value-based move ordering (estimates conversion value before making moves)
- Killer moves and history heuristic

## Usage

### UCI Engine

```bash
cargo run --release --bin benedict
```

Then interact via UCI protocol:

```
position startpos
go depth 10
```

Additional commands: `d` (display board), `perft <depth>`, `moves` (list legal moves).

### Self-Play / Benchmarking

Test evaluation improvements against the baseline:

```bash
cargo run --release --bin benedict-match -- --test queen_threat_high --games 20 --time 1000
```

Available tests: `baseline`, `mobility`, `knight_threats`, `king_shield`, `tempo`, `queen_threat_high`, `combined`, `qt_ks`, `qt_tempo`, `qt_ks_tempo`, `all`.

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

### Live API

The engine is deployed at:

```
https://benedict-chess-api.onrender.com
```

## Benchmarking Results

Each evaluation feature was tested in a 20-game match at 1s/move against the baseline:

| Feature | Score | ELO |
|---|---|---|
| QueenThreat(30) | 18-2 | +382 |
| Mobility(3) | 17-3 | +301 |
| KingShield(15) | 17-3 | +301 |
| Tempo(20) | 15-5 | +191 |
| KnightThreat(10) | 14-6 | +147 |
| All combined | 2-18 | -382 |

Key finding: **search depth dominates evaluation quality** in Benedict chess. Adding eval complexity slows the search and loses more strength than it gains. Each feature wins individually but combining them makes the eval too expensive, resulting in shallower search.

## References

- [Benedict Chess rules](https://www.chessvariants.com/difftaking.dir/benedict.html) — Original page by W.D. Troyka
- [White Opening Study](https://www.schemingmind.com/home/journalarticle.aspx?article_id=183) — Statistical analysis of the 1.e3 system
- [Black's Response to 1.e3](https://www.schemingmind.com/home/journalarticle.aspx?article_id=189) — Defensive theory
