from pathlib import Path
import pandas as pd
from typing import List, Optional
from tqdm import tqdm
from mosaic.core_measurements.euclidean_distance.euclidean_distance import EuclideanDistance
from mosaic.core_measurements.euclidean_distance.euclidean_distance_uncertainty import EuclideanDistanceUncertainty
from mosaic.schemas import WIDE_CSV_STRUCTURE

def run(input_file: str, output_file: Path, *, force: bool=False, start: int=0, end: Optional[int]=None) -> None:
    out_csv = output_file
    out_csv.parent.mkdir(parents=True, exist_ok=True)
    if not out_csv.exists() or out_csv.stat().st_size == 0:
        pd.DataFrame(columns=WIDE_CSV_STRUCTURE).to_csv(out_csv, index=False)

    df = pd.read_csv(output_file)
    D = EuclideanDistance(df, filtering=True)
    DELTA_D = EuclideanDistanceUncertainty(df, filtering=True)

    total = len(df)
    if end is None or end > total:
        end = total
    if start < 0:
        start = 0
    if start >= end:
        print("Nothing to do: start >= end.")
        return

    updates: List[dict] = []
    for row in tqdm(range(start, end), desc="Euclidean Distance", unit="frame"):
        try:
            d = D.euclideanDistanceCalc(row)
            delta_d = DELTA_D.EuclideanDistUncerCalc(row)
            rec = {"frame": int(row)+1}
            rec.update({f"d_{i}": float(d[i - 48]) for i in range(48, 68)})
            rec.update({f"d_{ii}_unc": float(delta_d[ii - 48]) for ii in range(48, 68)})
            updates.append(rec)
        except Exception as e:
            print(f"Error on row {row}: {e}")


    if updates:
        wide = pd.read_csv(out_csv).set_index("frame")
        upd = pd.DataFrame(updates).set_index("frame")
        wide.update(upd)
        new_frames = upd.index.difference(wide.index)
        if len(new_frames) > 0 and not upd.loc[new_frames].empty:
            wide = pd.concat([wide, upd.loc[new_frames]], axis=0)
        wide = wide.reset_index()
        wide = wide.reindex(columns=WIDE_CSV_STRUCTURE + [c for c in wide.columns if c not in WIDE_CSV_STRUCTURE])
        wide.sort_values("frame", inplace=True)
        wide.to_csv(out_csv, index=False)

    print(f"Euclidean distances → {out_csv.resolve()}")
