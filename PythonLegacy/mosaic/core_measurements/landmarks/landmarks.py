from pathlib import Path
import pandas as pd
from mosaic.core_measurements.landmarks.landmark_uncertainty import XYUncertainty
from mosaic.schemas import WIDE_CSV_STRUCTURE

def run(input_wide_csv: str | Path, output_wide_csv: Path, *, force: bool=False, value: float=0.5):
    out_csv = Path(output_wide_csv)
    out_csv.parent.mkdir(parents=True, exist_ok=True)
    if not out_csv.exists() or out_csv.stat().st_size == 0:
        pd.DataFrame(columns=WIDE_CSV_STRUCTURE).to_csv(out_csv, index=False)

    wide = pd.read_csv(out_csv)
    if "frame" not in wide.columns:
        raise FileNotFoundError("Wide CSV missing 'frame' column.")
    wide_idx = wide.set_index("frame")

    U = XYUncertainty()
    upd = U.run(wide_idx.index)

    missing_cols = [c for c in upd.columns if c not in wide_idx.columns]
    if missing_cols:
        wide_idx = wide_idx.reindex(columns=list(wide_idx.columns) + missing_cols)

    wide_idx.loc[upd.index, upd.columns] = upd.to_numpy()

    wide = wide_idx.reset_index()
    wide = wide.reindex(columns=WIDE_CSV_STRUCTURE + [c for c in wide.columns if c not in WIDE_CSV_STRUCTURE])
    wide.sort_values("frame", inplace=True)
    wide.to_csv(out_csv, index=False)
    print(f"XY uncertainties (±{value}px) → {out_csv.resolve()}")
