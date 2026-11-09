# Database verification

The economic-data branch starts from the latest upstream packed databases at `1c8e835`. It adds nullable monetary columns for quest rewards and item buy/sell prices, plus the corresponding `vmangos-money` provenance tokens. It preserves the original nonmonetary values and schema objects.

## Forever data

- All 62 table projections match upstream after excluding the three added monetary columns and removing only the owned provenance token.
- All 5,812 matching quest monetary values and 9,554 item-price rows match the original fork values, including null and zero values.
- The 27 newly added upstream quests remain present with unknown monetary rewards. No reward was invented.
- All original provenance is retained. Exactly 820 quest records gain the monetary provenance token.
- Upstream totals remain 5,839 quests, 198,398 spawns and 218,663 localization rows.
- SQLite integrity passes; compression round-trip reproduces the verified database bytes.

The packed Forever SHA256 is `4ce8559524b61a9c659d4899aae73a129019219bf7d75a00559ec1c34d422b25`.

## Classic data

All nonmonetary projections across 58 tables and the existing schema objects match upstream. The submitted changes consist of monetary columns and their corresponding values/provenance. The Classic and Forever nonmonetary comparisons were independently reviewed.

## Examples

All amounts are copper.

| Record | Amount |
| --- | ---: |
| Quest 7 reward | 25 |
| Quest 12 reward | 500 |
| Quest 1200 reward | 6,500 |
| Item 25 buy price | 35 |
| Item 25 sell price | 7 |

The changes preserve VMaNGOS attribution in `NOTICE.md`. Unpacked local databases and installed game data were not modified during preparation.
