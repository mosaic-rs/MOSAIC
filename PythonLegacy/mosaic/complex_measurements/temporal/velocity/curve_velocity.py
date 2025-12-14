"""
Takes the velocity of the change in curvature using the curve points and coeffs


WHAT WE CAN DO:

To see how much the curve just moves overall, we can calculate the current curve and the curve before it and
calculate the area in between the two curves

Then, to calculate how much the curve changes, we can compare the slop of each curve and get the difference
"""
import os.path

"""
THE REASON THE FIRST GRADIENT OF A CURVE IS NAN IS BECAUSE WE ARE PASSING T = 0 WHICH MAKES Y'(T) AND X'(T) = 0 SO WE ARE 
DIVIDING BY 0
"""
from pathlib import Path
import pandas as pd

class CurveVelocity:
    def __init__(self, src: str|Path|pd.DataFrame, curve_config: list, row: int) -> None | str | FileNotFoundError | TypeError:
        if isinstance(src, str):
            if os.path.exists(src):
                try: self.src = pd.read_csv(src)
                except Exception as e:
                    f"{src} is not a valid CSV file or {e}\n"
            else:
                raise FileNotFoundError

        if isinstance(src, Path):
            if os.path.exists(src):
                try: self.src = pd.read_csv(src)
                except Exception as e:
                    f"{src} is not a valid CSV file or {e}\n"
            else:
                raise FileNotFoundError

        if isinstance(src, pd.DataFrame):
            try: self.src = src
            except Exception as e: f"Unknown error reading {src} - {e}\n"

        if isinstance(curve_config, list):
            try: self.curve_config = curve_config
            except Exception as e: f"Unknown error reading curve config data - {e}\n"
        else:
            raise TypeError

        if isinstance(row, int):
            try: self.row = row
            except Exception as e: f"Unknown error when getting row integer - {e}\n"
        else:
            raise TypeError

    def _getting_data(self):
        df = self.src.set_index("frame")
        df_row = df.loc[self.row] # gets all the data from the csv of that specific row

        return df_row

    def _cubic_coeffs(self):
        """
        This function gets the cubic coeffs from self.src using the self.curve_config list

        :return: Iterated list [[..., ...], ...]
        """
        row_data = self._getting_data() # we call on this function to get the data for the row we are on
        coefficient_data = []
        for curve in self.curve_config: # iterate through the curve config where curve = single list of column names
            curve_coefficient_list = []

            for coefficient in curve: # now we get each curve column name
                coeff = row_data[coefficient]

                curve_coefficient_list.append(float(coeff)) # we append that coefficient value to the list

            coefficient_data.append(curve_coefficient_list) # lastly we append the list of coefficients to a greater list replicating the config sublists we had before but with the actual values

        print(coefficient_data)
        return coefficient_data



