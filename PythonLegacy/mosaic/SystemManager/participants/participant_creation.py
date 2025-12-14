"""
This file is to create participant folders
"""

import os
import json

class ParticipantCreation:
    def __init__(self):
        pass

    def creating_participant_directory(self, project_dir: str, ID: str = None):
        # I am gonna make it so they can submit a string of participants and then make a bunch of participant folders that way

        participants = [i.strip() for i in ID.split(",")]

        participant_directories = []

        for participant_id in participants:

            participant_dir = os.path.join(project_dir, "participants", participant_id)
            try:
                meta = {
                    "ID": participant_id,
                }
                os.mkdir(participant_dir)
                with open(os.path.join(participant_dir, "meta.mosaicproj"), "w")as f:
                    json.dump(meta, f, indent=4)

                participant_directories.append(participant_dir)

            except RuntimeError as e:
                if os.path.exists(participant_dir):
                    print(
                        f"Participant {participant_id} already exists in this project")
                else:
                    print("Unknown error in creating participant directory."
                          f"{e}")

        return participant_directories
