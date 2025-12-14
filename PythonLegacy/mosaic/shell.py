"""
I am making a shell so it looks more intuitive so when you enter a participant folder for example, you can see that in the prompt
"""

from mosaic.Session.session_manager import SessionManagement
from mosaic.SystemManager.project.project_creation import ProjectCreation
from mosaic.SystemManager.system_verification import SystemVerifier
from mosaic.SystemManager.participants.participant_list import ParticipantList
from mosaic.SystemManager.participants.participant_creation import ParticipantCreation
from mosaic.SystemManager.participants.participant_view import ParticipantViewer
from mosaic.SystemManager.trial.trial_run import TrialRun
import os
import shlex

class OASShell:
    def __init__(self, session_manager=None):
        from mosaic.Session.session_manager import SessionManagement
        self.session = session_manager or SessionManagement()

    def start(self):
        while True:
            prompt = self.session.get_prompt()
            user_input = input(f"{prompt} > ").strip()

            if user_input in ("exit", "quit"):
                break

            self.handle_command(user_input)

    def handle_command(self, user_input: str):
        parts = shlex.split(user_input)

        if not parts:
            return

        cmd = parts[0]

        if cmd == "open":
            try:
                subcmd = parts[1]

                if subcmd == "project":
                    # first we gotta verify it exists:
                    if len(parts) < 3:
                        print("[MOSAIC ERROR] Missing Arguments. Ensure you specific a path\nuse <open project 'path'>")
                        return

                    _, args, path = parts[:3]

                    # first we gotta verify it exists:
                    try:
                        SystemVerifier.project_verifier(path)

                    except RuntimeError as e:
                        print(f"[MOSAIC ERROR] {e}")
                        return

                    # when we know it exists, we then update the sessions data

                    self.session.set_current_project(path)
                    print(f"\nOpened project at: {path}\n")
                if subcmd == "participant":
                    if len(parts) < 3:
                        print("[MOSAIC ERROR] Missing Arguments. Ensure you specific a participant ID\nuse <open participant 'ID'>")
                        return

                    _, args, ID = parts[:3]

                    # first we have to verify they are in a project
                    self.session.require_project()

                    # then we verify the participant folder exists
                    session_data = self.session._read_session()

                    try:
                        SystemVerifier.participant_verifier(session_data["currentProject"], ID)

                    except RuntimeError as e:
                        print(f"[MOSAIC ERROR] {e}")
                        return

                    # Once that is done we edit the Session manager to write the participant we are in
                    self.session.set_current_participant(ID)

                    print(f"\nParticipant file opened at at: {session_data['currentProject']}/participants/{ID}\n")
                if subcmd == "trial":
                    if len(parts) < 3:
                        print(
                            "[MOSAIC ERROR] Missing Arguments. Ensure you specific a trial ID\nuse <open trial 'ID'>")
                        return

                    _, args, ID = parts[:3]

                    # we gotta verify they are in a participant folder (checks for proj at same time)

                    self.session.require_participant()

                    # now we have to verify the trial folder exists
                    session_data = self.session._read_session()

                    try:
                        SystemVerifier.trial_verifier(session_data["currentParticipant"], ID)

                    except RuntimeError as e:
                        print(f"[MOSAIC ERROR] {e}")
                        return

                    # once that is done we can edit the session manager to write the trial folder we are in

                    self.session.set_current_trial(ID)
                    print(f"\nTrial file opened at at: {session_data['currentParticipant']}/{ID}\n")

            except IndexError:
                print("[MOSAIC ERROR] Missing arguments. Use: open <project> or <participant> or <trial>")
            except RuntimeError as e:
                print(f"[MOSAIC ERROR] {e}")
            except Exception as e:
                print(f"[MOSAIC INTERNAL ERROR] {e}")

        elif cmd == "add":
            try:
                subcmd = parts[1]

                if subcmd == "project":

                    if len(parts) < 4:
                        print("[MOSAIC ERROR] Usage: add project <path> <name>")
                        return

                    _, args, path, name = parts[:4]

                    print("path:", path)
                    print("name:", name)

                    # now we can try to actually make the project and put them in the project folder

                    ProjectCreation().project_creation(path, name)

                    # Nowe we put them in that project

                    #self.session.set_current_project(os.path.join(path, name))

                elif subcmd == "participant":

                    if len(parts) < 3:
                        print("[MOSAIC ERROR] Usage: add <participant ID>")
                        return

                    _, args, ID = parts[:3]

                    # first we have to verify they are in a project
                    self.session.require_project()

                    # we need to get proj directory so we can make the participant folder
                    session_data = self.session._read_session()
                    directory = session_data["currentProject"]
                    # now we create the participant folder
                    from mosaic.SystemManager.participants.participant_creation import ParticipantCreation
                    try:
                        ParticipantCreation().creating_participant_directory(directory, ID)

                    except RuntimeError as e:
                        print(f"[MOSAIC ERROR] {e}")
                        return

                elif subcmd == "trial":
                    if len(parts) < 3:
                        print("[MOSAIC ERROR] Usage: add <trial ID>")
                        return

                    _, args, ID = parts[:3]

                    # first we have to verify they are in a participant file
                    self.session.require_participant()

                    # we need to get participant directory so we can make the trial folder
                    session_data = self.session._read_session()
                    directory = session_data["currentParticipant"]
                    # now we create the participant folder
                    from mosaic.SystemManager.trial.trial_creation import TrialCreation
                    try:
                        TrialCreation().creating_trial_directory(directory, ID)

                    except RuntimeError as e:
                        print(f"[MOSAIC ERROR] {e}")
                        return


                else:
                    print("[MOSAIC ERROR] Unknown add command")
                    return


            except IndexError:
                print("[MOSAIC ERROR] Missing arguments. Use: add <project> or <participant> or <trial>")
            except RuntimeError as e:
                print(f"[MOSAIC ERROR] {e}")
            except Exception as e:
                print(f"[MOSAIC INTERNAL ERROR] {e}")

        elif cmd == "delete":
            try:
                subcmd = parts[1]

            except IndexError:
                print("[MOSAIC ERROR] Missing arguments. Use: delete <project> or <participant> or <trial>")
            except RuntimeError as e:
                print(f"[MOSAIC ERROR] {e}")
            except Exception as e:
                print(f"[MOSAIC INTERNAL ERROR] {e}")

        elif cmd == "session":

            try:
                subcmd = parts[1]

                if subcmd == "reset":

                    self.session.clear_session()
                    print("Session reset. No project, participant, or trial is currently active.")
                    return

                elif subcmd == "c":

                    print(self.session.session_summary())

                else:
                    print("[MOSAIC ERROR] Unknown <session> command. Use <session help> for help")


            except IndexError:
                print("[MOSAIC ERROR] Missing arguments. Use: session <reset> or <c>\nor use <session help>")
            except RuntimeError as e:
                print(f"[MOSAIC ERROR] {e}")
            except Exception as e:
                print(f"[MOSAIC INTERNAL ERROR] {e}")

        elif cmd == "trial":

            try:
                subcmd = parts[1]

                if subcmd == "run":
                    # we have to verify they are in a tral folder

                    self.session.require_trial()

                    # we need to confirm the trial folder exists

                    trial_dir = self.session._read_session().get("currentTrial")

                    SystemVerifier.trial_verifier(trial_dir)

                    # once we confirm it exists we can call the trial_run function to run the trial

                    run = TrialRun

                    run._run_oas(trial_path=trial_dir)

                else:
                    print("[MOSAIC ERROR] Unknown <trial> command. Use <trial help> for help")


            except IndexError:
                print("[MOSAIC ERROR] Missing arguments. Use: session <reset> or <c>\nor use <session help>")
            except RuntimeError as e:
                print(f"[MOSAIC ERROR] {e}")
            except Exception as e:
                print(f"[MOSAIC INTERNAL ERROR] {e}")

        elif cmd in ("h", "help"):
            print("HELP")

        else:
            print("[MOSAIC ERROR] Unknown command. Type <h> or <help> for help")

