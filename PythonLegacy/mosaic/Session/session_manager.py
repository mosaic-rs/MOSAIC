"""
This is the session manager for the CLI so it can keep track of where the user is

it stores global data, like the directory the person is in so it sort of acts like a session system where they can
access "test-project/Project 40" for example, and then use commands within there.
"""


import os
import json
from typing import Optional, Dict, Any


HOME_DIR = os.path.expanduser("~")

# where mosaic is installed on the computer n stuff
SESSION_DIR = os.path.join(HOME_DIR, ".mosaic")

# This basically makes the global session.json folder in ~/.mosaic
SESSION_FILE = os.path.join(SESSION_DIR, "session.json")

class SessionManagement:
    """This essentially reads, writes, and updates the global session json file """

    def __init__(self, path: str | None = None) -> None:
        pass

    def _check_session_dir_exists(self) -> None:
        if not os.path.exists(SESSION_DIR):
            try:
                os.makedirs(SESSION_DIR, exist_ok=True)
            except OSError as e:
                raise RuntimeError(
                    f"Fatal Error: Unable to create MOSAIC session directory: {SESSION_DIR}\n{e}"
                )


    def _check_session_file_exists(self) -> Dict[str, Any]:
        # basically checking if session.json exists and if not, creating one with an empty session
        self._check_session_dir_exists()

        if not os.path.exists(SESSION_FILE):
            # Create  empty session
            default_session = {
                "currentProject": None,
                "currentParticipant": None,
                "currentTrial": None,
            }
            try:
                with open(SESSION_FILE, "w") as f:
                    json.dump(default_session, f, indent=4)
            except OSError as e:
                raise RuntimeError(
                    f"Fatal Error: Unable to create global session file: {SESSION_FILE}\n{e}"
                )

            return default_session

    def _read_session(self) -> Dict[str, Any]:
        # just reads the session data and returns it and if the session file is missing or corrupt, it'll try to make a new one
        self._check_session_dir_exists()

        if not os.path.exists(SESSION_FILE):
            # If missing we just create and return a fresh one
            return self._check_session_file_exists()

        try:
            with open(SESSION_FILE, "r") as f:
                session_data = json.load(f)
        except (OSError, json.JSONDecodeError) as e:
            # file is corrupt or just cant be read for some reason - I will probably try to make this recreate the session file in the future
            raise RuntimeError(
                f"Fatal Error: Global session file exists but cannot be read or parsed: {SESSION_FILE}\n{e}"
            )

        # Now we can just check everythign exists

        for key in ("currentProject", "currentParticipant", "currentTrial"):
            session_data.setdefault(key, None)

        return session_data

    def _write_session(self, sessionData: Dict[str, Any]) -> None:
        # Write the new info to the file !!
        self._check_session_dir_exists()

        try:
            with open(SESSION_FILE, "w") as f:
                json.dump(sessionData, f, indent=4)
        except OSError as e:
            raise RuntimeError(
                f"Fatal Error: Unable to write global session file: {SESSION_FILE}\n{e}"
            )

    """
    The next stuff is higher level sort of stuff like changing what project the user is in, what participant
    they are looking at, etc but it is kinda the whole point of this thing
    """

    def clear_session(self) -> Dict[str, Any]:
        # resets the session file to a fresh one - kinda like logging out in a way
        # it is used for when exiting a project
        session = {
            "currentProject": None,
            "currentParticipant": None,
            "currentTrial": None,
        }
        self._write_session(session)
        print("SESSION DATA UPDATED")
        return session

    def set_current_project(self, projectPath: str) -> Dict[str, Any]:
        """
        Setting the currentProject section of the session file to the desired project

        This is used after the project manager verifies that the path exists so the session manager doesnt have to deal
        with that logic

        We also have to verify the folder they opened is an actual OAS project by checking for the <project name>.oasproj file
        """

        projectPath = os.path.abspath(projectPath)

        if os.path.exists(os.path.join(projectPath, (os.path.basename(projectPath)+".mosaicproj"))):

            session = self._read_session()
            session["currentProject"] = projectPath
            session["currentParticipant"] = None # these reset because you open a new project so it is different data
            session["currentTrial"] = None

            self._write_session(session)
            print("SESSION DATA UPDATED")
            return session

        else:
            raise FileNotFoundError(f"Not a valid MOSAIC Project Folder. Can not find {os.path.join(projectPath, (os.path.basename(projectPath)+".mosaicproj"))}")

    def set_current_participant(self, participantId: str) -> Dict[str, Any]:
        """
        Sets the desired participant in the global session

        The user has to be "logged in"/in a project to do this
        """

        session = self._read_session()
        participant_global_dir = os.path.join(session.get("currentProject"), "participants")

        if not participant_global_dir:
            raise RuntimeError(
                "No project is currently open.\n"
                "Open a project first using:\n"
                "    mosaic project open <project-path>"
            )

        elif not os.path.join(participant_global_dir, participantId):
            raise RuntimeError(
                f"Participant <{participantId}> not in <{participant_global_dir}>"
            )

        session["currentParticipant"] = os.path.join(participant_global_dir, participantId)
        session["currentTrial"] = None  # Resets cuz new participant means new data
        self._write_session(session)
        print("SESSION DATA UPDATED")
        return session

    def set_current_trial(self, trialName: str) -> Dict[str, Any]:
        """
        Like the last two, this just selects the trial within the participant dir
        """
        session = self._read_session()

        if not session.get("currentProject"):
            raise RuntimeError(
                "No project is currently open.\n"
                "Open a project first using:\n"
                "    mosaic project open <project-path>"
            )

        if not session.get("currentParticipant"):
            raise RuntimeError(
                "No participant is currently selected.\n"
                "Select one using:\n"
                "    mosaic participant open <participant-id>"
            )

        # I will add the same error handling here as before to check if trial exists

        session["currentTrial"] = os.path.join(session["currentParticipant"], trialName)
        self._write_session(session)
        print("SESSION DATA UPDATED")
        return session

    """
    Now it is all validation stuff to make sure stuff works properly
    """

    def require_project(self) -> str:
        """
        Return the current project path, or raise if none is set. This should be used at the start of any command that requires a project.
        """
        session = self._read_session()
        current = session.get("currentProject")
        if not current:
            raise RuntimeError(
                "No project is currently open.\n"
                "Open a project with:\n"
                "    mosaic project open <project-path>"
            )
        return current

    def require_participant(self) -> str:
        """
        Same as before but for participant. Should be used for any command that requires a participant.
        """
        session = self._read_session()
        current_project = session.get("currentProject")
        current_participant = session.get("currentParticipant")

        if not current_project:
            raise RuntimeError(
                "No project is currently open.\n"
                "Open a project with:\n"
                "    mosaic project open <project-path>"
            )
        if not current_participant:
            raise RuntimeError(
                "No participant is currently selected.\n"
                "Select one with:\n"
                "    mosaic participant open <participant-id>"
            )

        return current_participant

    def require_trial(self) -> str:
        """
        Should be used when using a command for trials
        """
        session = self._read_session()
        current_project = session.get("currentProject")
        current_participant = session.get("currentParticipant")
        current_trial = session.get("currentTrial")

        if not current_project:
            raise RuntimeError(
                "No project is currently open.\n"
                "Open a project with:\n"
                "    mosaic project open <project-path>"
            )
        if not current_participant:
            raise RuntimeError(
                "No participant is currently selected.\n"
                "Select one with:\n"
                "    mosaic participant open <participant-id>"
            )
        if not current_trial:
            raise RuntimeError(
                "No trial is currently selected.\n"
                "Select one with:\n"
                "    mosaic trial open <trial-name>"
            )

        return current_trial

    def session_summary(self) -> str:
        """
        Returns a pretty summary of the global session data
            "Project: /path/to/project | Participant: 001 | Trial: baseline"
        """
        session = self._read_session()
        return (
            f"Project: {session.get('currentProject')!r} | "
            f"Participant: {session.get('currentParticipant')!r} | "
            f"Trial: {session.get('currentTrial')!r}"
        )

    def get_prompt(self):
        data = self._read_session()

        proj = data.get("currentProject")
        part = data.get("currentParticipant")
        trial = data.get("currentTrial")

        if proj is None:
            return "[MOSAIC] (no project)"

        # Build context
        ctx = f"[MOSAIC] {os.path.basename(os.path.normpath(proj))}"
        if part:
            ctx += f" :: participant: {os.path.basename(os.path.normpath(part))}"
        if trial:
            ctx += f" :: trial: {os.path.basename(os.path.normpath(trial))}"

        return ctx