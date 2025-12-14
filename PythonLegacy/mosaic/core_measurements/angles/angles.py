from pathlib import Path
from typing import Optional, List
import numpy as np
import pandas as pd
from tqdm import tqdm
from mosaic.core_measurements.angles.angle_calculator import AngleCalculator
from mosaic.core_measurements.angles.angle_uncertainty import AngleUncertaintyCalculator
from mosaic.config import LANDMARK_PAIRS
from mosaic.schemas import WIDE_CSV_STRUCTURE

def run(input_file: str, output_file: Path, *, force: bool=False, start: int=0, end: Optional[int]=None) -> None:
    out_csv = output_file
    out_csv.parent.mkdir(parents=True, exist_ok=True)
    if not out_csv.exists() or out_csv.stat().st_size == 0:
        pd.DataFrame(columns=WIDE_CSV_STRUCTURE).to_csv(out_csv, index=False)

    src = pd.read_csv(input_file)
    total = len(src)
    if end is None or end > total: end = total
    if start < 0: start = 0
    if start >= end:
        print("Nothing to do: start >= end."); return

    A = AngleCalculator(src)
    U = AngleUncertaintyCalculator(src)

    updates: List[dict] = []
    for row in tqdm(range(start, end), desc="Angles", unit="frame"):
        try:
            th = A.compute_angles(row, LANDMARK_PAIRS)
            su = U.compute_sigma_theta(row, LANDMARK_PAIRS)
            rec = {"frame": int(row)+1}
            rec.update({f"theta_{i}": float(th[i - 48]) for i in range(48, 68)})
            rec.update({f"theta_{i}_unc": float(su[i - 48]) if np.isfinite(su[i - 48]) else np.nan for i in range(48, 68)})
            updates.append(rec)
        except Exception as e:
            print(f"Error on row {row}: {e}")

    if updates:
        wide = pd.read_csv(out_csv)
        if "frame" not in wide.columns:
            raise FileNotFoundError("Wide CSV missing 'frame' column.")
        wide_idx = wide.set_index("frame")
        upd = pd.DataFrame(updates).set_index("frame")

        missing_cols = [c for c in upd.columns if c not in wide_idx.columns]
        if missing_cols:
            wide_idx = wide_idx.reindex(columns=list(wide_idx.columns) + missing_cols)

        all_idx = wide_idx.index.union(upd.index)
        wide_idx = wide_idx.reindex(all_idx)
        wide_idx.loc[upd.index, list(upd.columns)] = upd.to_numpy()

        wide = wide_idx.reset_index()
        wide = wide.reindex(columns=WIDE_CSV_STRUCTURE + [c for c in wide.columns if c not in WIDE_CSV_STRUCTURE])
        wide.sort_values("frame", inplace=True)
        wide.to_csv(out_csv, index=False)

    print(f"Angles → {out_csv.resolve()}")
