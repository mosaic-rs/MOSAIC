"""
This returns the list of directories that exist in <project folder/participants/>

That's it
"""
import os.path
from os import DirEntry


class ParticipantList:
    def __init__(self):
        pass

    @staticmethod
    def get_participants( path) -> None | list[DirEntry[str]] | str:
        participants = []
        participant_path = os.path.join(path, "participants")

        try:
            with os.scandir(participant_path) as d:
                for e in d:
                    if e.is_dir():
                        participants.append(e)

            if not participants:
                return "No participants in this project"

            else:
                return participants

        except PermissionError:
            print(f"Permission error: can not access {participant_path}")
        except FileNotFoundError:
            print(f"Directory error: can not find {participant_path}")


