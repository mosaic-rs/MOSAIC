"""
Calculates the velocity of the mouth area and all 4 quadrants
"""
import pandas as pd
from pathlib import Path
from mosaic.config import OUTER_QUAD_AREA, INNER_QUAD_AREA, OUTER_BIO_AREA, INNER_BIO_AREA


class AreaVelocity:
    def __init__(self, src: str | Path | pd.DataFrame):
        if isinstance(src, str):
            try:
                self.landmarks = pd.read_csv(src)
                return
            except TypeError as e:
                raise f"Invalid File Format: File must be a csv format\n{e}"

        elif isinstance(src, Path):
            try:
                self.landmarks = pd.read_csv(src)
                return
            except TypeError as e:
                raise f"Invalid File Format: File must be a csv format\n{e}"

        elif isinstance(src, pd.DataFrame):
            self.landmarks = src

        else:
            raise TypeError("Invalid File Format: File must be a csv format and passed in the following ways:\nString, Path, or pd.Dataframe")

    @staticmethod
    def _getting_data(self, row, columns)-> dict | TypeError:
        if isinstance(row, int):
            pass
        elif isinstance(row, str):
            row = int(row)
        else:
            return TypeError(f"row must be an int or str - not {type(row)}")

        df = self.landmarks.set_index("frame")
        df_row = df.loc[row]

        data = {}
        for column in columns:
            area = df_row[column]
            data.update({column: area})

        return data

    def area_velocity(self, row: int | str, columns) -> dict | TypeError:
        if isinstance(row, int):
            pass
        elif isinstance(row, str):
            row = int(row)
        else:
            return TypeError(f"row must be an int or str - not {type(row)}")

        current_row = self._getting_data(self.landmarks, row)
        if row == 1:
            return 0
        else:
            previous_row = self._getting_data(self.landmarks, row - 1)

        velocity = {}

        for i in columns:
            current_area = current_row[i]

            previous_area = previous_row[i]

            area_velocity = current_area - previous_area

            velocity.update({i: area_velocity})





