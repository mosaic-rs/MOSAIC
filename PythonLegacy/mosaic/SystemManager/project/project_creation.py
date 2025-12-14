"""
This file is to create projects in OAS. Ideally, a project will be able to have multiple participants and within those, multiple trials.
This should make stats wayyy easier.
"""

import os
import uuid
from datetime import datetime
import json

class ProjectCreation:
    def __init__(self):
        self.myuuid = uuid.uuid4()

    def _creating_project_directory(self, out_dir: str, name: str = None):
        if name is None:
            raise ValueError(f"{name} can not be an empty string.")
        else:
            project_name = name
        try:
            os.mkdir(f"{out_dir}/{project_name}")

        except OSError as e:
            if os.path.exists(f"{out_dir}/{project_name}"):
                print(
                    f"Error creating project directory -- ~/{out_dir}/{project_name} -- Directory already exists in -- ~/{out_dir}")
            else:
                print("Unknown error in creating project directory."
                      f"{e}")

        return f"{out_dir}/{name}"

    def _creating_project_metadata(self, name, path):
        metadata = {
            "Name": name,
            "UUID": str(self.myuuid),
            "CreatedAt": str(datetime.now()),
            "OASVersion": "v1.0.0",
            "Description": ""
        }
        if os.path.exists(path):
            try:
                with open(f"{path}/{name}.mosaicproj", "w")as f:
                    json.dump(metadata, f, indent=4)

                return f"{path}/{name}.mosaicproj"

            except FileNotFoundError as e:
                print(f"Error creating project metadata -- ~/{path} -- can not be found or does not exist.")

    def _creating_state_system(self, name, path):
        state = {
            "projectRoot": f"{path}",
            "participant": None,
            "trial": None,
        }
        if os.path.exists(path):
            try:
                with open(f"{path}/state.mosaicproj", "w")as f:
                    json.dump(state, f, indent=4)

                return f"{path}/state.mosaicproj"

            except FileNotFoundError as e:
                print(f"Error creating project metadata -- ~/{path} -- can not be found or does not exist.")

    def _make_participant_dir(self, path: str)-> str:
        if os.path.exists(path + "/participants/"):
            pass
        elif not os.path.exists(path + "/participants/"):
            os.mkdir(path + "/participants")

    def project_creation(self, out_dir: str, name: str):
        project_path = self._creating_project_directory(out_dir, name)
        # Now make project data for directory
        self._creating_project_metadata(name, project_path)
        # Now making state.oasproj so we can make command usage locative
        self._creating_state_system(name, project_path)
        # we can also make the participants folder after
        self._make_participant_dir(project_path)
