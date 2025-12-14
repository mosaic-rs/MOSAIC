"""
This is to view participant folder contents like how many trials there are, etc.
"""

import os

class ParticipantViewer:
    def __init__(self):
        pass

    @staticmethod
    def participant_viewer(path):
        print("RAW PATH:", repr(path))
        print("EXISTS:", os.path.exists(path))
        print("ISDIR:", os.path.isdir(path))
        if not os.path.exists(path):
            raise FileNotFoundError(f"Cannot find <{path}>\nMake sure this participant file exists.")

        try:
            data = os.listdir(path)
            print(data)
        except Exception as e:
            raise RuntimeError(f"Error fetching participant data: {e}") from e